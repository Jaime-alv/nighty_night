use axum::async_trait;
use axum_session_auth::Authentication;
use uuid::Uuid;

use crate::service::session_service::{load_user_session, read_user_from_db, save_user_session};

use super::role_model::Rol;

#[derive(Clone, Debug)]
pub struct CurrentUser {
    id: i64,
    anonymous: bool,
    username: String,
    roles: Vec<Rol>,
    active: bool,
    baby_unique_id: Vec<Uuid>,
}

impl CurrentUser {
    pub fn new(
        id: i64,
        anonymous: bool,
        username: String,
        roles: Vec<Rol>,
        active: bool,
        baby_id: Vec<Uuid>,
    ) -> Self {
        Self {
            id,
            anonymous,
            username,
            roles,
            active,
            baby_unique_id: baby_id,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Rol::Admin)
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn username(&self) -> String {
        self.username.to_string()
    }

    pub fn anonymous(&self) -> bool {
        self.anonymous
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn baby_id(&self) -> Vec<Uuid> {
        self.baby_unique_id.to_owned()
    }

    pub fn _roles(&self) -> Vec<Rol> {
        self.roles.to_owned()
    }

    pub fn roles_id(&self) -> Vec<u8> {
        self.roles
            .to_owned()
            .into_iter()
            .map(|rol| rol.into())
            .collect()
    }
}

impl Default for CurrentUser {
    fn default() -> Self {
        let anonymous: Vec<Rol> = vec![Rol::Anonymous];

        Self {
            id: 1,
            anonymous: true,
            username: "GUEST".to_string(),
            roles: anonymous,
            active: true,
            baby_unique_id: vec![],
        }
    }
}

#[async_trait]
impl Authentication<CurrentUser, i64, redis::Client> for CurrentUser {
    async fn load_user(
        user_id: i64,
        _pool: Option<&redis::Client>,
    ) -> Result<CurrentUser, anyhow::Error> {
        match load_user_session(user_id).await {
            Ok(u) => Ok(u),
            Err(_) => {
                let current_user = read_user_from_db(user_id.try_into().unwrap()).await?;
                save_user_session(&current_user, None).await?;
                Ok(current_user)
            }
        }
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}
