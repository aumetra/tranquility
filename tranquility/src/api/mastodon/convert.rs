use {
    crate::{
        database::{Actor as DbActor, OAuthApplication, Object as DbObject},
        error::Error,
        format_uuid,
        state::ArcState,
    },
    async_trait::async_trait,
    itertools::Itertools,
    serde::Serialize,
    tranquility_types::{
        activitypub::{Activity, Actor, Object},
        mastodon::{Account, App, Source, Status},
    },
    url::Url,
    warp::Rejection,
};

#[async_trait]
/// Trait for converting any object into an Mastodon API entity
pub trait IntoMastodon<ApiEntity>: Send + Sync
where
    ApiEntity: Serialize + 'static,
{
    /// Possible error that can occur
    type Error: Into<Rejection>;

    /// Convert the object into an Mastodon API entity
    async fn into_mastodon(self, state: &ArcState) -> Result<ApiEntity, Self::Error>;
}

#[async_trait]
impl IntoMastodon<Account> for DbActor {
    type Error = Error;

    async fn into_mastodon(self, _state: &ArcState) -> Result<Account, Self::Error> {
        let actor: Actor = serde_json::from_value(self.actor)?;

        let id = format_uuid!(self.id);
        let username = actor.username;
        let url = actor.id;

        let acct = if self.remote {
            let parsed_url = Url::parse(&url)?;

            format!(
                "{}@{}",
                username,
                parsed_url.host_str().ok_or(Error::MalformedUrl)?
            )
        } else {
            username.clone()
        };

        let display_name = actor.name;
        let avatar = actor
            .icon
            .map(|attachment| attachment.url)
            .unwrap_or_default();

        let header = actor
            .image
            .map(|attachment| attachment.url)
            .unwrap_or_default();

        let account = Account {
            id,
            username,
            acct,
            display_name,

            avatar_static: avatar.clone(),
            avatar,

            header_static: header.clone(),
            header,
            ..Account::default()
        };

        Ok(account)
    }
}

#[async_trait]
impl IntoMastodon<Source> for DbActor {
    type Error = Error;

    async fn into_mastodon(self, _state: &ArcState) -> Result<Source, Self::Error> {
        let actor: Actor = serde_json::from_value(self.actor)?;

        let source = Source {
            privacy: "public".into(),
            language: "en".into(),

            note: actor.summary,

            ..Source::default()
        };

        Ok(source)
    }
}

#[async_trait]
impl IntoMastodon<Status> for DbObject {
    type Error = Error;

    async fn into_mastodon(self, state: &ArcState) -> Result<Status, Self::Error> {
        let activity_or_object: Object = serde_json::from_value(self.data)?;

        activity_or_object.into_mastodon(state).await
    }
}

#[async_trait]
impl IntoMastodon<Vec<Account>> for Vec<DbObject> {
    type Error = Error;

    async fn into_mastodon(self, state: &ArcState) -> Result<Vec<Account>, Self::Error> {
        let db_to_url = |object: DbObject| {
            let activity: Activity = match serde_json::from_value(object.data) {
                Ok(activity) => activity,
                Err(err) => {
                    warn!("Couldn't deserialize activity: {}", err);
                    return None;
                }
            };

            activity.object.as_url().map(ToOwned::to_owned)
        };

        let fetch_account_fn = |url: String| async move {
            let account = DbActor::by_url(&state.db_pool, url.as_str()).await?;
            let account: Account = account.into_mastodon(state).await?;

            Ok::<_, Error>(account)
        };
        let account_futures = self.into_iter().filter_map(db_to_url).map(fetch_account_fn);

        // The `join_all` function has a complexity of O(n^2) because it polls every future whenever one is ready
        // This should be fine for this use-case though as not a lot of objects should get converted anyway
        let accounts = futures_util::future::join_all(account_futures)
            .await
            .into_iter()
            .try_collect()?;

        Ok(accounts)
    }
}

#[async_trait]
impl IntoMastodon<App> for OAuthApplication {
    type Error = Error;

    async fn into_mastodon(self, _state: &ArcState) -> Result<App, Self::Error> {
        let id = format_uuid!(self.id);
        let client_id = format_uuid!(self.client_id);
        let website = if self.website.is_empty() {
            None
        } else {
            Some(self.website)
        };

        let app = App {
            id,
            name: self.client_name,
            client_id,
            client_secret: self.client_secret,
            redirect_uri: self.redirect_uris,
            website,
            vapid_key: None,
        };

        Ok(app)
    }
}

#[async_trait]
impl IntoMastodon<Status> for Object {
    type Error = Error;

    async fn into_mastodon(self, state: &ArcState) -> Result<Status, Self::Error> {
        let db_object = DbObject::by_url(&state.db_pool, self.id.as_str()).await?;
        let (_actor, db_actor) =
            crate::activitypub::fetcher::fetch_actor(state, self.attributed_to.as_str()).await?;

        let id = format_uuid!(db_object.id);
        let application = super::DEFAULT_APPLICATION.clone();
        let account = db_actor.into_mastodon(state).await?;

        let status = Status {
            id,
            created_at: self.published,

            sensitive: self.sensitive,
            spoiler_text: self.summary,
            visibility: "public".into(),

            uri: self.id.clone(),
            url: self.id,

            content: self.content,

            application,
            account,

            ..Status::default()
        };

        Ok(status)
    }
}
