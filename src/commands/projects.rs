use crate::api::NodanaClient;
use crate::cli::ProjectCommands;

pub async fn run(client: &NodanaClient, command: ProjectCommands) -> Result<(), anyhow::Error> {
    match command {
        ProjectCommands::List => list_projects(client).await,
        ProjectCommands::Get { id } => get_project(client, &id).await,
        ProjectCommands::Create { name } => create_project(client, &name).await,
        ProjectCommands::Update { id, name } => update_project(client, &id, &name).await,
        ProjectCommands::Delete { id } => delete_project(client, &id).await,
        ProjectCommands::Apps { id } => list_apps(client, &id).await,
    }
}

pub async fn list_projects(client: &NodanaClient) -> Result<(), anyhow::Error> {
    let projects = client.list_projects().await?;

    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    println!("{:<40} {:<30}", "ID", "NAME");
    println!("{}", "-".repeat(70));

    for project in projects {
        println!("{:<40} {:<30}", project.id, project.name);
    }

    Ok(())
}

pub async fn get_project(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let project = client.get_project(id).await?;

    println!("ID:         {}", project.id);
    println!("Name:       {}", project.name);
    if let Some(created_at) = &project.created_at {
        println!("Created:    {}", created_at);
    }
    if let Some(updated_at) = &project.updated_at {
        println!("Updated:    {}", updated_at);
    }

    Ok(())
}

pub async fn create_project(client: &NodanaClient, name: &str) -> Result<(), anyhow::Error> {
    let project = client.create_project(name).await?;
    println!("Project created: {} ({})", project.name, project.id);
    Ok(())
}

pub async fn update_project(
    client: &NodanaClient,
    id: &str,
    name: &str,
) -> Result<(), anyhow::Error> {
    let result = client.update_project(id, name).await?;
    println!("Project {} updated: {}", result.id, result.updated);
    Ok(())
}

pub async fn delete_project(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let result = client.delete_project(id).await?;
    println!("Project {} deleted: {}", result.id, result.deleted);
    Ok(())
}

pub async fn list_apps(client: &NodanaClient, id: &str) -> Result<(), anyhow::Error> {
    let apps = client.list_project_apps(id).await?;

    if apps.is_empty() {
        println!("No apps found in this project.");
        return Ok(());
    }

    println!("{:<40} {:<20} {:<12} {}", "ID", "TEMPLATE", "STATUS", "URL");
    println!("{}", "-".repeat(100));

    for app in apps {
        println!(
            "{:<40} {:<20} {:<12} {}",
            app.id,
            app.template.as_deref().unwrap_or("-"),
            app.status.as_deref().unwrap_or("-"),
            app.url.as_deref().unwrap_or("-"),
        );
    }

    Ok(())
}
