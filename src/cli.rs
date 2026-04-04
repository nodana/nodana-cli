use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "nod")]
#[command(about = "Nodana CLI tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage projects
    Project(ProjectArgs),
    /// Manage apps
    App(AppArgs),
}

#[derive(Debug, Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectCommands,
}

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    /// List all projects
    List,
    /// Get a project by ID
    Get {
        /// Project ID
        id: String,
    },
    /// Create a new project
    Create {
        /// Project name
        #[arg(long)]
        name: String,
    },
    /// Update a project's name
    Update {
        /// Project ID
        id: String,
        /// New project name
        #[arg(long)]
        name: String,
    },
    /// Delete a project (must be empty)
    Delete {
        /// Project ID
        id: String,
    },
    /// List apps in a project
    Apps {
        /// Project ID
        id: String,
    },
}

#[derive(Debug, Args)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Debug, Subcommand)]
pub enum AppCommands {
    /// Create a new app
    #[command(after_help = "\
Examples:
  nod app create <template> --project-id <id> --<param> <value>
  nod app create phoenixd --auto-liquidity 2m
  nod app create cdk-mintd --mint-name \"My Mint\"
")]
    Create(AppCreateArgs),
    /// Get an app by ID
    Get {
        /// App ID
        id: String,
    },
    /// Update an app to the latest version
    Update {
        /// App ID
        id: String,
    },
    /// Start an app
    Start {
        /// App ID
        id: String,
    },
    /// Stop an app
    Stop {
        /// App ID
        id: String,
    },
    /// Restart an app
    Restart {
        /// App ID
        id: String,
    },
    /// Delete an app
    Delete {
        /// App ID
        id: String,
    },
}

#[derive(Debug, Args)]
pub struct AppCreateArgs {
    /// Template name (e.g. phoenixd, lnbits-phoenixd, alby-hub-phoenixd)
    pub template: String,

    /// Project ID (optional, a new project is created if omitted)
    #[arg(long, hide = true)]
    pub project_id: Option<String>,

    /// Template-specific params as `--key value` pairs
    #[arg(allow_hyphen_values = true, trailing_var_arg = true, hide = true)]
    pub params: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::{AppCommands, Cli, Commands, ProjectCommands};
    use clap::{Parser, error::ErrorKind};

    #[test]
    fn app_create_args_builds_params() {
        let args = super::AppCreateArgs {
            template: "phoenixd".into(),
            project_id: Some("prj-1".into()),
            params: vec![
                "--auto-liquidity".into(),
                "2m".into(),
                "--webhook".into(),
                "https://example.com/hook".into(),
            ],
        };

        let params = args
            .params()
            .expect("params to parse")
            .expect("params to be present");

        assert_eq!(
            params,
            serde_json::json!({
                "auto_liquidity": "2m",
                "webhook": "https://example.com/hook"
            })
        );
    }

    #[test]
    fn app_create_args_omits_empty_params() {
        let args = super::AppCreateArgs {
            template: "phoenixd".into(),
            project_id: None,
            params: vec![],
        };

        assert!(args.params().expect("params to parse").is_none());
    }

    #[test]
    fn project_create_uses_name_flag() {
        let cli = Cli::parse_from(["nod", "project", "create", "--name=test"]);

        match cli.command {
            Commands::Project(args) => match args.command {
                ProjectCommands::Create { name } => assert_eq!(name, "test"),
                other => panic!("expected create command, got {other:?}"),
            },
            other => panic!("expected project command, got {other:?}"),
        }
    }

    #[test]
    fn short_version_flag_displays_version() {
        let err = Cli::try_parse_from(["nod", "-V"]).expect_err("version flag to exit");

        assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    }

    #[test]
    fn long_version_flag_displays_version() {
        let err = Cli::try_parse_from(["nod", "--version"]).expect_err("version flag to exit");

        assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    }

    #[test]
    fn app_create_uses_named_flags() {
        let cli = Cli::parse_from([
            "nod",
            "app",
            "create",
            "phoenixd",
            "--project-id=prj-1",
            "--auto-liquidity=2m",
        ]);

        match cli.command {
            Commands::App(args) => match args.command {
                AppCommands::Create(args) => {
                    assert_eq!(args.template, "phoenixd");
                    assert_eq!(args.project_id.as_deref(), Some("prj-1"));
                    assert_eq!(
                        args.params().expect("params to parse"),
                        Some(serde_json::json!({"auto_liquidity": "2m"}))
                    );
                }
                other => panic!("expected create command, got {other:?}"),
            },
            other => panic!("expected app command, got {other:?}"),
        }
    }

    #[test]
    fn app_create_accepts_arbitrary_template_flags() {
        let cli = Cli::parse_from([
            "nod",
            "app",
            "create",
            "vss",
            "--project-id",
            "prj-1",
            "--vss-psql-address",
            "localhost",
            "--vss-psql-username",
            "alice",
        ]);

        match cli.command {
            Commands::App(args) => match args.command {
                AppCommands::Create(args) => {
                    assert_eq!(args.template, "vss");
                    assert_eq!(args.project_id.as_deref(), Some("prj-1"));
                    assert_eq!(
                        args.params,
                        vec![
                            "--vss-psql-address",
                            "localhost",
                            "--vss-psql-username",
                            "alice"
                        ]
                    );
                }
                other => panic!("expected create command, got {other:?}"),
            },
            other => panic!("expected app command, got {other:?}"),
        }
    }

    #[test]
    fn project_get_uses_positional_id() {
        let cli = Cli::parse_from(["nod", "project", "get", "prj-1"]);

        match cli.command {
            Commands::Project(args) => match args.command {
                ProjectCommands::Get { id } => assert_eq!(id, "prj-1"),
                other => panic!("expected get command, got {other:?}"),
            },
            other => panic!("expected project command, got {other:?}"),
        }
    }

    #[test]
    fn app_get_uses_positional_id() {
        let cli = Cli::parse_from(["nod", "app", "get", "app-1"]);

        match cli.command {
            Commands::App(args) => match args.command {
                AppCommands::Get { id } => assert_eq!(id, "app-1"),
                other => panic!("expected get command, got {other:?}"),
            },
            other => panic!("expected app command, got {other:?}"),
        }
    }
}
