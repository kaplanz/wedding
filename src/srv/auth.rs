use axum_login::memory_store::MemoryStore;
use axum_login::{extractors, RequireAuthorizationLayer};
use log::debug;

use crate::user::User;

pub type AuthContext = extractors::AuthContext<User, MemoryStore<User>>;
#[allow(unused)]
pub type RequireAuth = RequireAuthorizationLayer<User>;

pub async fn login(mut auth: AuthContext, user: User) {
    if auth.current_user.is_some() {
        logout(auth.clone()).await;
    }
    auth.login(&user).await.unwrap();
    debug!("login: `{user}`");
}

pub async fn logout(mut auth: AuthContext) {
    if let Some(user) = auth.current_user.as_ref() {
        debug!("logout: `{user}`");
    }
    auth.logout().await;
}
