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
    /// View diagnosis history
    History {
        /// Show specific entry by ID
        #[arg(long)]
        id: Option<i64>,
        /// Max entries to show (default: 20)
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Export full database as JSON
    Export {
        /// Output file path (prints to stdout if omitted)
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Set user profile for age/sex-aware scoring
    Profile {
        /// Age in years
        #[arg(long)]
        age: Option<u8>,
        /// Sex: male or female
        #[arg(long)]
        sex: Option<String>,
        /// Show current profile
        #[arg(long, default_value_t = false)]
        show: bool,
        /// Clear saved profile
        #[arg(long, default_value_t = false)]
        clear: bool,
    },
    /// Find diseases similar to a given disease (shared symptoms)
    Similar {
        /// Disease name
        name: String,
        /// Max results (default: 5)
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
    /// Assess disease risk based on personal risk factors
    Risk {
        /// Comma-separated risk factors, e.g. "smoking, obesity, diabetes"
        factors: String,
    },
    /// Explore diseases grouped by body system
    #[command(name = "body-system")]
    BodySystem {
        /// Filter by body system, e.g. "respiratory" or "cardiovascular"
        system: Option<String>,
    },
    /// Quick triage assessment — red flag detection and severity classification
    Triage {
        /// Comma or space-separated symptoms
        symptoms: String,
    },
    /// Validate database integrity and report issues
    Validate,
    /// Find diseases that commonly co-occur (shared risk factors and symptoms)
    Comorbidity {
        /// Disease name
        name: String,
        /// Max results (default: 10)
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Check drug-disease interactions and contraindications
    Interact {
        /// Drug name, e.g. "ibuprofen" or "metformin"
        drug: String,
    },
    /// Show expected disease progression timeline
    Timeline {
        /// Disease name, e.g. "malaria" or "heart attack"
        name: String,
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
        Commands::History { id, limit } => match id {
            Some(entry_id) => commands::history::show_detail(&conn, entry_id, cli.json),
            None => commands::history::run(&conn, limit, cli.json),
        },
        Commands::Export { output } => {
            commands::export::run(&conn, output.as_deref());
        }
        Commands::Profile { age, sex, show, clear } => {
            commands::profile::run(&conn, age, sex.as_deref(), show, clear, cli.json);
        }
        Commands::Similar { name, limit } => {
            commands::similar::run(&conn, &name, limit, cli.json);
        }
        Commands::Risk { factors } => {
            commands::risk::run(&conn, &factors, cli.json);
        }
        Commands::BodySystem { system } => {
            commands::body_system::run(&conn, system.as_deref(), cli.json);
        }
        Commands::Triage { symptoms } => {
            commands::triage::run(&conn, &symptoms, cli.json);
        }
        Commands::Validate => {
            commands::validate::run(&conn, cli.json);
        }
        Commands::Comorbidity { name, limit } => {
            commands::comorbidity::run(&conn, &name, limit, cli.json);
        }
        Commands::Interact { drug } => {
            commands::interact::run(&conn, &drug, cli.json);
        }
        Commands::Timeline { name } => {
            commands::timeline::run(&conn, &name, cli.json);
        }
    }
}
