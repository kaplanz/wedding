use axum_login::extractors;
use axum_login::memory_store::MemoryStore;
use log::debug;

use crate::user::User;

pub type Context = extractors::AuthContext<User, MemoryStore<User>>;

pub async fn login(mut auth: Context, user: User) {
    if auth.current_user.is_some() {
        logout(auth.clone()).await;
    }
    auth.login(&user).await.unwrap();
    debug!("login: `{user}`");
}

pub async fn logout(mut auth: Context) {
    if let Some(user) = auth.current_user.as_ref() {
        debug!("logout: `{user}`");
    }
    auth.logout().await;
}
