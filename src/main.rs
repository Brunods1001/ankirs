mod app;
mod models;
mod queries;

use app::start_app;

use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// shows menu and starts the app
    Start,
    /// oversees all cards in the app
    Card {
        #[command(subcommand)]
        command: Option<CardCommands>,
    },
}

#[derive(Subcommand)]
enum CardCommands {
    /// lists all cards
    List,
    /// creates a new card
    Create {
        /// the front of the card
        #[arg(short, long)]
        front: String,

        /// the back of the card
        #[arg(short, long)]
        back: String,
    },
    /// updates an existing cards
    Update {
        /// the id of the card
        #[arg(short, long)]
        id: i64,

        /// the front of the card
        #[arg(short, long)]
        front: Option<String>,

        /// the back of the card
        #[arg(short, long)]
        back: Option<String>,
    },
    /// deletes an existing card
    Delete {
        /// the id of the card
        #[arg(short, long)]
        id: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start) => {
            println!("Starting app");
            start_app(pool).await?;
        }
        _ => println!("no command given"),
        // Some(Commands::Card { command }) => match command {
        //     Some(CardCommands::List) => {
        //         list_cards(&mut tx).await?;
        //     }
        //     Some(CardCommands::Create { front, back }) => {
        //         create_card(&mut tx, front, back).await?;
        //     }
        //     Some(CardCommands::Update { id, front, back }) => {
        //         update_card(&mut tx, id, front, back).await?;
        //     }
        //     Some(CardCommands::Delete { id }) => {
        //         delete_card(&mut tx, id).await?;
        //     }
        //     None => println!("no command given"),
        // },
    }

    Ok(())
}
