use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    pub status: String,
    pub endpoint_url: String,
    pub failure_message: Option<String>,
    pub update_status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNodeResponse {
    pub node: Node,
    pub credentials: NodeCredentials,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCredentials {
    pub seed: String,
    pub password: String,
    pub restricted_password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeUpdateResponse {
    pub current_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorDetails,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetails {
    pub code: String,
    pub message: String,
}
