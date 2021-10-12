use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::{Response as AxumResponse, StatusCode},
    response::IntoResponse,
    Json,
};

use serde_json::json;

use crate::errors::Errors;

#[derive(Clone, Debug, Serialize)]
pub struct Response {
    pub xml: String,
}

impl Response {
    pub fn from_xml(xml: String) -> Self {
        Self { xml }
    }
}

impl IntoResponse for Errors {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> AxumResponse<Self::Body> {
        let status = match self {
            Errors::MissingAuthToken | Errors::InvalidAuthToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}
