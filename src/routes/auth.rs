use crate::appstate::AppState;
use crate::models::response::MessageResponse;
use crate::controllers::auth;
use crate::controllers::users;
use crate::errors::AppError;
use crate::models::auth::UserInput;
use axum::extract::{Json, State};
use cookie::{Cookie, CookieJar};

pub async fn signup(
    State(state): State<AppState>,
    input: Json<UserInput>,
    cookies: &mut CookieJar
) -> Result<Json<MessageResponse>, AppError> {
    match users::insert_one(&state.db, input).await {
        Ok(_user_doc) => {
            match auth::match_auth(&state.db, &_user_doc.username).await {
                Ok(_auth_info) => {
                    if _auth_info.is_none() {
                        return Err(AppError::BadRequest);
                    }
                    let token = auth::create_send_token(&_auth_info.unwrap()._id);
                    cookies.add(token);
                }
                Err(_error) => (),
            }
            Ok(Json(MessageResponse {
                message: "success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::BadRequest),
    }
}
