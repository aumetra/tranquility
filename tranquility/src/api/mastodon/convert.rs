use {
    crate::{database::model::OAuthApplication, error::Error},
    tranquility_types::mastodon::App,
};

pub trait IntoMastodon {
    type ApiEntity;
    type Error;

    fn into_mastodon(self) -> Result<Self::ApiEntity, Self::Error>;
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
