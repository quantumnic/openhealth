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
    /// Calculate Body Mass Index and health classification
    Bmi {
        /// Weight (kg) and height (cm), e.g. "75 180"
        input: String,
    },
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
    /// Compare multiple diseases side by side (symptoms, severity, risk factors)
    Compare {
        /// Disease names (2-5), comma-separated, e.g. "malaria,dengue fever,typhoid fever"
        diseases: String,
    },
    /// Find diseases that commonly co-occur (shared risk factors and symptoms)
    Comorbidity {
        /// Disease name
        name: String,
        /// Max results (default: 10)
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Disease prevalence overview grouped by category
    Prevalence {
        /// Filter by category (infectious, respiratory, cardiovascular, etc.)
        #[arg(long)]
        category: Option<String>,
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
    /// Explore diseases by body region (head, chest, abdomen, etc.)
    Region {
        /// Body region, e.g. "chest", "head", "abdomen" (omit to list all)
        region: Option<String>,
    },
    /// Seasonal health almanac — diseases and tips for each month
    Almanac {
        /// Month number (1-12, default: current month)
        #[arg(long)]
        month: Option<u32>,
    },
    /// Predict disease prognosis, complications, and outcomes
    Predict {
        /// Disease name, e.g. "malaria" or "heart attack"
        name: String,
    },
    /// WHO danger signs — critical warning signs for children, maternal, and neonatal emergencies
    #[command(name = "danger-signs")]
    DangerSigns {
        /// Category: child, maternal, neonatal, adult (omit for all)
        category: Option<String>,
    },
    /// Assess disease risk based on family medical history
    #[command(name = "family-history")]
    FamilyHistory {
        /// Comma-separated conditions in family, e.g. "diabetes, breast cancer, heart attack"
        conditions: String,
    },
    /// Interactive medical knowledge quiz — test your health knowledge
    Quiz {
        /// Number of questions (default: 10)
        #[arg(long, default_value_t = 10)]
        count: usize,
    },
    /// Interpret vital signs — heart rate, blood pressure, temperature, SpO2, respiratory rate
    Vitals {
        /// Vital signs as key=value pairs, e.g. "hr=72 bp=120/80 temp=37.2 spo2=98 rr=16"
        input: String,
    },
    /// Age- and sex-appropriate health screening recommendations (USPSTF, WHO, ACS)
    Screen {
        /// Age in years (filters to applicable screenings)
        #[arg(long)]
        age: Option<u8>,
        /// Sex: male or female (filters sex-specific screenings)
        #[arg(long)]
        sex: Option<String>,
    },
    /// Daily water intake calculator based on weight, activity, and climate
    Hydration {
        /// Weight (kg), activity level, climate, e.g. "70 moderate hot"
        input: String,
    },
    /// Severity classification guide with database statistics
    #[command(name = "severity-guide")]
    SeverityGuide,
    /// Medication reference — dosage, side effects, interactions, and contraindications
    Medication {
        /// Medication name or class to look up (omit to list all)
        name: Option<String>,
    },
    /// Show symptom specificity — how many diseases share each symptom
    #[command(name = "symptom-map")]
    SymptomMap {
        /// Filter symptoms by name (partial match)
        filter: Option<String>,
    },
    /// Drug reference — dosage, side effects, warnings, contraindications
    #[command(name = "drug-info")]
    DrugInfo {
        /// Drug name or class to look up (omit to list all)
        name: Option<String>,
    },
    /// First-aid quick reference — step-by-step emergency protocols
    #[command(name = "first-aid")]
    FirstAid {
        /// Situation to look up (e.g. "choking", "burn", "CPR"). Omit to list all.
        situation: Option<String>,
    },
    /// Vaccination reference — WHO schedules, doses, contraindications
    Vaccine {
        /// Filter by age group: neonates, infants, children, adults, elderly, all
        #[arg(long)]
        age_group: Option<String>,
        /// Search by vaccine name, abbreviation, or disease prevented
        #[arg(long)]
        name: Option<String>,
    },
    /// Emergency alert — check symptoms against critical emergency patterns
    Alert {
        /// Comma-separated symptoms, e.g. "chest pain, left arm pain, cold sweat"
        symptoms: String,
    },
    /// Personalized lifestyle and health recommendations
    Lifestyle {
        /// Age in years
        #[arg(long)]
        age: Option<u8>,
        /// Sex: male or female
        #[arg(long)]
        sex: Option<String>,
        /// Comma-separated risk factors, e.g. "smoking, obesity, diabetes"
        #[arg(long)]
        factors: Option<String>,
    },
    /// Nutritional deficiency reference and symptom-based assessment
    Nutrition {
        /// Search by nutrient name, symptom, or disease (omit to list all)
        query: Option<String>,
        /// Assess symptoms for possible nutritional deficiencies
        #[arg(long)]
        assess: Option<String>,
    },
    /// Polypharmacy checker — multi-drug interaction analysis
    Polypharm {
        /// Comma-separated medication names, e.g. "warfarin, aspirin, ibuprofen"
        drugs: String,
    },
    /// Medical terminology glossary — plain-language definitions
    Glossary {
        /// Search for a term (partial match). Omit to list all.
        query: Option<String>,
    },
    /// Water purification and safety guide — methods for making water safe to drink
    #[command(name = "water-safety")]
    WaterSafety {
        /// Filter by method name (boiling, sodis, chlorination, ceramic, cloth, tablets)
        method: Option<String>,
    },
    /// Filter diseases by symptom onset speed (sudden, acute, subacute, chronic)
    Onset {
        /// Onset type: sudden (seconds), acute (hours), subacute (days), chronic (weeks+)
        onset_type: String,
        /// Optional: comma-separated symptoms to cross-reference
        #[arg(long)]
        symptoms: Option<String>,
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
        Commands::Bmi { input } => {
            commands::bmi::run(&input, cli.json);
        }
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
        Commands::Compare { diseases } => {
            let names: Vec<&str> = diseases.split(',').map(|s| s.trim()).collect();
            commands::compare::run(&conn, &names, cli.json);
        }
        Commands::Comorbidity { name, limit } => {
            commands::comorbidity::run(&conn, &name, limit, cli.json);
        }
        Commands::Prevalence { category } => {
            commands::prevalence::run(&conn, category.as_deref(), cli.json);
        }
        Commands::Interact { drug } => {
            commands::interact::run(&conn, &drug, cli.json);
        }
        Commands::Timeline { name } => {
            commands::timeline::run(&conn, &name, cli.json);
        }
        Commands::Region { region } => {
            commands::region::run(&conn, region.as_deref(), cli.json);
        }
        Commands::Almanac { month } => {
            commands::almanac::run(&conn, month, cli.json);
        }
        Commands::Predict { name } => {
            commands::predict::run(&conn, &name, cli.json);
        }
        Commands::DangerSigns { category } => {
            commands::danger_signs::run(category.as_deref(), cli.json);
        }
        Commands::FamilyHistory { conditions } => {
            commands::family_history::run(&conn, &conditions, cli.json);
        }
        Commands::Quiz { count } => {
            commands::quiz::run(&conn, count);
        }
        Commands::Vitals { input } => {
            commands::vitals::run(&input, cli.json);
        }
        Commands::Screen { age, sex } => {
            commands::screen::run(age, sex.as_deref(), cli.json);
        }
        Commands::Hydration { input } => {
            commands::hydration::run(&input, cli.json);
        }
        Commands::SeverityGuide => {
            commands::severity_guide::run(&conn, cli.json);
        }
        Commands::Medication { name } => {
            if let Some(name) = name {
                commands::medication::run(&name, cli.json);
            } else {
                commands::medication::run_list(cli.json);
            }
        }
        Commands::SymptomMap { filter } => {
            commands::symptom_map::run(&conn, filter.as_deref(), cli.json);
        }
        Commands::DrugInfo { name } => {
            if let Some(name) = name {
                commands::drug_info::run(&name, cli.json);
            } else {
                commands::drug_info::run_list(cli.json);
            }
        }
        Commands::FirstAid { situation } => {
            commands::first_aid::run(situation.as_deref(), cli.json);
        }
        Commands::Vaccine { age_group, name } => {
            commands::vaccine::run(age_group.as_deref(), name.as_deref(), cli.json);
        }
        Commands::Lifestyle { age, sex, factors } => {
            commands::lifestyle::run(&conn, age, sex.as_deref(), factors.as_deref(), cli.json);
        }
        Commands::Alert { symptoms } => {
            commands::alert::run(&conn, &symptoms, cli.json);
        }
        Commands::Nutrition { query, assess } => {
            if let Some(symptoms) = assess {
                commands::nutrition::assess(&symptoms, cli.json);
            } else {
                commands::nutrition::run(query.as_deref(), cli.json);
            }
        }
        Commands::Polypharm { drugs } => {
            commands::polypharm::run(&conn, &drugs, cli.json);
        }
        Commands::Onset { onset_type, symptoms } => {
            commands::onset::run(&conn, &onset_type, symptoms.as_deref(), cli.json);
        }
        Commands::Glossary { query } => {
            commands::glossary::run(query.as_deref(), cli.json);
        }
        Commands::WaterSafety { method } => {
            commands::water_safety::run(method.as_deref(), cli.json);
        }
    }
}
