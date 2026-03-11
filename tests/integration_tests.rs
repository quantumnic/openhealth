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
