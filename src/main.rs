mod commands;
mod db;
mod display;
mod engine;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "openhealth")]
#[command(about = "Offline AI Medical Diagnostics for Everyone")]
#[command(version)]
pub struct Cli {
    /// Path to database file (default: ~/.openhealth/openhealth.db)
    #[arg(long, global = true)]
    db_path: Option<PathBuf>,

    /// Output results as JSON
    #[arg(long, global = true, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Interactive symptom checker — guided Q&A
    Check,
    /// Quick symptom analysis
    Symptoms {
        /// Comma or space-separated symptoms, e.g. "fever headache nausea"
        symptoms: String,
    },
    /// Disease information lookup
    Disease {
        /// Disease name, e.g. "malaria"
        name: String,
    },
    /// WHO treatment protocol
    Treatment {
        /// Disease name, e.g. "malaria"
        name: String,
    },
    /// Emergency checklist
    Emergency,
    /// List all diseases in the database
    List {
        /// Filter by category (infectious, respiratory, cardiovascular, etc.)
        #[arg(long)]
        category: Option<String>,
    },
    /// Database statistics
    Stats,
    /// Update the local database
    Update,
    /// Search symptoms and diseases in the database
    Search {
        /// Search query, e.g. "fever" or "malaria"
        query: String,
    },
    /// Differential diagnosis — compare two diseases side by side
    Diff {
        /// First disease name
        disease_a: String,
        /// Second disease name
        disease_b: String,
    },
}

fn default_db_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".openhealth")
        .join("openhealth.db")
}

fn main() {
    let cli = Cli::parse();
    let db_path = cli.db_path.unwrap_or_else(default_db_path);

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let conn = match db::init_database(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to initialize database: {e}");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Check => commands::check::run(&conn),
        Commands::Symptoms { symptoms } => commands::symptoms::run(&conn, &symptoms, cli.json),
        Commands::Disease { name } => commands::disease::run(&conn, &name, cli.json),
        Commands::Treatment { name } => commands::treatment::run(&conn, &name, cli.json),
        Commands::Emergency => commands::emergency::run(),
        Commands::List { category } => {
            commands::list::run(&conn, category.as_deref(), cli.json);
        }
        Commands::Stats => commands::stats::run(&conn),
        Commands::Update => commands::update::run(&conn),
        Commands::Search { query } => commands::search::run(&conn, &query, cli.json),
        Commands::Diff { disease_a, disease_b } => {
            commands::diff::run(&conn, &disease_a, &disease_b, cli.json);
        }
    }
}
