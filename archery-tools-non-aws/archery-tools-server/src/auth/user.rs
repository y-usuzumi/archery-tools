use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserId = Uuid;

use crate::util::rand_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    pub username: String,
    pub access_token: String,
}

impl User {
    pub fn rand_user() -> Self {
        let id = Uuid::new_v4();
        let username = rand_string(7);
        let access_token = rand_string(32);

        User {
            id,
            username,
            access_token,
        }
    }
}

#[derive(Clone)]
pub struct Credentials {
    user_id: UserId,
}

impl AuthUser for User {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.access_token.as_bytes()
    }
}
