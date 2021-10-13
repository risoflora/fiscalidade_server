use std::borrow::Cow;

use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    http::StatusCode,
    BoxError, Json as AxumJson,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, Default)]
pub struct Json<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, AxumJson<Value>);

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match AxumJson::<T>::from_request(request).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body): (_, Cow<'_, str>) = match rejection {
                    JsonRejection::InvalidJsonBody(_) => (
                        StatusCode::BAD_REQUEST,
                        "Conteúdo de payload inválido".into(),
                    ),
                    JsonRejection::MissingJsonContentType(_) => (
                        StatusCode::BAD_REQUEST,
                        "Faltando 'Content-Type: application/json'".into(),
                    ),
                    error => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Erro desconhecido: {}", error).into(),
                    ),
                };
                Err((status, AxumJson(json!({ "erro": body }))))
            }
        }
    }
}
