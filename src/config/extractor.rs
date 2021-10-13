use std::result;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
};

use crate::{
    config::{models::deployment::DeploymentConfiguration, Deployments},
    errors::Errors,
};

#[async_trait]
impl<B> FromRequest<B> for DeploymentConfiguration
where
    B: Send,
{
    type Rejection = Errors;

    async fn from_request(request: &mut RequestParts<B>) -> result::Result<Self, Self::Rejection> {
        let Extension(deployments) = Extension::<Deployments>::from_request(request)
            .await
            .map_err(|error| Errors::from(error))?;
        let token = request
            .headers()
            .ok_or(Errors::HeadersTakenByAnotherExtractor)?
            .get("X-Auth-Token")
            .ok_or(Errors::MissingAuthToken)?
            .to_str()
            .map_err(|_| Errors::InvalidAuthToken)?;
        let deployment = deployments
            .get(token)
            .ok_or(Errors::DeploymentNotFound(token.to_string()))?;
        Ok(deployment.clone())
    }
}
