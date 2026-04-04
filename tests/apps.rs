use mockito::{Mock, Server};
use nod::api::NodanaClient;

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

fn mock_post(server: &mut Server, path: &str, body: &str) -> Mock {
    server
        .mock("POST", path)
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create()
}

#[tokio::test]
async fn create_app_without_params() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/apps")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .match_header("content-type", "application/json")
        .match_body(r#"{"project_id":"prj-1","template":"phoenixd"}"#)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"data": {
            "id": "app-new",
            "projectId": "prj-1",
            "url": "https://app-new.nodana.app:8080"
        }}"#,
        )
        .create();

    let result = client(&server)
        .create_app("phoenixd", Some("prj-1"), None)
        .await
        .unwrap();

    mock.assert();
    assert_eq!(result.id, "app-new");
    assert_eq!(result.project_id.as_deref(), Some("prj-1"));
    assert_eq!(
        result.url.as_deref(),
        Some("https://app-new.nodana.app:8080")
    );
}

#[tokio::test]
async fn create_app_with_params() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/apps")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .match_header("content-type", "application/json")
        .match_body(
            r#"{"project_id":"prj-1","template":"phoenixd","params":{"auto_liquidity":"2m"}}"#,
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"data": {
            "id": "app-new",
            "projectId": "prj-1",
            "url": "https://app-new.nodana.app:8080"
        }}"#,
        )
        .create();

    let params = serde_json::json!({"auto_liquidity": "2m"});
    let result = client(&server)
        .create_app("phoenixd", Some("prj-1"), Some(params))
        .await
        .unwrap();

    mock.assert();
    assert_eq!(result.id, "app-new");
}

#[tokio::test]
async fn create_app_without_project_id() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/apps")
        .match_header("authorization", "Basic OnRlc3Qta2V5")
        .match_header("content-type", "application/json")
        .match_body(r#"{"template":"lnbits-phoenixd"}"#)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"data": {
            "id": "app-new",
            "projectId": "prj-auto",
            "url": "https://app-new.nodana.app:8080"
        }}"#,
        )
        .create();

    let result = client(&server)
        .create_app("lnbits-phoenixd", None, None)
        .await
        .unwrap();

    mock.assert();
    assert_eq!(result.project_id.as_deref(), Some("prj-auto"));
}

#[tokio::test]
async fn get_app_returns_app() {
    let mut server = Server::new_async().await;
    let mock = mock_get(
        &mut server,
        "/apps/app-1",
        r#"{"data": {
            "id": "app-1",
            "projectId": "prj-1",
            "template": "phoenixd",
            "rate": 0.000233,
            "url": "https://phoenixd-ad4f.nodana.app:8000",
            "status": "available",
            "createdAt": "2025-05-24T08:07:10.509Z",
            "updatedAt": "2025-05-24T08:07:10.509Z"
        }}"#,
    );

    let app = client(&server).get_app("app-1").await.unwrap();

    mock.assert();
    assert_eq!(app.id, "app-1");
    assert_eq!(app.template.as_deref(), Some("phoenixd"));
    assert_eq!(app.status.as_deref(), Some("available"));
    assert_eq!(app.rate, Some(0.000233));
}

#[tokio::test]
async fn update_app_returns_version() {
    let mut server = Server::new_async().await;
    let mock = mock_post(
        &mut server,
        "/apps/app-1/update",
        r#"{"data": {
            "id": "app-1",
            "image": "nodana/phoenixd:0.7.2",
            "version": "0.7.2"
        }}"#,
    );

    let result = client(&server).update_app("app-1").await.unwrap();

    mock.assert();
    assert_eq!(result.id, "app-1");
    assert_eq!(result.version.as_deref(), Some("0.7.2"));
    assert_eq!(result.image.as_deref(), Some("nodana/phoenixd:0.7.2"));
}

#[tokio::test]
async fn start_app() {
    let mut server = Server::new_async().await;
    let mock = mock_post(
        &mut server,
        "/apps/app-1/start",
        r#"{"data": {"id": "app-1"}}"#,
    );

    let result = client(&server).start_app("app-1").await.unwrap();

    mock.assert();
    assert_eq!(result.id, "app-1");
}

#[tokio::test]
async fn stop_app() {
    let mut server = Server::new_async().await;
    let mock = mock_post(
        &mut server,
        "/apps/app-1/stop",
        r#"{"data": {"id": "app-1"}}"#,
    );

    let result = client(&server).stop_app("app-1").await.unwrap();

    mock.assert();
    assert_eq!(result.id, "app-1");
}

#[tokio::test]
async fn restart_app() {
    let mut server = Server::new_async().await;
    let mock = mock_post(
        &mut server,
        "/apps/app-1/restart",
        r#"{"data": {"id": "app-1"}}"#,
    );

    let result = client(&server).restart_app("app-1").await.unwrap();

    mock.assert();
    assert_eq!(result.id, "app-1");
}

#[tokio::test]
async fn delete_app() {
    let mut server = Server::new_async().await;
    let mock = mock_post(
        &mut server,
        "/apps/app-1/delete",
        r#"{"data": {"id": "app-1"}}"#,
    );

    let result = client(&server).delete_app("app-1").await.unwrap();

    mock.assert();
    assert_eq!(result.id, "app-1");
}

#[tokio::test]
async fn api_error_on_forbidden() {
    let mut server = Server::new_async().await;
    let mock = mock_get(
        &mut server,
        "/apps/app-1",
        r#"{"message": "Request is forbidden", "error": "ForbiddenError", "statusCode": 403}"#,
    );

    let result = client(&server).get_app("app-1").await;

    mock.assert();
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("ForbiddenError"));
}
