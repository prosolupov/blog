use std::fs;
use std::path::PathBuf;

use blog_client::{BlogClient, Transport};
use clap::{Parser, Subcommand};

const TOKEN_FILE: &str = ".blog_token";

#[derive(Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI client for blog backend", long_about = None)]
struct Cli {
    /// Use gRPC transport (default is HTTP)
    #[arg(long)]
    grpc: bool,

    /// Server address (e.g. http://127.0.0.1:8080 or http://127.0.0.1:50051)
    #[arg(long)]
    server: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    Login {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
    },
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    Get {
        #[arg(long)]
        id: String,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
    List {
        #[arg(long, default_value_t = 20)]
        limit: u32,
        #[arg(long, default_value_t = 0)]
        offset: u32,
    },
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let transport = if cli.grpc {
        let server = cli
            .server
            .unwrap_or_else(|| "http://127.0.0.1:50051".to_string());
        Transport::Grpc(server)
    } else {
        let server = cli
            .server
            .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
        Transport::Http(server)
    };

    let mut client = match BlogClient::new(transport).await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to create client: {err}");
            std::process::exit(1);
        }
    };

    if let Some(token) = read_token() {
        client.set_token(token);
    }

    let result = match cli.command {
        Commands::Register { username, email, password } => {
            match client.register(&username, &email, &password).await {
                Ok(auth) => {
                    let _ = write_token(&auth.access_token);
                    println!("{auth:?}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Commands::Login { username, password } => {
            match client.login(&username, &password).await {
                Ok(auth) => {
                    let _ = write_token(&auth.access_token);
                    println!("{auth:?}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Commands::Create { title, content } => {
            match client.create_post(&title, &content).await {
                Ok(post) => {
                    println!("{post:?}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Commands::Get { id } => {
            match client.get_post(&id).await {
                Ok(post) => {
                    println!("{post:?}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Commands::Update { id, title, content } => {
            match client.update_post(&id, &title, &content).await {
                Ok(post) => {
                    println!("{post:?}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Commands::Delete { id } => {
            match client.delete_post(&id).await {
                Ok(ok) => {
                    println!("deleted: {ok}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        Commands::List { limit, offset } => {
            match client.list_posts(limit, offset).await {
                Ok(list) => {
                    println!("{list:?}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn token_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(TOKEN_FILE)
}

fn read_token() -> Option<String> {
    fs::read_to_string(token_path()).ok().map(|s| s.trim().to_string())
}

fn write_token(token: &str) -> std::io::Result<()> {
    fs::write(token_path(), token)
}
