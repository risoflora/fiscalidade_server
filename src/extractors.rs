use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
};

use crate::{
    config::{models::deployment::DeploymentConfiguration, Deployments},
    response::ResponseError,
};

#[async_trait]
impl<B> FromRequest<B> for DeploymentConfiguration
where
    B: Send,
{
    type Rejection = ResponseError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(deployments) = Extension::<Deployments>::from_request(request)
            .await
            .map_err(|error| ResponseError::from(error))?;
        let token = request
            .headers()
            .ok_or(ResponseError::HeadersTakenByAnotherExtractor)?
            .get("X-Auth-Token")
            .ok_or(ResponseError::MissingToken)?
            .to_str()
            .map_err(|_| ResponseError::InvalidToken)?;
        let deployment = deployments
            .get(token)
            .ok_or(ResponseError::DeploymentNotFound(token.to_string()))?;
        Ok(deployment.clone())
    }
}
