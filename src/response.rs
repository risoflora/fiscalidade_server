use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::{header, HeaderValue, Response as AxumResponse, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;

use crate::{errors::Errors, json::Json};

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
        (status, Json(json!({ "erro": self.to_string() }))).into_response()
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> AxumResponse<Self::Body> {
        let bytes = match serde_json::to_vec(&self.0) {
            Ok(response) => response,
            Err(error) => {
                return AxumResponse::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Full::from(json!({ "erro": error.to_string() }).to_string()))
                    .unwrap();
            }
        };
        let mut response = AxumResponse::new(Full::from(bytes));
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        response
    }
}
