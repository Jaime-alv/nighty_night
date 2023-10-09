use axum::response::IntoResponse;
use tokio::signal;

use crate::{
    configuration::constant::GlobalCte,
    response::error::ApiError,
    service::{
        session_service::{read_user_from_db, save_user_indefinitely, user_exists_in_session},
        util_service::not_found,
    },
};

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    tokio::select! {
        _ = ctrl_c=> {},

    }
}

pub async fn error_404() -> impl IntoResponse {
    not_found()
}

pub async fn set_anonymous_user() -> Result<(), ApiError> {
    let id: i64 = GlobalCte::DefaultAnonymousID.get().into();
    if user_exists_in_session(id).await? {
        return Ok(());
    } else {
        let anonymous_id: i32 = GlobalCte::DefaultAnonymousID.get().try_into().unwrap();
        let user = read_user_from_db(anonymous_id).await?;
        save_user_indefinitely(&user).await
    }
}
