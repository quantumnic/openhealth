use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct ExportData {
    version: String,
    disease_count: usize,
    diseases: Vec<ExportDisease>,
}

#[derive(Serialize)]
struct ExportDisease {
    name: String,
    description: String,
    severity: String,
    contagious: bool,
    icd11_code: Option<String>,
    age_group: String,
    category: String,
    symptoms: Vec<ExportSymptom>,
    treatment: Option<ExportTreatment>,
    risk_factors: Vec<ExportRiskFactor>,
}

#[derive(Serialize)]
struct ExportSymptom {
    name: String,
    weight: f64,
    is_primary: bool,
}

#[derive(Serialize)]
struct ExportTreatment {
    protocol: String,
    source: String,
    first_aid: Option<String>,
    prevention: Option<String>,
}

#[derive(Serialize)]
struct ExportRiskFactor {
    factor: String,
    impact: String,
}

pub fn run(conn: &Connection, output_path: Option<&str>) {
    let mut stmt = conn
        .prepare("SELECT id, name, description, severity, contagious, icd11_code, age_group, category FROM diseases ORDER BY name")
        .unwrap();

    struct DiseaseRow {
        id: i64,
        name: String,
        description: String,
        severity: String,
        contagious: bool,
        icd11_code: Option<String>,
        age_group: String,
        category: String,
    }

    let diseases_raw: Vec<DiseaseRow> = stmt
        .query_map([], |row| {
            Ok(DiseaseRow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                severity: row.get(3)?,
                contagious: row.get::<_, i32>(4)? != 0,
                icd11_code: row.get(5)?,
                age_group: row.get(6)?,
                category: row.get(7)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let mut diseases = Vec::new();

    for dr in &diseases_raw {
        let symptoms = get_symptoms(conn, dr.id);
        let treatment = get_treatment(conn, dr.id);
        let risk_factors = get_risk_factors(conn, dr.id);

        diseases.push(ExportDisease {
            name: dr.name.clone(),
            description: dr.description.clone(),
            severity: dr.severity.clone(),
            contagious: dr.contagious,
            icd11_code: dr.icd11_code.clone(),
            age_group: dr.age_group.clone(),
            category: dr.category.clone(),
            symptoms,
            treatment,
            risk_factors,
        });
    }

    let export = ExportData {
        version: env!("CARGO_PKG_VERSION").to_string(),
        disease_count: diseases.len(),
        diseases,
    };

    let json = serde_json::to_string_pretty(&export).unwrap();

    match output_path {
        Some(path) => {
            std::fs::write(path, &json).expect("Failed to write export file");
            eprintln!("✅ Exported {} diseases to {}", export.disease_count, path);
        }
        None => {
            println!("{json}");
        }
    }
}

fn get_symptoms(conn: &Connection, disease_id: i64) -> Vec<ExportSymptom> {
    let mut stmt = conn
        .prepare(
            "SELECT s.name, ds.weight, ds.is_primary FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.disease_id = ?1 ORDER BY ds.weight DESC",
        )
        .unwrap();
    stmt.query_map([disease_id], |row| {
        Ok(ExportSymptom {
            name: row.get(0)?,
            weight: row.get(1)?,
            is_primary: row.get::<_, i32>(2)? != 0,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}

fn get_treatment(conn: &Connection, disease_id: i64) -> Option<ExportTreatment> {
    conn.query_row(
        "SELECT protocol, source, first_aid, prevention FROM treatments WHERE disease_id = ?1",
        [disease_id],
        |row| {
            Ok(ExportTreatment {
                protocol: row.get(0)?,
                source: row.get(1)?,
                first_aid: row.get(2)?,
                prevention: row.get(3)?,
            })
        },
    )
    .ok()
}

fn get_risk_factors(conn: &Connection, disease_id: i64) -> Vec<ExportRiskFactor> {
    let mut stmt = conn
        .prepare("SELECT factor, impact FROM risk_factors WHERE disease_id = ?1")
        .unwrap();
    stmt.query_map([disease_id], |row| {
        Ok(ExportRiskFactor {
            factor: row.get(0)?,
            impact: row.get(1)?,
        })
    })
    .unwrap()
    .filter_map(|r| r.ok())
    .collect()
}
