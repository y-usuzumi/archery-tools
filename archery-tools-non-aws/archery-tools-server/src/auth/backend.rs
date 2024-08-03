use super::user::{Credentials, User, UserId};
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend};
use lazy_static::lazy_static;
use std::collections::HashMap;

fn uuid_user_pair(user: User) -> (UserId, User) {
    let uuid = user.id();
    (uuid, user)
}

lazy_static! {
    pub static ref BACKEND: Backend = {
        Backend::with_users(
            vec![
                User::rand_user(),
                User::rand_user(),
                User::rand_user(),
                User::rand_user(),
                User::rand_user(),
            ]
            .into_iter()
            .map(uuid_user_pair),
        )
    };
}

#[derive(Debug, Clone)]
pub struct Backend {
    users: HashMap<UserId, User>,
}

impl Backend {
    pub fn new() -> Self {
        Backend {
            users: HashMap::new(),
        }
    }

    pub fn with_users<I>(users: I) -> Self
    where
        I: IntoIterator<Item = (UserId, User)>,
    {
        let mut usermap = HashMap::new();
        for (uuid, user) in users.into_iter() {
            usermap.insert(uuid, user);
        }

        Backend { users: usermap }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    #[doc = " Authenticating user type."]
    type User = User;

    #[doc = " Credential type used for authentication."]
    type Credentials = Credentials;

    #[doc = " An error which can occur during authentication and authorization."]
    type Error = std::convert::Infallible;

    #[doc = " Authenticates the given credentials with the backend."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        todo!()
    }

    #[doc = " Gets the user by provided ID from the backend."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        todo!()
    }
}
