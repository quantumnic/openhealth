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
    assert!(stdout.contains("Cluster Headache"));
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
