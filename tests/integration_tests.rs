use std::process::Command;

fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_openhealth"));
    cmd.arg("--db-path").arg("/tmp/openhealth_test_integration.db");
    cmd
}

#[test]
fn test_cli_symptoms_runs() {
    let output = cargo_bin()
        .args(["symptoms", "fever headache"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OpenHealth") || stdout.contains("Analyzing"));
}

#[test]
fn test_cli_symptoms_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_integration_json.db", "--json", "symptoms", "fever headache"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease_name") || stdout.contains("probability"));
}

#[test]
fn test_cli_disease_malaria() {
    let output = cargo_bin()
        .args(["disease", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Malaria"));
}

#[test]
fn test_cli_disease_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_integration_djson.db", "--json", "disease", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"name\"") && stdout.contains("Malaria"));
}

#[test]
fn test_cli_disease_not_found() {
    let output = cargo_bin()
        .args(["disease", "xyznonexistent"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not found"));
}

#[test]
fn test_cli_treatment_cholera() {
    let output = cargo_bin()
        .args(["treatment", "cholera"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ORS") || stdout.contains("Cholera"));
}

#[test]
fn test_cli_emergency() {
    let output = cargo_bin()
        .args(["emergency"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("EMERGENCY") || stdout.contains("CPR"));
}

#[test]
fn test_cli_list() {
    let output = cargo_bin()
        .args(["list"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Malaria") || stdout.contains("Disease Database"));
}

#[test]
fn test_cli_list_category() {
    let output = cargo_bin()
        .args(["list", "--category", "infectious"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Malaria"));
}

#[test]
fn test_cli_stats() {
    let output = cargo_bin()
        .args(["stats"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Statistics") && stdout.contains("Diseases"));
}

#[test]
fn test_cli_update() {
    let output = cargo_bin()
        .args(["update"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Database") || stdout.contains("database"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .arg("--version")
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("openhealth"));
}

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .arg("--help")
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Offline AI Medical Diagnostics"));
}

#[test]
fn test_cli_search() {
    let output = cargo_bin()
        .args(["search", "fever"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fever") || stdout.contains("Search"));
}

#[test]
fn test_cli_search_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_search.db", "--json", "search", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Malaria"));
}

#[test]
fn test_cli_diff() {
    let output = cargo_bin()
        .args(["diff", "malaria", "dengue"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Malaria") || stdout.contains("Dengue") || stdout.contains("Shared"));
}

#[test]
fn test_cli_diff_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_diff.db", "--json", "diff", "malaria", "dengue"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("shared_symptoms") || stdout.contains("disease_a"));
}

#[test]
fn test_cli_diff_not_found() {
    let output = cargo_bin()
        .args(["diff", "xyznothing", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not found"));
}

#[test]
fn test_cli_history_empty() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_hist.db", "history"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No diagnosis history") || stdout.contains("History"));
}

#[test]
fn test_cli_history_json_empty() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_hist_j.db", "--json", "history"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[]"));
}

#[test]
fn test_cli_export() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_export.db", "export"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease_count") && stdout.contains("Malaria"));
}

#[test]
fn test_cli_export_file() {
    let _ = std::fs::remove_file("/tmp/openhealth_test_export_out.json");
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_export2.db", "export", "--output", "/tmp/openhealth_test_export_out.json"])
        .output()
        .expect("failed to execute");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Exported"));
    let contents = std::fs::read_to_string("/tmp/openhealth_test_export_out.json").unwrap();
    assert!(contents.contains("Malaria"));
}

#[test]
fn test_cli_profile_show() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_profile.db", "profile", "--show"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Profile") || stdout.contains("not set"));
}

#[test]
fn test_cli_profile_set() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_profile_set.db", "profile", "--age", "30", "--sex", "male"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("updated") || stdout.contains("Updated") || stdout.contains("✅"));
}

#[test]
fn test_cli_profile_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_profile_j.db", "--json", "profile", "--show"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("age") && stdout.contains("sex"));
}

#[test]
fn test_cli_similar() {
    let output = cargo_bin()
        .args(["similar", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Similar") || stdout.contains("similar"));
}

#[test]
fn test_cli_similar_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_similar.db", "--json", "similar", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("target") && stdout.contains("similar_diseases"));
}

#[test]
fn test_cli_similar_not_found() {
    let output = cargo_bin()
        .args(["similar", "xyznothing"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not found"));
}

#[test]
fn test_cli_body_system_overview() {
    let output = cargo_bin()
        .args(["body-system"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Body Systems") || stdout.contains("body systems"));
}

#[test]
fn test_cli_body_system_filter() {
    let output = cargo_bin()
        .args(["body-system", "respiratory"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Respiratory"));
}

#[test]
fn test_cli_risk_smoking() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_risk.db", "risk", "smoking, obesity"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Risk Assessment") || stdout.contains("risk"));
}

#[test]
fn test_cli_risk_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_risk_j.db", "--json", "risk", "smoking"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease") || stdout.contains("risk_score") || stdout.contains("[]"));
}

#[test]
fn test_cli_disease_pulmonary_embolism() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_pe.db", "disease", "Pulmonary Embolism"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pulmonary Embolism"));
}

#[test]
fn test_cli_body_system_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_bs.db", "--json", "body-system"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease_count") || stdout.contains("system"));
}
