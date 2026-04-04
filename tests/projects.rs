use mockito::{Mock, Server};
use nod::api::{ApiError, NodanaClient};
use reqwest::StatusCode;

fn client(server: &Server) -> NodanaClient {
    NodanaClient::with_base_url("test-key".into(), server.url())
}

fn mock_get(server: &mut Server, path: &str, body: &str) -> Mock {
    server
        .mock("GET", path)
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create()
}

#[tokio::test]
async fn list_projects_returns_projects() {
    let mut server = Server::new_async().await;
    let mock = mock_get(
        &mut server,
        "/projects",
        r#"{"data": [
            {"id": "prj-1", "name": "Project One"},
            {"id": "prj-2", "name": "Project Two"}
        ]}"#,
    );

    let projects = client(&server).list_projects().await.unwrap();

    mock.assert();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].id, "prj-1");
    assert_eq!(projects[0].name, "Project One");
    assert_eq!(projects[1].id, "prj-2");
    assert_eq!(projects[1].name, "Project Two");
}

#[tokio::test]
async fn list_projects_returns_empty() {
    let mut server = Server::new_async().await;
    let mock = mock_get(&mut server, "/projects", r#"{"data": []}"#);

    let projects = client(&server).list_projects().await.unwrap();

    mock.assert();
    assert!(projects.is_empty());
}

#[tokio::test]
async fn get_project_returns_project() {
    let mut server = Server::new_async().await;
    let mock = mock_get(
        &mut server,
        "/projects/prj-1",
        r#"{"data": {
            "id": "prj-1",
            "name": "My Project",
            "createdAt": "2025-05-01T07:46:12.368Z",
            "updatedAt": "2025-05-01T07:46:12.368Z"
        }}"#,
    );

    let project = client(&server).get_project("prj-1").await.unwrap();

    mock.assert();
    assert_eq!(project.id, "prj-1");
    assert_eq!(project.name, "My Project");
    assert_eq!(
        project.created_at.as_deref(),
        Some("2025-05-01T07:46:12.368Z")
    );
}

#[tokio::test]
async fn create_project_returns_project() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/projects")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .match_header("content-type", "application/json")
        .match_body(r#"{"name":"New Project"}"#)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data": {"id": "prj-new", "name": "New Project"}}"#)
        .create();

    let project = client(&server).create_project("New Project").await.unwrap();

    mock.assert();
    assert_eq!(project.id, "prj-new");
    assert_eq!(project.name, "New Project");
}

#[tokio::test]
async fn update_project_returns_result() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/projects/prj-1/update")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .match_header("content-type", "application/json")
        .match_body(r#"{"name":"Updated Name"}"#)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data": {"id": "prj-1", "updated": true}}"#)
        .create();

    let result = client(&server)
        .update_project("prj-1", "Updated Name")
        .await
        .unwrap();

    mock.assert();
    assert_eq!(result.id, "prj-1");
    assert!(result.updated);
}

#[tokio::test]
async fn delete_project_returns_result() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/projects/prj-1/delete")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data": {"id": "prj-1", "deleted": true}}"#)
        .create();

    let result = client(&server).delete_project("prj-1").await.unwrap();

    mock.assert();
    assert_eq!(result.id, "prj-1");
    assert!(result.deleted);
}

#[tokio::test]
async fn list_project_apps_returns_apps() {
    let mut server = Server::new_async().await;
    let mock = mock_get(
        &mut server,
        "/projects/prj-1/apps",
        r#"{"data": [{
            "id": "app-1",
            "projectId": "prj-1",
            "template": "phoenixd",
            "status": "available",
            "url": "https://app-1.nodana.app:8080"
        }]}"#,
    );

    let apps = client(&server).list_project_apps("prj-1").await.unwrap();

    mock.assert();
    assert_eq!(apps.len(), 1);
    assert_eq!(apps[0].id, "app-1");
    assert_eq!(apps[0].template.as_deref(), Some("phoenixd"));
    assert_eq!(apps[0].status.as_deref(), Some("available"));
}

#[tokio::test]
async fn api_error_returns_error() {
    let mut server = Server::new_async().await;
    let mock = mock_get(
        &mut server,
        "/projects/bad-id",
        r#"{"message": "Not found", "error": "NotFoundError", "statusCode": 404}"#,
    );

    let result = client(&server).get_project("bad-id").await;

    mock.assert();
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("NotFoundError"));
}

#[tokio::test]
async fn non_json_http_error_preserves_status_and_body() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/projects/bad-id")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .with_status(500)
        .with_header("content-type", "text/plain")
        .with_body("upstream exploded")
        .create();

    let result = client(&server).get_project("bad-id").await;

    mock.assert();
    let err = result.expect_err("request to fail");
    match err {
        ApiError::Http {
            status,
            message,
            error_code,
            body,
        } => {
            assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
            assert_eq!(message, "server returned 500");
            assert_eq!(error_code, None);
            assert_eq!(body.as_deref(), Some("upstream exploded"));
        }
        other => panic!("expected http error, got {other:?}"),
    }
}

#[tokio::test]
async fn malformed_success_response_returns_invalid_response() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/projects/prj-1")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{not-json")
        .create();

    let result = client(&server).get_project("prj-1").await;

    mock.assert();
    let err = result.expect_err("request to fail");
    match err {
        ApiError::InvalidResponse {
            status,
            message,
            body,
            ..
        } => {
            assert_eq!(status, Some(StatusCode::OK));
            assert_eq!(message, "failed to parse API response body");
            assert_eq!(body.as_deref(), Some("{not-json"));
        }
        other => panic!("expected invalid response error, got {other:?}"),
    }
}

#[tokio::test]
async fn success_without_data_returns_invalid_response() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/projects/prj-1")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message":"ok"}"#)
        .create();

    let result = client(&server).get_project("prj-1").await;

    mock.assert();
    let err = result.expect_err("request to fail");
    match err {
        ApiError::InvalidResponse {
            status,
            message,
            body,
            ..
        } => {
            assert_eq!(status, Some(StatusCode::OK));
            assert_eq!(message, "response did not include a data field");
            assert_eq!(body.as_deref(), Some(r#"{"message":"ok"}"#));
        }
        other => panic!("expected invalid response error, got {other:?}"),
    }
}
