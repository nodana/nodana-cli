use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
    #[allow(unused)]
    #[serde(rename = "statusCode")]
    pub status: Option<u16>,
}

// --- Projects ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateProjectRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectUpdatedResponse {
    pub id: String,
    pub updated: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectDeletedResponse {
    pub id: String,
    pub deleted: bool,
}

// --- Apps ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub id: String,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub rate: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    pub template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppResponse {
    pub id: String,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    // Additional fields may be present depending on the template
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct AppUpdatedResponse {
    pub id: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppActionResponse {
    pub id: String,
}
