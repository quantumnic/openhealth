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

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Interactive symptom checker — guided Q&A
    Check,
    /// Quick symptom analysis
    Symptoms {
        /// Space-separated symptoms, e.g. "fever headache nausea"
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
    /// Update the local database
    Update,
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
        Commands::Symptoms { symptoms } => commands::symptoms::run(&conn, &symptoms),
        Commands::Disease { name } => commands::disease::run(&conn, &name),
        Commands::Treatment { name } => commands::treatment::run(&conn, &name),
        Commands::Emergency => commands::emergency::run(),
        Commands::Update => commands::update::run(&conn),
    }
}
