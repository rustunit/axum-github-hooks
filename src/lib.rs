use axum::{
    async_trait,
    extract::{rejection::BytesRejection, FromRequest, Request},
    http::StatusCode,
    response::IntoResponse,
    RequestExt,
};
use bytes::Bytes;
use octocrab::models::webhook_events::WebhookEvent;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebhookExtractRejection {
    #[error("Invalid header")]
    InvalidHeader,

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::error::Error),

    #[error("Bytes error: {0}")]
    BytesError(#[from] BytesRejection),
}

impl IntoResponse for WebhookExtractRejection {
    fn into_response(self) -> axum::response::Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}

#[derive(Debug, Clone)]
pub struct GithubWebhook(pub WebhookEvent);

#[async_trait]
impl<S> FromRequest<S> for GithubWebhook
where
    S: Send + Sync,
{
    type Rejection = WebhookExtractRejection;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(event) = req.headers().get("X-GitHub-Event") else {
            return Err(WebhookExtractRejection::InvalidHeader);
        };

        let Ok(event) = event.to_str() else {
            return Err(WebhookExtractRejection::InvalidHeader);
        };

        let event = event.to_string();

        let b: Bytes = req.extract().await?;

        let hook = WebhookEvent::try_from_header_and_body(&event, &b)?;

        Ok(Self(hook))
    }
}
