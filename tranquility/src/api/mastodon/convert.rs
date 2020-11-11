use {
    crate::{
        database::model::{Actor as DBActor, OAuthApplication},
        error::Error,
    },
    serde::Serialize,
    tranquility_types::{
        activitypub::Actor,
        mastodon::{Account, App},
    },
    url::Url,
    warp::Rejection,
};

pub trait IntoMastodon {
    type ApiEntity: Serialize;
    type Error: Into<Rejection>;

    fn into_mastodon(self) -> Result<Self::ApiEntity, Self::Error>;
}

impl IntoMastodon for DBActor {
    type ApiEntity = Account;
    type Error = Error;

    fn into_mastodon(self) -> Result<Self::ApiEntity, Self::Error> {
        let actor: Actor = serde_json::from_value(self.actor)?;

        let id = self.id.to_simple().to_string();
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

        Ok(Account {
            id,
            username,
            acct,
            display_name,

            avatar_static: avatar.clone(),
            avatar,

            header_static: header.clone(),
            header,
            ..Account::default()
        })
    }
}

impl IntoMastodon for OAuthApplication {
    type ApiEntity = App;
    type Error = Error;

    fn into_mastodon(self) -> Result<Self::ApiEntity, Self::Error> {
        let id = self.id.to_simple().to_string();
        let client_id = self.client_id.to_simple().to_string();
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
