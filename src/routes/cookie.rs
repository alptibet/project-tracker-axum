use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, CookieJar};

pub async fn deneme(mut jar: CookieJar) -> impl IntoResponse {
    jar.add(Cookie::new("secret", "secret-dasdfasdf"))
}
