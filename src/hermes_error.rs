use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HermesError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

// because most if not all HermesError will be turned into a Response
impl IntoResponse for HermesError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            HermesError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            },
            // just in case HermesError gets populated with other errors (prob not)
            _ => (StatusCode::NOT_IMPLEMENTED, "not implemented yet lmao".to_string())
        };

        (status, Json(json!({"message" : message}))).into_response()
    }
}
