use colored::*;
use serde::Serialize;

#[derive(Serialize)]
struct DangerSignsResult {
    category: String,
    title: String,
    signs: Vec<DangerSign>,
    action: String,
}

#[derive(Serialize)]
struct DangerSign {
    sign: String,
    urgency: String,
    explanation: String,
}

/// WHO IMCI-based danger signs for children, maternal/obstetric, and neonatal emergencies.
pub fn run(category: Option<&str>, json: bool) {
    let categories = match category {
        Some(c) => vec![c.to_lowercase()],
        None => vec![
            "child".to_string(),
            "maternal".to_string(),
            "neonatal".to_string(),
        ],
    };

    let mut results: Vec<DangerSignsResult> = Vec::new();

    for cat in &categories {
        match cat.as_str() {
            "child" | "children" | "pediatric" => results.push(child_danger_signs()),
            "maternal" | "pregnancy" | "obstetric" => results.push(maternal_danger_signs()),
            "neonatal" | "newborn" | "neonate" => results.push(neonatal_danger_signs()),
            "adult" | "general" => results.push(adult_danger_signs()),
            _ => {
                if !json {
                    println!("Unknown category: '{}'. Use: child, maternal, neonatal, adult", cat);
                }
            }
        }
    }

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&results).unwrap_or_else(|_| "[]".to_string())
        );
        return;
    }

    for result in &results {
        println!();
        println!(
            "{}",
            "╔══════════════════════════════════════════════════════════╗".bright_red()
        );
        println!(
            "║  ⚠️   {}",
            format!("{:<52}║", result.title).bright_red()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════════════════════╝".bright_red()
        );
        println!();
        println!("  Category: {}", result.category.bright_white());
        println!("  Source: WHO IMCI / WHO Maternal Health Guidelines");
        println!();

        for sign in &result.signs {
            let icon = match sign.urgency.as_str() {
                "critical" => "🔴",
                "urgent" => "🟡",
                _ => "🟢",
            };
            println!(
                "  {} {} {}",
                icon,
                sign.sign.bold(),
                format!("[{}]", sign.urgency).dimmed()
            );
            println!("    {}", sign.explanation.dimmed());
            println!();
        }

        println!("  {}", "━━━ Recommended Action ━━━".bold());
        println!("  {}", result.action);
        println!();
        println!(
            "  {}",
            "⚠️  Any danger sign = SEEK IMMEDIATE MEDICAL CARE".red().bold()
        );
    }
    println!();
}

fn child_danger_signs() -> DangerSignsResult {
    DangerSignsResult {
        category: "Pediatric (2 months — 5 years)".into(),
        title: "WHO IMCI DANGER SIGNS — CHILDREN".into(),
        signs: vec![
            DangerSign {
                sign: "Unable to drink or breastfeed".into(),
                urgency: "critical".into(),
                explanation: "Child refuses all fluids — risk of severe dehydration and shock.".into(),
            },
            DangerSign {
                sign: "Vomits everything".into(),
                urgency: "critical".into(),
                explanation: "Cannot retain any fluids — oral rehydration impossible.".into(),
            },
            DangerSign {
                sign: "Convulsions / seizures".into(),
                urgency: "critical".into(),
                explanation: "May indicate cerebral malaria, meningitis, or severe infection.".into(),
            },
            DangerSign {
                sign: "Lethargic or unconscious".into(),
                urgency: "critical".into(),
                explanation: "Reduced consciousness — sign of severe systemic illness.".into(),
            },
            DangerSign {
                sign: "Chest indrawing (subcostal retraction)".into(),
                urgency: "urgent".into(),
                explanation: "Sign of severe pneumonia — child is working hard to breathe.".into(),
            },
            DangerSign {
                sign: "Stridor when calm".into(),
                urgency: "urgent".into(),
                explanation: "Upper airway obstruction — risk of complete obstruction.".into(),
            },
            DangerSign {
                sign: "Severe malnutrition (visible wasting)".into(),
                urgency: "urgent".into(),
                explanation: "Marasmus or kwashiorkor — immune system severely compromised.".into(),
            },
            DangerSign {
                sign: "Bulging fontanelle".into(),
                urgency: "critical".into(),
                explanation: "In infants: raised intracranial pressure — possible meningitis.".into(),
            },
            DangerSign {
                sign: "Sunken eyes + skin pinch goes back very slowly".into(),
                urgency: "urgent".into(),
                explanation: "Signs of severe dehydration — needs IV fluids urgently.".into(),
            },
        ],
        action: "🚨 Any ONE danger sign = refer urgently to hospital. Give first dose of antibiotic, antimalarial, or ORS as appropriate before transfer.".into(),
    }
}

fn maternal_danger_signs() -> DangerSignsResult {
    DangerSignsResult {
        category: "Maternal / Obstetric".into(),
        title: "WHO DANGER SIGNS — PREGNANCY & CHILDBIRTH".into(),
        signs: vec![
            DangerSign {
                sign: "Severe vaginal bleeding".into(),
                urgency: "critical".into(),
                explanation: "Placenta previa, abruption, or postpartum hemorrhage — life-threatening.".into(),
            },
            DangerSign {
                sign: "Severe headache with blurred vision".into(),
                urgency: "critical".into(),
                explanation: "Preeclampsia/eclampsia — risk of seizures and organ damage.".into(),
            },
            DangerSign {
                sign: "High fever (>38°C)".into(),
                urgency: "urgent".into(),
                explanation: "Infection (puerperal sepsis, chorioamnionitis) — needs antibiotics.".into(),
            },
            DangerSign {
                sign: "Convulsions / fits".into(),
                urgency: "critical".into(),
                explanation: "Eclampsia — give magnesium sulfate and refer immediately.".into(),
            },
            DangerSign {
                sign: "Swollen hands/face + headache".into(),
                urgency: "urgent".into(),
                explanation: "Preeclampsia — check blood pressure urgently.".into(),
            },
            DangerSign {
                sign: "Foul-smelling vaginal discharge".into(),
                urgency: "urgent".into(),
                explanation: "Postpartum infection — needs antibiotics urgently.".into(),
            },
            DangerSign {
                sign: "Baby not moving (after 28 weeks)".into(),
                urgency: "urgent".into(),
                explanation: "Reduced fetal movement — may indicate fetal distress.".into(),
            },
            DangerSign {
                sign: "Water breaks before 37 weeks".into(),
                urgency: "urgent".into(),
                explanation: "Preterm premature rupture of membranes — risk of preterm labor and infection.".into(),
            },
        ],
        action: "🚨 Any ONE danger sign = go to hospital/health facility IMMEDIATELY. In pregnancy: lie on left side during transport. Postpartum: keep warm, elevate legs if bleeding.".into(),
    }
}

fn neonatal_danger_signs() -> DangerSignsResult {
    DangerSignsResult {
        category: "Neonatal (0 — 2 months)".into(),
        title: "WHO DANGER SIGNS — NEWBORNS".into(),
        signs: vec![
            DangerSign {
                sign: "Not feeding well / unable to suckle".into(),
                urgency: "critical".into(),
                explanation: "Poor feeding in newborns is often the first sign of serious infection.".into(),
            },
            DangerSign {
                sign: "Convulsions".into(),
                urgency: "critical".into(),
                explanation: "Neonatal seizures — may indicate meningitis, hypoglycemia, or birth injury.".into(),
            },
            DangerSign {
                sign: "Fast breathing (≥60/min)".into(),
                urgency: "urgent".into(),
                explanation: "Tachypnea — possible neonatal pneumonia or sepsis.".into(),
            },
            DangerSign {
                sign: "Severe chest indrawing".into(),
                urgency: "critical".into(),
                explanation: "Respiratory distress — needs oxygen and urgent care.".into(),
            },
            DangerSign {
                sign: "Fever (>37.5°C) or hypothermia (<35.5°C)".into(),
                urgency: "urgent".into(),
                explanation: "Temperature instability in newborns suggests serious infection.".into(),
            },
            DangerSign {
                sign: "Skin pustules or umbilical redness spreading".into(),
                urgency: "urgent".into(),
                explanation: "Skin/umbilical infection — can progress to sepsis rapidly.".into(),
            },
            DangerSign {
                sign: "Jaundice in first 24 hours or spreading to palms/soles".into(),
                urgency: "critical".into(),
                explanation: "Pathological jaundice — risk of kernicterus (brain damage).".into(),
            },
            DangerSign {
                sign: "Lethargic / floppy / no movement".into(),
                urgency: "critical".into(),
                explanation: "Severely ill newborn — immediate referral needed.".into(),
            },
        ],
        action: "🚨 Any ONE danger sign in a newborn = REFER IMMEDIATELY. Keep baby warm (skin-to-skin). Give first dose of antibiotics if available. Express breastmilk if baby cannot suckle.".into(),
    }
}

fn adult_danger_signs() -> DangerSignsResult {
    DangerSignsResult {
        category: "Adult / General".into(),
        title: "DANGER SIGNS — ADULTS".into(),
        signs: vec![
            DangerSign {
                sign: "Chest pain or pressure".into(),
                urgency: "critical".into(),
                explanation: "Possible heart attack, aortic dissection, or pulmonary embolism.".into(),
            },
            DangerSign {
                sign: "Sudden facial drooping, arm weakness, speech difficulty".into(),
                urgency: "critical".into(),
                explanation: "FAST signs of stroke — every minute counts.".into(),
            },
            DangerSign {
                sign: "Difficulty breathing at rest".into(),
                urgency: "critical".into(),
                explanation: "Respiratory failure — needs immediate oxygen and assessment.".into(),
            },
            DangerSign {
                sign: "Severe allergic reaction (swollen tongue/throat)".into(),
                urgency: "critical".into(),
                explanation: "Anaphylaxis — give epinephrine immediately.".into(),
            },
            DangerSign {
                sign: "Sudden severe headache (worst of life)".into(),
                urgency: "critical".into(),
                explanation: "Thunderclap headache — possible subarachnoid hemorrhage.".into(),
            },
            DangerSign {
                sign: "Coughing or vomiting blood".into(),
                urgency: "critical".into(),
                explanation: "Internal bleeding — needs urgent investigation.".into(),
            },
            DangerSign {
                sign: "Confusion or altered consciousness".into(),
                urgency: "critical".into(),
                explanation: "May indicate stroke, sepsis, metabolic crisis, or poisoning.".into(),
            },
            DangerSign {
                sign: "Suicidal thoughts or self-harm intent".into(),
                urgency: "critical".into(),
                explanation: "Mental health emergency — seek immediate help. Crisis line or emergency services.".into(),
            },
        ],
        action: "🚨 Any ONE danger sign = call emergency services or go to emergency department. Do not drive yourself. Time is critical.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_danger_signs_child() {
        run(Some("child"), false);
    }

    #[test]
    fn test_danger_signs_maternal() {
        run(Some("maternal"), false);
    }

    #[test]
    fn test_danger_signs_neonatal() {
        run(Some("neonatal"), false);
    }

    #[test]
    fn test_danger_signs_adult() {
        run(Some("adult"), false);
    }

    #[test]
    fn test_danger_signs_all() {
        run(None, false);
    }

    #[test]
    fn test_danger_signs_json() {
        run(Some("child"), true);
    }

    #[test]
    fn test_child_has_signs() {
        let result = child_danger_signs();
        assert!(result.signs.len() >= 8);
    }

    #[test]
    fn test_maternal_has_signs() {
        let result = maternal_danger_signs();
        assert!(result.signs.len() >= 7);
    }

    #[test]
    fn test_neonatal_has_signs() {
        let result = neonatal_danger_signs();
        assert!(result.signs.len() >= 7);
    }
}
