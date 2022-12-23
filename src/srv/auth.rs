use axum_login::memory_store::MemoryStore;
use axum_login::{extractors, RequireAuthorizationLayer};
use log::trace;

use crate::user::User;

pub type AuthContext = extractors::AuthContext<User, MemoryStore<User>>;
pub type RequireAuth = RequireAuthorizationLayer<User>;

pub async fn login(mut auth: AuthContext, user: User) {
    if auth.current_user.is_some() {
        logout(auth.clone()).await;
    }
    auth.login(&user).await.unwrap();
    trace!("login: `{user}`");
}

pub async fn logout(mut auth: AuthContext) {
    if let Some(user) = auth.current_user.as_ref() {
        trace!("logout: `{user}`");
    }
    auth.logout().await;
}
