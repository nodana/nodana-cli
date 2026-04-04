use crate::api::NodanaClient;
use crate::cli::{AppCommands, AppCreateArgs};

impl AppCreateArgs {
    pub fn params(&self) -> Result<Option<serde_json::Value>, anyhow::Error> {
        let mut params = serde_json::Map::new();
        let mut args = self.params.iter();

        while let Some(arg) = args.next() {
            let arg = arg.strip_prefix("--").ok_or_else(|| {
                anyhow::anyhow!(
                    "invalid app create param '{}'; expected --<param> <value>",
                    arg
                )
            })?;

            if let Some((key, value)) = arg.split_once('=') {
                params.insert(key.replace('-', "_"), value.to_string().into());
                continue;
            }

            let value = args.next().ok_or_else(|| {
                anyhow::anyhow!(
                    "missing value for app create param '--{}'; expected --<param> <value>",
                    arg
                )
            })?;
            params.insert(arg.replace('-', "_"), value.clone().into());
        }

        if params.is_empty() {
            Ok(None)
        } else {
            Ok(Some(serde_json::Value::Object(params)))
        }
    }
}

pub async fn run(client: &NodanaClient, command: AppCommands) -> Result<(), anyhow::Error> {
    match command {
        AppCommands::Create(args) => {
            create_app(
                client,
                &args.template,
                args.project_id.as_deref(),
                args.params()?,
            )
            .await
        }
        AppCommands::Get { id } => get_app(client, &id).await,
        AppCommands::Update { id } => update_app(client, &id).await,
        AppCommands::Start { id } => start_app(client, &id).await,
        AppCommands::Stop { id } => stop_app(client, &id).await,
        AppCommands::Restart { id } => restart_app(client, &id).await,
        AppCommands::Delete { id } => delete_app(client, &id).await,
    }
}

pub async fn create_app(
    client: &NodanaClient,
    template: &str,
    project_id: Option<&str>,
    params: Option<serde_json::Value>,
) -> Result<(), anyhow::Error> {
    let result = client.create_app(template, project_id, params).await?;

    println!("ID:          {}", result.id);
    if let Some(project_id) = &result.project_id {
        println!("Project:     {}", project_id);
    }
    if let Some(url) = &result.url {
        println!("URL:         {}", url);
    }

    // Print any extra fields from the response (seeds, passwords, etc.)
    if let serde_json::Value::Object(map) = &result.extra {
        for (key, value) in map {
            if key == "id" || key == "projectId" || key == "url" {
                continue;
            }
            match value {
                serde_json::Value::String(s) => println!("{:<13}{}", format!("{}:", key), s),
                other => println!("{:<13}{}", format!("{}:", key), other),
            }
        }
    }

    Ok(())
}

pub async fn get_app(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let app = client.get_app(id).await?;

    println!("ID:         {}", app.id);
    if let Some(v) = &app.project_id {
        println!("Project:    {}", v);
    }
    if let Some(v) = &app.template {
        println!("Template:   {}", v);
    }
    if let Some(v) = &app.status {
        println!("Status:     {}", v);
    }
    if let Some(v) = &app.url {
        println!("URL:        {}", v);
    }
    if let Some(v) = &app.rate {
        println!("Rate:       {}", v);
    }
    if let Some(v) = &app.created_at {
        println!("Created:    {}", v);
    }
    if let Some(v) = &app.updated_at {
        println!("Updated:    {}", v);
    }

    Ok(())
}

pub async fn update_app(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let result = client.update_app(id).await?;
    println!("App {} updated", result.id);
    if let Some(version) = &result.version {
        println!("Version:    {}", version);
    }
    if let Some(image) = &result.image {
        println!("Image:      {}", image);
    }
    Ok(())
}

pub async fn start_app(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let result = client.start_app(id).await?;
    println!("App {} started", result.id);
    Ok(())
}

pub async fn stop_app(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let result = client.stop_app(id).await?;
    println!("App {} stopped", result.id);
    Ok(())
}

pub async fn restart_app(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let result = client.restart_app(id).await?;
    println!("App {} restarted", result.id);
    Ok(())
}

pub async fn delete_app(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let result = client.delete_app(id).await?;
    println!("App {} deleted", result.id);
    Ok(())
}
