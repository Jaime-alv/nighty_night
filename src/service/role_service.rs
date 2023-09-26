use crate::{
    model::role_model::Rol, repository::role_repository::select_role_from_role_name,
    response::error::ApiError,
};

pub async fn get_role_by_name_service(rol_name: &str) -> Result<Rol, ApiError> {
    let normalized_str = rol_name.to_lowercase();
    let role_model = select_role_from_role_name(&normalized_str)?;
    let rol: Rol = role_model.id().into();
    Ok(rol)
}

#[cfg(test)]
mod test_role_service {
    use std::path::Path;

    use super::*;

    fn load_env() {
        dotenvy::from_path(Path::new("./key/local.env")).unwrap();
    }

    #[tokio::test]
    async fn test_role_name() {
        load_env();
        assert_eq!(get_role_by_name_service("admin").await.unwrap(), Rol::Admin);
        assert_eq!(get_role_by_name_service("AdMin").await.unwrap(), Rol::Admin);
        assert!(get_role_by_name_service("invent").await.is_err());
    }
}
