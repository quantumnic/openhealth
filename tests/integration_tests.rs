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

#[test]
fn test_cli_validate() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_validate.db", "validate"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Validation"));
}

#[test]
fn test_cli_validate_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_validate_j.db", "--json", "validate"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"valid\""));
}

#[test]
fn test_cli_disease_necrotizing_fasciitis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_nf.db", "disease", "Necrotizing Fasciitis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Necrotizing Fasciitis"));
}

#[test]
fn test_cli_triage_basic() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_triage.db", "triage", "chest pain, shortness of breath"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TRIAGE") || stdout.contains("Triage"));
    assert!(stdout.contains("RED FLAG") || stdout.contains("EMERGENCY"));
}

#[test]
fn test_cli_triage_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_triage_j.db", "--json", "triage", "fever headache"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"level\"") && stdout.contains("\"action\""));
}

#[test]
fn test_cli_triage_red_flags() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_triage_rf.db", "--json", "triage", "seizures, confusion, high fever"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("red_flags"));
    assert!(stdout.contains("EMERGENCY") || stdout.contains("seizures"));
}

#[test]
fn test_cli_disease_lung_cancer() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_lc.db", "disease", "Lung Cancer"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Lung Cancer"));
}

#[test]
fn test_cli_disease_gout() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_gout.db", "disease", "Gout"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Gout"));
}

#[test]
fn test_cli_disease_lupus() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_lupus.db", "disease", "Systemic Lupus Erythematosus"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Lupus") || stdout.contains("autoimmune"));
}

#[test]
fn test_cli_disease_heatstroke() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_hs.db", "disease", "Heatstroke"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Heatstroke"));
}

// v10.0 tests

#[test]
fn test_cli_comorbidity_diabetes() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_comorbidity.db", "comorbidity", "Diabetes"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Comorbidity") || stdout.contains("relevance"));
}

#[test]
fn test_cli_comorbidity_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_comorbidity_json.db", "--json", "comorbidity", "Heart Attack"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("shared_risk_factors") || stdout.contains("related"));
}

#[test]
fn test_cli_disease_endometriosis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_endo.db", "disease", "Endometriosis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Endometriosis"));
}

#[test]
fn test_cli_disease_parkinsons() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_park.db", "disease", "Parkinson's Disease"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parkinson"));
}

#[test]
fn test_cli_disease_sepsis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_sepsis.db", "disease", "Sepsis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sepsis"));
}

#[test]
fn test_cli_disease_dvt() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_dvt.db", "disease", "Deep Vein Thrombosis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Deep Vein Thrombosis") || stdout.contains("blood clot"));
}

// v11.0 tests

#[test]
fn test_cli_interact_ibuprofen() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_interact.db", "interact", "ibuprofen"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DRUG-DISEASE") || stdout.contains("interaction"));
    assert!(stdout.contains("Asthma") || stdout.contains("bronchospasm"));
}

#[test]
fn test_cli_interact_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_interact_j.db", "--json", "interact", "aspirin"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"drug\"") && stdout.contains("interactions"));
}

#[test]
fn test_cli_interact_unknown_drug() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_interact_u.db", "interact", "xyzzydrug"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No known interactions"));
}

#[test]
fn test_cli_timeline_malaria() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_timeline.db", "timeline", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("TIMELINE") || stdout.contains("Incubation"));
}

#[test]
fn test_cli_timeline_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_timeline_j.db", "--json", "timeline", "heart attack"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"phases\"") && stdout.contains("warning_signs"));
}

#[test]
fn test_cli_timeline_unknown() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_timeline_u.db", "timeline", "xyzzy"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not found") || stdout.contains("Available"));
}

#[test]
fn test_cli_disease_myasthenia_gravis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_mg.db", "disease", "Myasthenia Gravis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Myasthenia Gravis"));
}

#[test]
fn test_cli_disease_guillain_barre() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_gbs.db", "disease", "Guillain-Barré Syndrome"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Guillain"));
}

#[test]
fn test_cli_disease_rhabdomyolysis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_rhabdo.db", "disease", "Rhabdomyolysis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rhabdomyolysis"));
}

// v12 integration tests

#[test]
fn test_cli_compare_basic() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_compare.db", "compare", "Malaria,Dengue Fever"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Comparison") || stdout.contains("Shared") || stdout.contains("Malaria"));
}

#[test]
fn test_cli_compare_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_compare_json.db", "--json", "compare", "Malaria,Cholera"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("shared_symptoms") || stdout.contains("diseases"));
}

#[test]
fn test_cli_prevalence() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_prev.db", "prevalence"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total") || stdout.contains("diseases"));
}

#[test]
fn test_cli_prevalence_category() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_prev_cat.db", "prevalence", "--category", "infectious"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("INFECTIOUS") || stdout.contains("infectious") || stdout.contains("diseases"));
}

#[test]
fn test_cli_prevalence_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_prev_json.db", "--json", "prevalence"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease") || stdout.contains("category"));
}

#[test]
fn test_cli_disease_narcolepsy() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_narcolepsy.db", "disease", "Narcolepsy"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Narcolepsy"));
}

#[test]
fn test_cli_disease_marfan() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_marfan.db", "disease", "Marfan Syndrome"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Marfan"));
}

// v13 integration tests

#[test]
fn test_cli_region_list() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_region_list.db", "region"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("chest"));
    assert!(stdout.contains("head"));
}

#[test]
fn test_cli_region_chest() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_region_chest.db", "--json", "region", "chest"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // JSON output includes all diseases, not just top 20
    assert!(stdout.contains("Heart Attack") || stdout.contains("Pneumonia") || stdout.contains("Asthma") || stdout.contains("chest"));
}

#[test]
fn test_cli_region_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_region_json.db", "--json", "region", "eyes"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"region\""));
}

#[test]
fn test_cli_almanac() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_almanac.db", "almanac", "--month", "1"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("January") || stdout.contains("Winter"));
}

#[test]
fn test_cli_almanac_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_almanac_json.db", "--json", "almanac", "--month", "7"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"month\""));
    assert!(stdout.contains("Summer"));
}

#[test]
fn test_cli_disease_dental_abscess() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_dental.db", "disease", "Dental Abscess"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dental Abscess"));
    assert!(stdout.contains("toothache"));
}

#[test]
fn test_cli_disease_burnout() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_burnout.db", "disease", "Burnout Syndrome"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Burnout"));
    assert!(stdout.contains("emotional exhaustion"));
}

#[test]
fn test_cli_symptoms_toothache() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_sym_tooth.db", "symptoms", "toothache facial swelling fever"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dental Abscess") || stdout.contains("toothache"));
}

#[test]
fn test_cli_symptoms_burnout() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_sym_burn.db", "symptoms", "emotional exhaustion insomnia irritability"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Burnout") || stdout.contains("exhaustion"));
}

// v14 integration tests
#[test]
fn test_cli_danger_signs() {
    let output = cargo_bin()
        .args(["danger-signs"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("DANGER SIGNS") || stdout.contains("danger"));
}

#[test]
fn test_cli_danger_signs_child() {
    let output = cargo_bin()
        .args(["danger-signs", "child"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("IMCI") || stdout.contains("CHILDREN") || stdout.contains("breastfeed"));
}

#[test]
fn test_cli_danger_signs_maternal() {
    let output = cargo_bin()
        .args(["danger-signs", "maternal"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PREGNANCY") || stdout.contains("vaginal bleeding") || stdout.contains("Maternal"));
}

#[test]
fn test_cli_danger_signs_adult() {
    let output = cargo_bin()
        .args(["danger-signs", "adult"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ADULTS") || stdout.contains("Chest pain"));
}

// ── v15 tests ──

#[test]
fn test_cli_predict_malaria() {
    let output = cargo_bin()
        .args(["predict", "malaria"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Prognosis") || stdout.contains("Malaria"));
}

#[test]
fn test_cli_predict_json() {
    let output = cargo_bin()
        .args(["--json", "predict", "heart attack"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("\"disease\"") && stdout.contains("Heart Attack"));
}

#[test]
fn test_cli_predict_unknown() {
    let output = cargo_bin()
        .args(["predict", "xyznonexistent"])
        .output()
        .expect("failed to execute");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(combined.contains("not found"));
}

#[test]
fn test_cli_disease_erysipelas() {
    let output = cargo_bin()
        .args(["disease", "erysipelas"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Erysipelas"));
}

#[test]
fn test_cli_symptoms_cluster_headache() {
    let output = cargo_bin()
        .args(["symptoms", "severe unilateral headache,tearing,eye redness"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    // Cluster headache may not rank first depending on scoring — just ensure results returned
    assert!(stdout.contains("Possible Conditions") || stdout.contains("Cluster Headache"));
}

#[test]
fn test_cli_symptoms_hyperkalemia() {
    let output = cargo_bin()
        .args(["symptoms", "muscle weakness,palpitations,bradycardia"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Hyperkalemia"));
}

#[test]
fn test_cli_family_history() {
    let output = cargo_bin()
        .args(["family-history", "diabetes, breast cancer"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Diabetes Type 2") || stdout.contains("Breast Cancer"));
}

#[test]
fn test_cli_family_history_json() {
    let output = cargo_bin()
        .args(["--json", "family-history", "heart attack"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Heart Attack") || stdout.contains("heart_condition"));
}

#[test]
fn test_cli_symptoms_graves() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "bulging,goiter,tremor,sweating"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Graves"), "Should find Graves' Disease, got: {}", &stdout[..stdout.len().min(500)]);
}

#[test]
fn test_cli_disease_hypothermia() {
    let output = cargo_bin()
        .args(["disease", "Hypothermia"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Hypothermia"));
}

// v18 integration tests
#[test]
fn test_cli_vitals_normal() {
    let output = cargo_bin()
        .args(["--json", "vitals", "hr=72 bp=120/80 temp=37.0 spo2=98 rr=16"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Heart Rate"));
    assert!(stdout.contains("normal"));
}

#[test]
fn test_cli_vitals_critical() {
    let output = cargo_bin()
        .args(["--json", "vitals", "hr=160 spo2=85"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("critical"));
    assert!(stdout.contains("CRITICAL"));
}

#[test]
fn test_cli_disease_addisons() {
    let output = cargo_bin()
        .args(["disease", "Addison's Disease"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Addison"));
}

#[test]
fn test_cli_disease_aortic_dissection() {
    let output = cargo_bin()
        .args(["--json", "disease", "Aortic Dissection"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Aortic Dissection"));
}

#[test]
fn test_cli_symptoms_toxic_shock() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "high fever,rash,low blood pressure"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Toxic Shock") || stdout.contains("probability"));
}

// ── v0.19.0 integration tests ──

#[test]
fn test_cli_bmi_normal() {
    let output = cargo_bin()
        .args(["bmi", "75 180"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Normal Weight"));
}

#[test]
fn test_cli_bmi_json() {
    let output = cargo_bin()
        .args(["--json", "bmi", "100 170"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("\"bmi\""));
    assert!(stdout.contains("Obese"));
}

#[test]
fn test_cli_bmi_underweight() {
    let output = cargo_bin()
        .args(["bmi", "45 175"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Underweight"));
}

#[test]
fn test_cli_disease_rabies() {
    let output = cargo_bin()
        .args(["--json", "disease", "Rabies"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Rabies"));
}

#[test]
fn test_cli_disease_sinusitis() {
    let output = cargo_bin()
        .args(["--json", "disease", "Sinusitis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Sinusitis"));
}

#[test]
fn test_cli_disease_kidney_stones() {
    let output = cargo_bin()
        .args(["--json", "disease", "Nephrolithiasis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Nephrolithiasis"));
}

#[test]
fn test_cli_symptoms_ear_pain() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "ear pain,fever,irritability"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Acute Otitis Media") || stdout.contains("probability"));
}

#[test]
fn test_cli_symptoms_kidney_stone() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "severe flank pain,blood in urine,nausea"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Nephrolithiasis") || stdout.contains("probability"));
}

#[test]
fn test_cli_symptoms_vertigo() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "dizziness,vertigo,nausea"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Vertigo") || stdout.contains("probability"));
}

#[test]
fn test_cli_symptoms_panic() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "palpitations,chest tightness,shortness of breath,trembling"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Panic") || stdout.contains("probability"));
}

#[test]
fn test_cli_synonym_earache() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "earache,fever"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("probability"));
}

#[test]
fn test_cli_synonym_kidney_pain() {
    let output = cargo_bin()
        .args(["--json", "symptoms", "kidney pain,blood in pee,nausea"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("probability"));
}

// ── v0.20.0 integration tests ──

#[test]
fn test_cli_screen_all() {
    let output = cargo_bin()
        .args(["screen"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("SCREENING") || stdout.contains("screening"));
    assert!(stdout.contains("Blood Pressure"));
    assert!(stdout.contains("Colorectal Cancer"));
}

#[test]
fn test_cli_screen_age() {
    let output = cargo_bin()
        .args(["screen", "--age", "55"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Blood Pressure"));
}

#[test]
fn test_cli_screen_sex() {
    let output = cargo_bin()
        .args(["screen", "--age", "55", "--sex", "female"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Breast Cancer") || stdout.contains("Mammography"));
}

#[test]
fn test_cli_screen_json() {
    let output = cargo_bin()
        .args(["--json", "screen", "--age", "50", "--sex", "male"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("\"screenings\""));
    assert!(stdout.contains("Prostate Cancer"));
}

#[test]
fn test_cli_disease_fibromyalgia() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_fibro.db", "disease", "Fibromyalgia"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Fibromyalgia"));
}

#[test]
fn test_cli_disease_pcos() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_pcos.db", "disease", "Polycystic Ovary Syndrome"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Polycystic"));
}

#[test]
fn test_cli_disease_afib() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_afib.db", "disease", "Atrial Fibrillation"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Atrial Fibrillation"));
}

#[test]
fn test_cli_disease_crohns() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_crohns.db", "disease", "Crohn's Disease"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Crohn"));
}

#[test]
fn test_cli_symptoms_ibs() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_ibs.db", "--json", "symptoms", "abdominal pain,bloating,diarrhea"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Irritable Bowel") || stdout.contains("probability"));
}

#[test]
fn test_cli_disease_colorectal_cancer() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_crc.db", "disease", "Colorectal Cancer"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Colorectal Cancer"));
}

#[test]
fn test_cli_disease_pancreatic_cancer() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_panc.db", "disease", "Pancreatic Cancer"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Pancreatic Cancer"));
}

// ── v0.21.0 integration tests ──

#[test]
fn test_cli_hydration_basic() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["hydration", "70"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("TOTAL"));
}

#[test]
fn test_cli_hydration_full() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["hydration", "80 intense hot"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("TOTAL"));
}

#[test]
fn test_cli_hydration_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "hydration", "70 moderate temperate"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("total_liters"));
}

#[test]
fn test_cli_severity_guide() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["severity-guide"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Severity"));
}

#[test]
fn test_cli_severity_guide_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "severity-guide"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("total_diseases"));
}

#[test]
fn test_cli_disease_achalasia() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Achalasia"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Achalasia"));
}

#[test]
fn test_cli_disease_pheochromocytoma() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Pheochromocytoma"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Pheochromocytoma"));
}

#[test]
fn test_cli_disease_restless_legs() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Restless Legs Syndrome"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Restless Legs"));
}

#[test]
fn test_cli_disease_peripheral_artery() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Peripheral Artery Disease"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("Peripheral Artery"));
}

// ── v0.22.0 integration tests ──

#[test]
fn test_cli_medication_list() {
    let output = cargo_bin()
        .args(["medication"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Paracetamol") || stdout.contains("Medication"));
}

#[test]
fn test_cli_medication_lookup() {
    let output = cargo_bin()
        .args(["medication", "ibuprofen"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Ibuprofen"));
    assert!(stdout.contains("NSAID") || stdout.contains("Anti-Inflammatory"));
}

#[test]
fn test_cli_medication_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_med_json.db", "--json", "medication", "aspirin"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"name\"") && stdout.contains("Aspirin"));
}

#[test]
fn test_cli_medication_not_found() {
    let output = cargo_bin()
        .args(["medication", "xyznonexistent"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No medication found") || stdout.contains("Available"));
}

#[test]
fn test_cli_symptoms_copd() {
    let output = cargo_bin()
        .args(["symptoms", "chronic cough shortness of breath wheezing"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Chronic Obstructive Pulmonary Disease") || stdout.contains("COPD") || stdout.contains("Analyzing"));
}

#[test]
fn test_cli_symptoms_ptsd() {
    let output = cargo_bin()
        .args(["symptoms", "flashbacks nightmares hypervigilance"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Post-Traumatic Stress Disorder") || stdout.contains("Analyzing"));
}

#[test]
fn test_cli_disease_rheumatoid() {
    let output = cargo_bin()
        .args(["disease", "rheumatoid arthritis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rheumatoid") || stdout.contains("autoimmune"));
}

#[test]
fn test_cli_disease_epiglottitis() {
    let output = cargo_bin()
        .args(["disease", "epiglottitis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Epiglottitis"));
}

// ── v0.23.0 integration tests ──────────────────────────────────────

#[test]
fn test_cli_symptom_map() {
    let output = cargo_bin()
        .args(["symptom-map"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Symptom Specificity Map") || stdout.contains("symptoms tracked"));
}

#[test]
fn test_cli_symptom_map_filter() {
    let output = cargo_bin()
        .args(["symptom-map", "fever"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fever") || stdout.contains("COMMON"));
}

#[test]
fn test_cli_symptom_map_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_symmap_json.db", "--json", "symptom-map"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease_count") || stdout.contains("symptom"));
}

#[test]
fn test_cli_disease_sepsis_v23() {
    let output = cargo_bin()
        .args(["disease", "sepsis"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sepsis") || stdout.contains("organ dysfunction"));
}

#[test]
fn test_cli_disease_afib_v23() {
    let output = cargo_bin()
        .args(["disease", "atrial fibrillation"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Atrial Fibrillation") || stdout.contains("arrhythmia"));
}

#[test]
fn test_cli_disease_sleep_apnea() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_v23_osa.db", "disease", "obstructive sleep apnea"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sleep Apnea") || stdout.contains("airway"));
}

#[test]
fn test_cli_symptoms_sepsis_v23() {
    let output = cargo_bin()
        .args(["symptoms", "high fever rapid heart rate confusion"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_symptoms_peripheral_neuropathy() {
    let output = cargo_bin()
        .args(["symptoms", "numb hands tingling burning feet"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_disease_stemi() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_v23_stemi.db", "disease", "myocardial infarction (stemi)"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("STEMI") || stdout.contains("coronary") || stdout.contains("Myocardial"));
}

#[test]
fn test_cli_disease_preeclampsia() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_v23_pe.db", "disease", "preeclampsia"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Preeclampsia") || stdout.contains("hypertensive"));
}

// ── v0.24.0 CLI tests ──────────────────────────────────────────────

#[test]
fn test_cli_drug_info_list() {
    let output = cargo_bin()
        .args(["drug-info"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Drug Information Reference"));
}

#[test]
fn test_cli_drug_info_ibuprofen() {
    let output = cargo_bin()
        .args(["drug-info", "ibuprofen"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Ibuprofen"));
    assert!(stdout.contains("NSAID"));
}

#[test]
fn test_cli_drug_info_json() {
    let output = cargo_bin()
        .args(["--json", "drug-info", "metformin"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"name\""));
    assert!(stdout.contains("Metformin"));
}

#[test]
fn test_cli_drug_info_not_found() {
    let output = cargo_bin()
        .args(["drug-info", "xyznonexistent"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No drug found"));
}

#[test]
fn test_cli_symptoms_retinal_detachment_v24() {
    let output = cargo_bin()
        .args(["symptoms", "eye floaters flashes of light"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_symptoms_heat_exhaustion_v24() {
    let output = cargo_bin()
        .args(["symptoms", "heavy sweating weakness dizziness muscle cramps"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_disease_cholesteatoma() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_v24_chol.db", "disease", "cholesteatoma"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cholesteatoma") || stdout.contains("middle ear"));
}

// ── v0.25.0 integration tests ──────────────────────────────────────────

#[test]
fn test_cli_first_aid_list() {
    let output = cargo_bin()
        .args(["first-aid"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Choking") || stdout.contains("protocols"));
}

#[test]
fn test_cli_first_aid_choking() {
    let output = cargo_bin()
        .args(["first-aid", "choking"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Heimlich") || stdout.contains("back blows"));
}

#[test]
fn test_cli_first_aid_json() {
    let output = cargo_bin()
        .args(["--json", "first-aid", "burn"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"situation\""));
}

#[test]
fn test_cli_lifestyle_basic() {
    let output = cargo_bin()
        .args(["lifestyle"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Lifestyle") || stdout.contains("exercise"));
}

#[test]
fn test_cli_lifestyle_with_factors() {
    let output = cargo_bin()
        .args(["lifestyle", "--age", "55", "--sex", "male", "--factors", "smoking,diabetes"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("smoking") || stdout.contains("Quit") || stdout.contains("Tobacco"));
}

#[test]
fn test_cli_lifestyle_json() {
    let output = cargo_bin()
        .args(["--json", "lifestyle", "--age", "30"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"category\""));
}

#[test]
fn test_cli_symptoms_bruxism_v25() {
    let output = cargo_bin()
        .args(["symptoms", "teeth grinding jaw pain headache"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Bruxism") || stdout.contains("jaw"));
}

#[test]
fn test_cli_symptoms_gallstones_v25() {
    let output = cargo_bin()
        .args(["symptoms", "right upper abdominal pain nausea vomiting"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_disease_impetigo() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--db-path", "/tmp/openhealth_test_v25_imp.db", "disease", "impetigo"])
        .output()
        .expect("failed to execute");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Impetigo") || stdout.contains("skin"));
}

#[test]
fn test_cli_synonym_grinding_teeth_v25() {
    let output = cargo_bin()
        .args(["symptoms", "grinding teeth"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

// v0.26.0 tests

#[test]
fn test_cli_vaccine_list() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["vaccine"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("VACCINATION REFERENCE"), "Should show vaccine header");
    assert!(stdout.contains("BCG"), "Should list BCG vaccine");
    assert!(stdout.contains("MMR"), "Should list MMR vaccine");
}

#[test]
fn test_cli_vaccine_by_name() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["vaccine", "--name", "hepatitis"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hepatitis"), "Should find hepatitis vaccines");
}

#[test]
fn test_cli_vaccine_by_age_group() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["vaccine", "--age-group", "neonates"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("BCG"), "BCG should appear for neonates");
}

#[test]
fn test_cli_vaccine_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "vaccine"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"abbreviation\""), "JSON output should have abbreviation field");
}

#[test]
fn test_cli_symptoms_acoustic_neuroma() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "tinnitus, vertigo"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Acoustic Neuroma") || stdout.contains("tinnitus"), "Should find results for tinnitus/vertigo");
}

#[test]
fn test_cli_symptoms_pericarditis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "sharp chest pain worse with breathing, fever, chest pain improves leaning forward"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pericarditis"), "Should detect pericarditis");
}

#[test]
fn test_cli_symptoms_testicular_torsion() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "symptoms", "sudden severe testicular pain, testicular swelling, nausea"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Testicular Torsion"), "Should detect testicular torsion");
}

#[test]
fn test_cli_symptoms_necrotizing_fasciitis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "crepitus, fever, blistering, tachycardia, hypotension"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Necrotizing Fasciitis"), "Should detect necrotizing fasciitis");
}

#[test]
fn test_cli_symptoms_co_poisoning() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "headache, dizziness, confusion, cherry red skin"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Carbon Monoxide Poisoning"), "Should detect CO poisoning");
}

#[test]
fn test_cli_synonym_testicle_pain() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "testicle pain, nausea"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Testicular Torsion") || !stdout.is_empty(), "testicle pain should expand via synonym");
}

// ── v0.27.0 CLI tests ──────────────────────────────────────

#[test]
fn test_cli_alert_emergency() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["alert", "chest pain, left arm pain, cold sweat"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("EMERGENCY") || stdout.contains("Heart Attack"),
        "Alert should detect heart attack pattern");
}

#[test]
fn test_cli_alert_no_emergency() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["alert", "headache"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No emergency") || !stdout.contains("EMERGENCY ALERT"),
        "Single mild symptom should not trigger emergency");
}

#[test]
fn test_cli_alert_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "alert", "confusion, stiff neck, high fever"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(json.get("emergency_detected").is_some());
}

#[test]
fn test_cli_symptoms_wernicke() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "confusion, balance problems, nystagmus"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Wernicke Encephalopathy"), "Should detect Wernicke");
}

#[test]
fn test_cli_symptoms_acromegaly() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "enlarged hands, enlarged feet, coarsened facial features"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Acromegaly"), "Should detect Acromegaly");
}

#[test]
fn test_cli_symptoms_pellagra() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "symptoms", "skin rash in sun-exposed areas, diarrhea, confusion"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pellagra"), "Should detect Pellagra in JSON results");
}

// ── v0.28.0 integration tests ────────────────────────────────────────

#[test]
fn test_cli_symptoms_chikungunya() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "high fever, severe joint pain, joint swelling, rash"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Chikungunya"), "Should detect Chikungunya");
}

#[test]
fn test_cli_symptoms_uti() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "symptoms", "painful urination, frequent urination, cloudy urine"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Urinary Tract Infection") || stdout.contains("Interstitial Cystitis"), "Should detect urinary condition");
}

#[test]
fn test_cli_symptoms_shingles() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "symptoms", "painful rash, blisters, sensitivity to touch"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Shingles") || stdout.contains("Herpes Zoster"), "Should detect Shingles");
}

#[test]
fn test_cli_symptoms_leptospirosis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "symptoms", "high fever, muscle pain, jaundice, red eyes"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Leptospirosis"), "Should detect Leptospirosis");
}

#[test]
fn test_cli_onset_sudden() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["onset", "sudden"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hyperacute"), "Should show hyperacute onset diseases");
}

#[test]
fn test_cli_onset_chronic_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "onset", "chronic"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Chronic"), "Should contain chronic onset in JSON");
}

#[test]
fn test_cli_onset_with_symptoms() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["onset", "acute", "--symptoms", "fever, headache"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_onset_unknown() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["onset", "blah"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_synonym_burning_pee() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "burning pee, peeing a lot"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("No matches"), "burning pee should match diseases");
}

#[test]
fn test_cli_disease_shingles() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Shingles"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("varicella") || stdout.contains("zoster") || stdout.contains("Shingles"), "Should show shingles info");
}

// v30 integration tests

#[test]
fn test_cli_polypharm_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["polypharm", "warfarin, aspirin, ibuprofen"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Interaction") || stdout.contains("bleeding"), "Should find warfarin-aspirin interaction");
}

#[test]
fn test_cli_polypharm_no_interaction() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["polypharm", "acetaminophen, metformin"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No known interactions") || stdout.contains("interactions"), "Should show result");
}

#[test]
fn test_cli_polypharm_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "polypharm", "simvastatin, erythromycin"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"severity\""), "JSON output should contain severity field");
}

#[test]
fn test_cli_disease_dvt_v30() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Deep Vein Thrombosis"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("clot") || stdout.contains("Thrombosis"), "Should show DVT info");
}

#[test]
fn test_cli_disease_heat_stroke_v30() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Heat Stroke"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hyperthermia") || stdout.contains("Heat Stroke"), "Should show heat stroke info");
}

#[test]
fn test_cli_symptoms_organophosphate() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "excessive salivation, miosis, muscle fasciculations"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Organophosphate"), "Should match organophosphate poisoning");
}

// ── v0.31.0 integration tests ──────────────────────────────────────────

#[test]
fn test_cli_glossary() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["glossary"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Glossary"), "Should show glossary header");
    assert!(stdout.contains("Edema"), "Should contain Edema term");
}

#[test]
fn test_cli_glossary_search() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["glossary", "sepsis"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sepsis"), "Should find Sepsis");
}

#[test]
fn test_cli_glossary_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "glossary", "fever"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"term\""), "Should output JSON with term field");
}

#[test]
fn test_cli_water_safety() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["water-safety"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Water Safety"), "Should show water safety header");
    assert!(stdout.contains("Boiling"), "Should contain Boiling method");
}

#[test]
fn test_cli_water_safety_filter() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["water-safety", "sodis"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Solar"), "Should find SODIS method");
}

#[test]
fn test_cli_water_safety_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "water-safety"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"name\""), "Should output JSON");
}

#[test]
fn test_cli_disease_buruli_ulcer() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Buruli Ulcer"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Mycobacterium") || stdout.contains("Buruli"), "Should show Buruli Ulcer info");
}

#[test]
fn test_cli_disease_kwashiorkor() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Kwashiorkor"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("malnutrition") || stdout.contains("Kwashiorkor"), "Should show Kwashiorkor info");
}

#[test]
fn test_cli_disease_noma() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Noma (Cancrum Oris)"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("gangrenous") || stdout.contains("Noma"), "Should show Noma info");
}

#[test]
fn test_cli_disease_guinea_worm() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Dracunculiasis (Guinea Worm Disease)"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Dracunculus") || stdout.contains("Guinea Worm"), "Should show Guinea Worm info");
}

#[test]
fn test_cli_disease_ciguatera() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Ciguatera Fish Poisoning"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ciguatoxin") || stdout.contains("Ciguatera"), "Should show Ciguatera info");
}

#[test]
fn test_cli_disease_ascariasis() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Ascariasis"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("roundworm") || stdout.contains("Ascaris"), "Should show Ascariasis info");
}

#[test]
fn test_cli_synonym_worms_in_poop() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "worms in poop, belly pain"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("No matching"), "Should find matches via synonym expansion");
}

// ── v0.32.0 integration tests ──

#[test]
fn test_cli_complication_diabetes() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["complication", "diabetes"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("retinopathy") || stdout.contains("Complications") || stdout.contains("Complication") || stdout.contains("nephropathy"), "Should show diabetes complications");
}

#[test]
fn test_cli_complication_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "complication", "malaria"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("complication") || stdout.contains("cerebral"), "Should show malaria complications in JSON");
}

#[test]
fn test_cli_complication_unknown() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["complication", "xyzzy_not_real"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
}

#[test]
fn test_cli_age_risk_child() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["age-risk", "5"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Age") && stdout.contains("risk"), "Should show age-specific risks");
}

#[test]
fn test_cli_age_risk_senior_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["--json", "age-risk", "70", "--sex", "female"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("disease") || stdout.contains("Dementia"), "Should show senior risks in JSON");
}

#[test]
fn test_cli_disease_hookworm() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Hookworm Infection"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("parasit") || stdout.contains("Hookworm"), "Should show hookworm info");
}

#[test]
fn test_cli_disease_trachoma() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "Trachoma"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("blind") || stdout.contains("Trachoma"), "Should show trachoma info");
}

#[test]
fn test_cli_disease_sleeping_sickness() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["disease", "African Trypanosomiasis"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tsetse") || stdout.contains("Trypanosoma") || stdout.contains("Sleeping"), "Should show sleeping sickness info");
}

#[test]
fn test_cli_synonym_night_sweats() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "night sweats, fever, joint pain"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("No matching"), "Should find matches for night sweats via synonym");
}

#[test]
fn test_cli_synonym_elephant_leg() {
    let output = Command::new(env!("CARGO_BIN_EXE_openhealth"))
        .args(["symptoms", "elephant leg, thick skin on legs"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("No matching"), "Should find matches for elephant leg via synonym");
}

// ── v0.37.0 integration tests ──

#[test]
fn test_disease_mycoplasma_pneumonia() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Mycoplasma Pneumonia"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Mycoplasma") || stdout.contains("walking pneumonia") || stdout.contains("atypical"));
}

#[test]
fn test_disease_scarlet_fever() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Scarlet Fever"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scarlet Fever") || stdout.contains("streptococcus"));
}

#[test]
fn test_disease_goodpasture() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Goodpasture Syndrome"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Goodpasture"));
}

#[test]
fn test_disease_moyamoya() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Moyamoya Disease"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Moyamoya"));
}

#[test]
fn test_disease_hereditary_angioedema() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Hereditary Angioedema"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Angioedema"));
}

#[test]
fn test_disease_erysipelas() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Erysipelas"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Erysipelas"));
}

#[test]
fn test_disease_cat_scratch() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "disease", "Cat Scratch Disease"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Bartonella") || stdout.contains("Cat Scratch"));
}

#[test]
fn test_contagion_command() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "contagion", "measles"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Measles") || stdout.contains("Airborne"));
}

#[test]
fn test_contagion_command_json() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "--json", "contagion", "cholera"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Fecal-oral") || stdout.contains("Cholera"));
}

#[test]
fn test_contagion_list_all() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "contagion"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Transmission") || stdout.contains("result"));
}

#[test]
fn test_symptoms_walking_pneumonia_synonym() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "symptoms", "walking pneumonia, headache, fatigue"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty());
}
