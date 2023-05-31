use anyhow::Ok;
use axum::async_trait;
use axum_session_auth::Authentication;

use crate::{
    mapping::rol_mapper::translate_roles,
    repository::user_repository::load_user_by_id,
    service::session_service::{load_user_session, save_user_session, user_session_exists},
};

use super::role_model::Rol;

#[derive(Clone, Debug)]
pub struct CurrentUser {
    id: i64,
    anonymous: bool,
    username: String,
    roles: Vec<Rol>,
    babies: Vec<i32>,
    active: bool,
}

impl CurrentUser {
    pub fn new(id: i64, anonymous: bool, username: String, roles: Vec<Rol>,babies: Vec<i32>, active: bool) -> Self {
        Self {
            id,
            anonymous,
            username,
            roles,
            babies,
            active,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Rol::Admin)
    }

    pub fn has_baby(&self, baby: i32) -> bool {
        self.babies.contains(&baby)
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

    pub fn babies(&self) -> Vec<i32> {
        self.babies.to_owned()
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
            babies: vec![],
            active: true,
        }
    }
}

#[async_trait]
impl Authentication<CurrentUser, i64, redis::Client> for CurrentUser {
    async fn load_user(
        user_id: i64,
        _pool: Option<&redis::Client>,
    ) -> Result<CurrentUser, anyhow::Error> {
        if user_session_exists(user_id).await {
            let user = load_user_session(user_id).await;
            return Ok(user);
        } else {
            let tmp_user = load_user_by_id(user_id.try_into().unwrap());
            if tmp_user.is_err() {
                return Err(anyhow::anyhow!("{:#?}", tmp_user.err()))
            }
            let current_user = tmp_user.unwrap();

            let roles: Vec<u8> = current_user.find_roles_id().into_iter().collect();
            let translate_roles: Vec<Rol> = translate_roles(&roles);

            let user_session = CurrentUser::new(
                user_id,
                translate_roles.contains(&Rol::Anonymous),
                current_user.username(),
                translate_roles,
                current_user.find_babies_id(),
                current_user.active(),
            );
            let tmp_response = save_user_session(&user_session, roles).await;
            if tmp_response.is_err() {
                let error = tmp_response.err().unwrap();
                return Err(anyhow::anyhow!("{error}"));
            }
            Ok(user_session)
        }
    }

    fn is_authenticated(&self) -> bool {
        self.roles.contains(&Rol::User)
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}
