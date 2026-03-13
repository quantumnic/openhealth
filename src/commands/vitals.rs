use colored::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VitalSign {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub status: String,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize)]
struct VitalsReport {
    vitals: Vec<VitalSign>,
    overall_assessment: String,
    recommendations: Vec<String>,
}

/// Parse a vitals string like "hr=72 bp=120/80 temp=37.2 spo2=98 rr=16"
fn parse_vitals(input: &str) -> Vec<VitalSign> {
    let mut vitals = Vec::new();
    let parts: Vec<&str> = input.split([',', ' '])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for part in parts {
        if let Some((key, val)) = part.split_once('=') {
            let key_lower = key.to_lowercase();
            match key_lower.as_str() {
                "hr" | "heartrate" | "heart_rate" | "pulse" | "bpm" => {
                    if let Ok(v) = val.parse::<f64>() {
                        vitals.push(interpret_heart_rate(v));
                    }
                }
                "temp" | "temperature" | "t" => {
                    if let Ok(v) = val.parse::<f64>() {
                        vitals.push(interpret_temperature(v));
                    }
                }
                "spo2" | "o2" | "oxygen" | "sat" => {
                    if let Ok(v) = val.parse::<f64>() {
                        vitals.push(interpret_spo2(v));
                    }
                }
                "rr" | "resp" | "respiratory_rate" | "breathing" => {
                    if let Ok(v) = val.parse::<f64>() {
                        vitals.push(interpret_respiratory_rate(v));
                    }
                }
                "bp" | "blood_pressure" => {
                    if let Some((sys_s, dia_s)) = val.split_once('/') {
                        if let (Ok(sys), Ok(dia)) = (sys_s.parse::<f64>(), dia_s.parse::<f64>()) {
                            vitals.push(interpret_bp_systolic(sys));
                            vitals.push(interpret_bp_diastolic(dia));
                        }
                    }
                }
                "sys" | "systolic" => {
                    if let Ok(v) = val.parse::<f64>() {
                        vitals.push(interpret_bp_systolic(v));
                    }
                }
                "dia" | "diastolic" => {
                    if let Ok(v) = val.parse::<f64>() {
                        vitals.push(interpret_bp_diastolic(v));
                    }
                }
                _ => {} // ignore unknown keys
            }
        }
    }
    vitals
}

fn interpret_heart_rate(hr: f64) -> VitalSign {
    let (status, interp) = if hr < 40.0 {
        ("critical", "Severe bradycardia — seek emergency care")
    } else if hr < 60.0 {
        ("low", "Bradycardia — may be normal in athletes, otherwise evaluate")
    } else if hr <= 100.0 {
        ("normal", "Normal resting heart rate")
    } else if hr <= 120.0 {
        ("elevated", "Tachycardia — may indicate stress, fever, dehydration, or cardiac issue")
    } else {
        ("critical", "Severe tachycardia — seek medical attention immediately")
    };
    VitalSign {
        name: "Heart Rate".into(),
        value: hr,
        unit: "bpm".into(),
        status: status.into(),
        interpretation: interp.into(),
    }
}

fn interpret_temperature(temp: f64) -> VitalSign {
    // Assume Celsius; if > 45, probably Fahrenheit
    let temp_c = if temp > 45.0 { (temp - 32.0) * 5.0 / 9.0 } else { temp };
    let (status, interp) = if temp_c < 35.0 {
        ("critical", "Hypothermia — seek emergency care, warm gradually")
    } else if temp_c < 36.1 {
        ("low", "Below normal — monitor, ensure adequate warmth")
    } else if temp_c <= 37.2 {
        ("normal", "Normal body temperature")
    } else if temp_c <= 38.0 {
        ("elevated", "Low-grade fever — monitor, rest, hydrate")
    } else if temp_c <= 39.5 {
        ("high", "Fever — consider antipyretics, seek medical advice if persistent")
    } else {
        ("critical", "High fever — seek medical attention immediately, risk of febrile seizures")
    };
    VitalSign {
        name: "Temperature".into(),
        value: temp_c,
        unit: "°C".into(),
        status: status.into(),
        interpretation: interp.into(),
    }
}

fn interpret_spo2(spo2: f64) -> VitalSign {
    let (status, interp) = if spo2 >= 95.0 {
        ("normal", "Normal oxygen saturation")
    } else if spo2 >= 90.0 {
        ("low", "Below normal — may indicate respiratory compromise, seek evaluation")
    } else if spo2 >= 85.0 {
        ("critical", "Hypoxemia — supplemental oxygen needed, seek emergency care")
    } else {
        ("critical", "Severe hypoxemia — life-threatening, immediate emergency care required")
    };
    VitalSign {
        name: "SpO2".into(),
        value: spo2,
        unit: "%".into(),
        status: status.into(),
        interpretation: interp.into(),
    }
}

fn interpret_respiratory_rate(rr: f64) -> VitalSign {
    let (status, interp) = if rr < 8.0 {
        ("critical", "Dangerously low — possible respiratory depression, seek emergency care")
    } else if rr < 12.0 {
        ("low", "Below normal — monitor closely")
    } else if rr <= 20.0 {
        ("normal", "Normal respiratory rate")
    } else if rr <= 30.0 {
        ("elevated", "Tachypnea — may indicate infection, pain, anxiety, or respiratory issue")
    } else {
        ("critical", "Severe tachypnea — seek immediate medical attention")
    };
    VitalSign {
        name: "Respiratory Rate".into(),
        value: rr,
        unit: "breaths/min".into(),
        status: status.into(),
        interpretation: interp.into(),
    }
}

fn interpret_bp_systolic(sys: f64) -> VitalSign {
    let (status, interp) = if sys < 90.0 {
        ("critical", "Hypotension — risk of shock, seek medical care")
    } else if sys < 120.0 {
        ("normal", "Normal systolic blood pressure")
    } else if sys < 130.0 {
        ("elevated", "Elevated — lifestyle modifications recommended")
    } else if sys < 140.0 {
        ("high", "Stage 1 hypertension — medical evaluation recommended")
    } else if sys < 180.0 {
        ("high", "Stage 2 hypertension — needs treatment")
    } else {
        ("critical", "Hypertensive crisis — seek emergency care immediately")
    };
    VitalSign {
        name: "Systolic BP".into(),
        value: sys,
        unit: "mmHg".into(),
        status: status.into(),
        interpretation: interp.into(),
    }
}

fn interpret_bp_diastolic(dia: f64) -> VitalSign {
    let (status, interp) = if dia < 60.0 {
        ("low", "Low diastolic — may cause dizziness, evaluate if symptomatic")
    } else if dia < 80.0 {
        ("normal", "Normal diastolic blood pressure")
    } else if dia < 90.0 {
        ("high", "Stage 1 hypertension — medical evaluation recommended")
    } else if dia < 120.0 {
        ("high", "Stage 2 hypertension — needs treatment")
    } else {
        ("critical", "Hypertensive crisis — seek emergency care immediately")
    };
    VitalSign {
        name: "Diastolic BP".into(),
        value: dia,
        unit: "mmHg".into(),
        status: status.into(),
        interpretation: interp.into(),
    }
}

fn generate_recommendations(vitals: &[VitalSign]) -> Vec<String> {
    let mut recs = Vec::new();
    for v in vitals {
        match v.status.as_str() {
            "critical" => {
                recs.push(format!("🚨 {} is critical ({} {}) — {}", v.name, v.value, v.unit, v.interpretation));
            }
            "high" | "elevated" => {
                recs.push(format!("⚠️  {} is {} ({} {}) — {}", v.name, v.status, v.value, v.unit, v.interpretation));
            }
            "low" => {
                recs.push(format!("📉 {} is low ({} {}) — {}", v.name, v.value, v.unit, v.interpretation));
            }
            _ => {}
        }
    }
    if recs.is_empty() {
        recs.push("✅ All vitals within normal range.".to_string());
    }
    recs
}

fn overall_assessment(vitals: &[VitalSign]) -> String {
    let has_critical = vitals.iter().any(|v| v.status == "critical");
    let has_high = vitals.iter().any(|v| v.status == "high" || v.status == "elevated");
    let has_low = vitals.iter().any(|v| v.status == "low");

    if has_critical {
        "🔴 CRITICAL — One or more vital signs are dangerously abnormal. Seek emergency medical care immediately.".to_string()
    } else if has_high || has_low {
        "🟡 ABNORMAL — Some vital signs are outside normal range. Medical evaluation recommended.".to_string()
    } else {
        "🟢 NORMAL — All vital signs are within normal limits.".to_string()
    }
}

pub fn run(input: &str, json: bool) {
    let vitals = parse_vitals(input);

    if vitals.is_empty() {
        if json {
            println!("{{\"error\": \"No valid vitals parsed. Use format: hr=72 bp=120/80 temp=37.2 spo2=98 rr=16\"}}");
        } else {
            println!("{}", "No valid vital signs found.".yellow());
            println!();
            println!("Usage: openhealth vitals \"hr=72 bp=120/80 temp=37.2 spo2=98 rr=16\"");
            println!();
            println!("Supported keys:");
            println!("  hr, pulse, bpm    — Heart rate (beats per minute)");
            println!("  bp                — Blood pressure (sys/dia, e.g. 120/80)");
            println!("  temp, temperature — Temperature (°C or °F, auto-detected)");
            println!("  spo2, o2, sat     — Oxygen saturation (%)");
            println!("  rr, resp          — Respiratory rate (breaths/min)");
        }
        return;
    }

    let assessment = overall_assessment(&vitals);
    let recommendations = generate_recommendations(&vitals);

    if json {
        let report = VitalsReport {
            vitals,
            overall_assessment: assessment,
            recommendations,
        };
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string()));
        return;
    }

    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║            💓  VITAL SIGNS ASSESSMENT                    ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_cyan());
    println!();

    for v in &vitals {
        let status_display = match v.status.as_str() {
            "critical" => format!("[{}]", v.status).red().bold().to_string(),
            "high" => format!("[{}]", v.status).red().to_string(),
            "elevated" => format!("[{}]", v.status).yellow().to_string(),
            "low" => format!("[{}]", v.status).yellow().to_string(),
            _ => format!("[{}]", v.status).green().to_string(),
        };
        println!("  {} {} {} {}",
            v.name.bold(),
            format!("{:.1} {}", v.value, v.unit).bright_white(),
            status_display,
            v.interpretation.dimmed()
        );
    }

    println!();
    println!("{}", "━━━ Assessment ━━━".bold());
    println!("  {assessment}");
    println!();

    if recommendations.iter().any(|r| r.starts_with('🚨') || r.starts_with('⚠') || r.starts_with('📉')) {
        println!("{}", "━━━ Recommendations ━━━".bold());
        for rec in &recommendations {
            println!("  {rec}");
        }
        println!();
    }

    println!("{}", "⚠️  This is automated vital sign interpretation, NOT a medical diagnosis.".yellow());
    println!("{}", "   Normal ranges vary by age, sex, and medical condition.".yellow());
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heart_rate() {
        let vitals = parse_vitals("hr=72");
        assert_eq!(vitals.len(), 1);
        assert_eq!(vitals[0].name, "Heart Rate");
        assert_eq!(vitals[0].status, "normal");
    }

    #[test]
    fn test_parse_blood_pressure() {
        let vitals = parse_vitals("bp=120/80");
        assert_eq!(vitals.len(), 2);
        assert_eq!(vitals[0].name, "Systolic BP");
        assert_eq!(vitals[1].name, "Diastolic BP");
    }

    #[test]
    fn test_parse_multiple_vitals() {
        let vitals = parse_vitals("hr=72 bp=120/80 temp=37.0 spo2=98 rr=16");
        assert_eq!(vitals.len(), 6); // hr + sys + dia + temp + spo2 + rr
    }

    #[test]
    fn test_critical_heart_rate() {
        let vitals = parse_vitals("hr=150");
        assert_eq!(vitals[0].status, "critical");
    }

    #[test]
    fn test_normal_spo2() {
        let vitals = parse_vitals("spo2=98");
        assert_eq!(vitals[0].status, "normal");
    }

    #[test]
    fn test_low_spo2() {
        let vitals = parse_vitals("spo2=88");
        assert_eq!(vitals[0].status, "critical");
    }

    #[test]
    fn test_fahrenheit_auto_convert() {
        let vitals = parse_vitals("temp=98.6");
        assert_eq!(vitals[0].name, "Temperature");
        assert!((vitals[0].value - 37.0).abs() < 0.5); // ~37°C
        assert_eq!(vitals[0].status, "normal");
    }

    #[test]
    fn test_hypertensive_crisis() {
        let vitals = parse_vitals("bp=185/125");
        assert_eq!(vitals[0].status, "critical"); // systolic
        assert_eq!(vitals[1].status, "critical"); // diastolic
    }

    #[test]
    fn test_hypotension() {
        let vitals = parse_vitals("bp=85/55");
        assert_eq!(vitals[0].status, "critical"); // systolic
        assert_eq!(vitals[1].status, "low"); // diastolic
    }

    #[test]
    fn test_overall_normal() {
        let vitals = parse_vitals("hr=72 spo2=98 rr=16");
        let assessment = overall_assessment(&vitals);
        assert!(assessment.contains("NORMAL"));
    }

    #[test]
    fn test_overall_critical() {
        let vitals = parse_vitals("hr=160 spo2=85");
        let assessment = overall_assessment(&vitals);
        assert!(assessment.contains("CRITICAL"));
    }

    #[test]
    fn test_empty_input() {
        let vitals = parse_vitals("nothing here");
        assert!(vitals.is_empty());
    }

    #[test]
    fn test_respiratory_depression() {
        let vitals = parse_vitals("rr=6");
        assert_eq!(vitals[0].status, "critical");
    }

    #[test]
    fn test_recommendations_normal() {
        let vitals = parse_vitals("hr=72 spo2=98");
        let recs = generate_recommendations(&vitals);
        assert!(recs[0].contains("normal range"));
    }

    #[test]
    fn test_hypothermia() {
        let vitals = parse_vitals("temp=34.0");
        assert_eq!(vitals[0].status, "critical");
    }

    #[test]
    fn test_fever() {
        let vitals = parse_vitals("temp=39.0");
        assert_eq!(vitals[0].status, "high");
    }

    #[test]
    fn test_pulse_alias() {
        let vitals = parse_vitals("pulse=80");
        assert_eq!(vitals.len(), 1);
        assert_eq!(vitals[0].name, "Heart Rate");
    }
}
