use crate::models::*;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::env;
use std::error::Error as StdError;
use std::fmt;

const DEFAULT_BASE_URL: &str = "https://api.nodana.io/v1";
const BASE_URL_ENV_VAR: &str = "NODANA_API_BASE_URL";

#[derive(Debug)]
pub enum ApiError {
    Transport(reqwest::Error),
    Http {
        status: StatusCode,
        message: String,
        error_code: Option<String>,
        body: Option<String>,
    },
    InvalidResponse {
        status: Option<StatusCode>,
        message: String,
        body: Option<String>,
        source: Option<serde_json::Error>,
    },
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => write!(f, "request failed: {error}"),
            Self::Http {
                status,
                message,
                error_code,
                ..
            } => {
                write!(f, "API request failed ({status})")?;
                if !message.is_empty() {
                    write!(f, ": {message}")?;
                }
                if let Some(error_code) = error_code {
                    write!(f, " [{error_code}]")?;
                }
                Ok(())
            }
            Self::InvalidResponse {
                status, message, ..
            } => {
                if let Some(status) = status {
                    write!(f, "invalid API response ({status}): {message}")
                } else {
                    write!(f, "invalid API response: {message}")
                }
            }
        }
    }
}

impl StdError for ApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::InvalidResponse {
                source: Some(error),
                ..
            } => Some(error),
            Self::Http { .. } | Self::InvalidResponse { source: None, .. } => None,
        }
    }
}

pub struct NodanaClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl NodanaClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: env::var(BASE_URL_ENV_VAR)
                .ok()
                .filter(|base_url| !base_url.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            api_key,
        }
    }

    #[allow(dead_code)]
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .basic_auth("", Some(&self.api_key))
            .send()
            .await
            .map_err(ApiError::Transport)?;

        Self::parse_response(response).await
    }

    async fn post<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.post(&url).basic_auth("", Some(&self.api_key));

        if let Some(body) = body {
            req = req.json(body);
        }

        let response = req.send().await.map_err(ApiError::Transport)?;

        Self::parse_response(response).await
    }

    async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, ApiError> {
        let status = response.status();
        let body = response.text().await.map_err(ApiError::Transport)?;
        let body = if body.trim().is_empty() {
            None
        } else {
            Some(body)
        };

        let parsed = match &body {
            Some(body) => serde_json::from_str::<ApiResponse<T>>(body).map_err(|error| {
                ApiError::InvalidResponse {
                    status: Some(status),
                    message: "failed to parse API response body".into(),
                    body: Some(body.clone()),
                    source: Some(error),
                }
            }),
            None => Err(ApiError::InvalidResponse {
                status: Some(status),
                message: "empty response body".into(),
                body: None,
                source: None,
            }),
        };

        match parsed {
            Ok(envelope) => Self::extract_data(status, envelope, body),
            Err(_) if !status.is_success() => Err(Self::http_error_from_raw(status, body)),
            Err(error) => Err(error),
        }
    }

    fn extract_data<T>(
        status: StatusCode,
        envelope: ApiResponse<T>,
        body: Option<String>,
    ) -> Result<T, ApiError> {
        let effective_status = envelope
            .status
            .and_then(|status| StatusCode::from_u16(status).ok())
            .unwrap_or(status);

        if !status.is_success() || envelope.error.is_some() {
            return Err(ApiError::Http {
                status: effective_status,
                message: envelope
                    .message
                    .unwrap_or_else(|| "request failed".to_string()),
                error_code: envelope.error,
                body,
            });
        }

        envelope.data.ok_or(ApiError::InvalidResponse {
            status: Some(effective_status),
            message: "response did not include a data field".into(),
            body,
            source: None,
        })
    }

    fn http_error_from_raw(status: StatusCode, body: Option<String>) -> ApiError {
        ApiError::Http {
            status,
            message: format!("server returned {}", status.as_u16()),
            error_code: None,
            body,
        }
    }

    // --- Projects ---

    pub async fn list_projects(&self) -> Result<Vec<Project>, ApiError> {
        self.get("/projects").await
    }

    pub async fn get_project(&self, id: &str) -> Result<Project, ApiError> {
        self.get(&format!("/projects/{}", id)).await
    }

    pub async fn create_project(&self, name: &str) -> Result<Project, ApiError> {
        let body = CreateProjectRequest {
            name: name.to_string(),
        };
        self.post::<_, Project>("/projects", Some(&body)).await
    }

    pub async fn update_project(
        &self,
        id: &str,
        name: &str,
    ) -> Result<ProjectUpdatedResponse, ApiError> {
        let body = UpdateProjectRequest {
            name: name.to_string(),
        };
        self.post(&format!("/projects/{}/update", id), Some(&body))
            .await
    }

    pub async fn delete_project(&self, id: &str) -> Result<ProjectDeletedResponse, ApiError> {
        self.post::<serde_json::Value, _>(&format!("/projects/{}/delete", id), None)
            .await
    }

    pub async fn list_project_apps(&self, id: &str) -> Result<Vec<App>, ApiError> {
        self.get(&format!("/projects/{}/apps", id)).await
    }

    // --- Apps ---

    pub async fn create_app(
        &self,
        template: &str,
        project_id: Option<&str>,
        params: Option<serde_json::Value>,
    ) -> Result<CreateAppResponse, ApiError> {
        let body = CreateAppRequest {
            project_id: project_id.map(|s| s.to_string()),
            template: template.to_string(),
            params,
        };
        self.post("/apps", Some(&body)).await
    }

    pub async fn get_app(&self, id: &str) -> Result<App, ApiError> {
        self.get(&format!("/apps/{}", id)).await
    }

    pub async fn update_app(&self, id: &str) -> Result<AppUpdatedResponse, ApiError> {
        self.post::<serde_json::Value, _>(&format!("/apps/{}/update", id), None)
            .await
    }

    pub async fn start_app(&self, id: &str) -> Result<AppActionResponse, ApiError> {
        self.post::<serde_json::Value, _>(&format!("/apps/{}/start", id), None)
            .await
    }

    pub async fn stop_app(&self, id: &str) -> Result<AppActionResponse, ApiError> {
        self.post::<serde_json::Value, _>(&format!("/apps/{}/stop", id), None)
            .await
    }

    pub async fn restart_app(&self, id: &str) -> Result<AppActionResponse, ApiError> {
        self.post::<serde_json::Value, _>(&format!("/apps/{}/restart", id), None)
            .await
    }

    pub async fn delete_app(&self, id: &str) -> Result<AppActionResponse, ApiError> {
        self.post::<serde_json::Value, _>(&format!("/apps/{}/delete", id), None)
            .await
    }
}
