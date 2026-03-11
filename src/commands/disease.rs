use crate::display;
use rusqlite::Connection;

pub fn run(conn: &Connection, name: &str, json: bool) {
    if !json {
        display::print_banner();
    }

    let disease = conn.query_row(
        "SELECT id, name, description, severity, contagious, icd11_code, age_group, category FROM diseases WHERE name LIKE ?1",
        [format!("%{name}%")],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i32>(4)? != 0,
                row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                row.get::<_, Option<String>>(6)?.unwrap_or_else(|| "all".to_string()),
                row.get::<_, Option<String>>(7)?.unwrap_or_else(|| "general".to_string()),
            ))
        },
    );

    match disease {
        Ok((id, dname, desc, severity, contagious, icd11, age_group, category)) => {
            let mut stmt = conn
                .prepare(
                    "SELECT s.name, ds.weight, ds.is_primary FROM disease_symptoms ds JOIN symptoms s ON s.id = ds.symptom_id WHERE ds.disease_id = ?1 ORDER BY ds.weight DESC",
                )
                .unwrap();
            let symptoms: Vec<(String, f64, bool)> = stmt
                .query_map([id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get::<_, i32>(2)? != 0))
                })
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            // Get risk factors
            let mut rf_stmt = conn
                .prepare("SELECT factor, impact FROM risk_factors WHERE disease_id = ?1")
                .unwrap();
            let risk_factors: Vec<(String, String)> = rf_stmt
                .query_map([id], |row| Ok((row.get(0)?, row.get(1)?)))
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            if json {
                let sym_json: Vec<serde_json::Value> = symptoms
                    .iter()
                    .map(|(n, w, p)| {
                        serde_json::json!({"name": n, "weight": w, "primary": p})
                    })
                    .collect();
                let rf_json: Vec<serde_json::Value> = risk_factors
                    .iter()
                    .map(|(f, i)| serde_json::json!({"factor": f, "impact": i}))
                    .collect();
                let obj = serde_json::json!({
                    "name": dname,
                    "description": desc,
                    "severity": severity,
                    "contagious": contagious,
                    "icd11_code": icd11,
                    "age_group": age_group,
                    "category": category,
                    "symptoms": sym_json,
                    "risk_factors": rf_json,
                });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
                return;
            }

            display::print_disease_info(
                &dname,
                &desc,
                &severity,
                contagious,
                &icd11,
                &age_group,
                &category,
                &symptoms,
                &risk_factors,
            );
        }
        Err(_) => {
            if json {
                println!("null");
            } else {
                println!("Disease '{}' not found in database.", name);
                println!("Try: openhealth disease \"malaria\"");
            }
        }
    }
}
