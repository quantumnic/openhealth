use colored::Colorize;

struct FirstAidProtocol {
    situation: &'static str,
    keywords: &'static [&'static str],
    steps: &'static [&'static str],
    do_not: &'static [&'static str],
    call_emergency: bool,
    source: &'static str,
}

const PROTOCOLS: &[FirstAidProtocol] = &[
    FirstAidProtocol {
        situation: "Choking (Adult/Child >1 year)",
        keywords: &["choking", "choke", "airway", "can't breathe", "something stuck in throat"],
        steps: &[
            "Encourage coughing if partial obstruction",
            "Give 5 back blows between shoulder blades",
            "Give 5 abdominal thrusts (Heimlich maneuver)",
            "Alternate back blows and abdominal thrusts",
            "If unconscious: start CPR, check mouth before breaths",
        ],
        do_not: &["Do NOT perform blind finger sweeps", "Do NOT give water to drink"],
        call_emergency: true,
        source: "American Red Cross",
    },
    FirstAidProtocol {
        situation: "Choking (Infant <1 year)",
        keywords: &["baby choking", "infant choking"],
        steps: &[
            "Place infant face-down on your forearm, supporting head",
            "Give 5 back blows between shoulder blades with heel of hand",
            "Turn infant face-up, give 5 chest thrusts (2 fingers on breastbone)",
            "Repeat until object is expelled or infant becomes unconscious",
            "If unconscious: start infant CPR",
        ],
        do_not: &["Do NOT perform abdominal thrusts on infants", "Do NOT perform blind finger sweeps"],
        call_emergency: true,
        source: "American Heart Association",
    },
    FirstAidProtocol {
        situation: "CPR (Cardiopulmonary Resuscitation)",
        keywords: &["CPR", "not breathing", "no pulse", "cardiac arrest", "heart stopped", "unresponsive"],
        steps: &[
            "Check responsiveness — tap shoulders, shout",
            "Call emergency services (or have someone call)",
            "Place heel of hand on center of chest",
            "Push hard and fast: 100-120 compressions/minute, 5-6cm deep",
            "30 compressions, then 2 rescue breaths (if trained)",
            "Continue until help arrives or AED available",
        ],
        do_not: &["Do NOT stop compressions unless patient responds", "Do NOT delay — brain damage begins in 4 minutes"],
        call_emergency: true,
        source: "European Resuscitation Council",
    },
    FirstAidProtocol {
        situation: "Severe Bleeding",
        keywords: &["bleeding", "blood", "cut", "wound", "hemorrhage", "laceration"],
        steps: &[
            "Apply direct pressure with clean cloth or bandage",
            "Keep pressure firm and continuous — do not lift to check",
            "Elevate injured limb above heart level if possible",
            "If blood soaks through, add more material on top",
            "Apply tourniquet only if bleeding is life-threatening and won't stop (limbs only)",
        ],
        do_not: &["Do NOT remove embedded objects", "Do NOT use a tourniquet on neck or torso", "Do NOT apply tourniquet over joints"],
        call_emergency: true,
        source: "WHO First Aid Guidelines",
    },
    FirstAidProtocol {
        situation: "Burns",
        keywords: &["burn", "scald", "boiling water", "fire", "chemical burn"],
        steps: &[
            "Remove from heat source safely",
            "Cool burn under cool running water for 20 minutes",
            "Remove clothing/jewelry near burn (unless stuck)",
            "Cover loosely with cling film or clean non-fluffy dressing",
            "Give paracetamol/ibuprofen for pain",
        ],
        do_not: &["Do NOT use ice, butter, or toothpaste", "Do NOT burst blisters", "Do NOT remove clothing stuck to burn"],
        call_emergency: true,
        source: "British Red Cross",
    },
    FirstAidProtocol {
        situation: "Seizure / Convulsion",
        keywords: &["seizure", "convulsion", "epilepsy", "fit", "fitting"],
        steps: &[
            "Protect from injury — clear nearby hazards",
            "Place something soft under head",
            "Time the seizure",
            "When seizure stops, place in recovery position",
            "Stay with them until fully recovered",
        ],
        do_not: &["Do NOT restrain the person", "Do NOT put anything in their mouth", "Do NOT try to hold them down"],
        call_emergency: false,
        source: "Epilepsy Foundation",
    },
    FirstAidProtocol {
        situation: "Anaphylaxis (Severe Allergic Reaction)",
        keywords: &["anaphylaxis", "allergic reaction", "epipen", "swelling throat", "allergic shock"],
        steps: &[
            "Use adrenaline auto-injector (EpiPen) if available — inject into outer thigh",
            "Call emergency services immediately",
            "Lie patient down with legs raised (unless breathing difficulty — sit upright)",
            "Give second EpiPen after 5-15 minutes if no improvement",
            "Be prepared to start CPR if breathing stops",
        ],
        do_not: &["Do NOT delay giving adrenaline", "Do NOT make patient stand or walk", "Do NOT give antihistamines as sole treatment"],
        call_emergency: true,
        source: "World Allergy Organization",
    },
    FirstAidProtocol {
        situation: "Stroke Recognition (FAST)",
        keywords: &["stroke", "face drooping", "arm weakness", "speech difficulty", "FAST"],
        steps: &[
            "F — Face: Ask to smile. Does one side droop?",
            "A — Arms: Ask to raise both arms. Does one drift down?",
            "S — Speech: Ask to repeat a phrase. Is speech slurred?",
            "T — Time: Note time symptoms started, call emergency immediately",
            "Keep patient comfortable, do not give food or drink",
        ],
        do_not: &["Do NOT delay — every minute counts", "Do NOT give aspirin unless instructed", "Do NOT let patient fall asleep without monitoring"],
        call_emergency: true,
        source: "American Stroke Association",
    },
    FirstAidProtocol {
        situation: "Heatstroke",
        keywords: &["heatstroke", "heat stroke", "overheating", "heat exhaustion", "sunstroke"],
        steps: &[
            "Move to cool/shaded area immediately",
            "Remove excess clothing",
            "Cool rapidly: cold water immersion, ice packs to neck/armpits/groin",
            "Fan while misting with water",
            "Give cool water to drink if conscious and able to swallow",
        ],
        do_not: &["Do NOT give aspirin or paracetamol", "Do NOT use ice-cold water for drinking", "Do NOT leave unattended"],
        call_emergency: true,
        source: "WHO Heat Health Action Plans",
    },
    FirstAidProtocol {
        situation: "Fracture / Broken Bone",
        keywords: &["broken bone", "fracture", "broken arm", "broken leg", "bone sticking out"],
        steps: &[
            "Keep the injured area still — do not try to realign",
            "Support the limb in the position found",
            "Apply ice wrapped in cloth to reduce swelling",
            "Immobilize with splint if trained (padded rigid material)",
            "Monitor for shock (pale, cold, rapid pulse)",
        ],
        do_not: &["Do NOT move the injured part unnecessarily", "Do NOT straighten a deformed limb", "Do NOT apply ice directly to skin"],
        call_emergency: true,
        source: "American Academy of Orthopaedic Surgeons",
    },
    FirstAidProtocol {
        situation: "Poisoning / Overdose",
        keywords: &["poisoning", "overdose", "swallowed poison", "ingested", "toxic"],
        steps: &[
            "Call Poison Control or emergency services",
            "Identify the substance if possible (keep container)",
            "Note time of ingestion and estimated amount",
            "If unconscious: recovery position, monitor breathing",
            "Follow Poison Control instructions exactly",
        ],
        do_not: &["Do NOT induce vomiting unless instructed", "Do NOT give activated charcoal without medical advice", "Do NOT give anything by mouth if unconscious"],
        call_emergency: true,
        source: "WHO Guidelines on Poisoning Management",
    },
    FirstAidProtocol {
        situation: "Drowning / Near-Drowning",
        keywords: &["drowning", "underwater", "submersion", "near drowning", "pulled from water"],
        steps: &[
            "Remove from water safely (do not endanger yourself)",
            "Check for breathing and pulse",
            "If not breathing: start CPR immediately (begin with 5 rescue breaths)",
            "Continue CPR until help arrives",
            "Even if patient recovers, get medical evaluation (secondary drowning risk)",
        ],
        do_not: &["Do NOT attempt rescue beyond your ability", "Do NOT try to drain water from lungs", "Do NOT assume recovery — monitor closely"],
        call_emergency: true,
        source: "International Life Saving Federation",
    },
];

pub fn run(query: Option<&str>, json: bool) {
    let protocols: Vec<&FirstAidProtocol> = if let Some(q) = query {
        let q_lower = q.to_lowercase();
        PROTOCOLS
            .iter()
            .filter(|p| {
                p.situation.to_lowercase().contains(&q_lower)
                    || p.keywords.iter().any(|k| {
                        k.contains(&q_lower) || q_lower.contains(k)
                    })
            })
            .collect()
    } else {
        PROTOCOLS.iter().collect()
    };

    if json {
        let json_out: Vec<serde_json::Value> = protocols
            .iter()
            .map(|p| {
                serde_json::json!({
                    "situation": p.situation,
                    "steps": p.steps,
                    "do_not": p.do_not,
                    "call_emergency": p.call_emergency,
                    "source": p.source,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_out).unwrap());
        return;
    }

    if protocols.is_empty() {
        println!("\n  ⚠️ No first-aid protocol found for {:?}", query.unwrap_or(""));
        println!("  Run {} to see all available protocols.\n", "openhealth first-aid".bold());
        return;
    }

    println!("\n{}", "╔══════════════════════════════════════════════════╗".red());
    println!("{}", "║         🚑 First-Aid Quick Reference            ║".red());
    println!("{}", "╚══════════════════════════════════════════════════╝".red());

    if query.is_none() {
        println!("\n  📋 Available protocols ({}):\n", protocols.len());
        for p in &protocols {
            let emergency_marker = if p.call_emergency { "🔴" } else { "🟡" };
            println!("    {} {}", emergency_marker, p.situation);
        }
        println!("\n  Use {} for detailed steps.", "openhealth first-aid <situation>".bold());
        println!("  Example: {}\n", "openhealth first-aid choking".dimmed());
        return;
    }

    for p in &protocols {
        println!();
        if p.call_emergency {
            println!("  {} ", "🔴 CALL EMERGENCY SERVICES".red().bold());
        }
        println!("  🚑 {}", p.situation.bold().underline());
        println!();

        println!("  {}:", "Steps".green().bold());
        for (i, step) in p.steps.iter().enumerate() {
            println!("    {}. {}", i + 1, step);
        }
        println!();

        if !p.do_not.is_empty() {
            println!("  {}:", "⛔ Do NOT".red().bold());
            for item in p.do_not {
                println!("    {} {}", "✗".red(), item);
            }
            println!();
        }

        println!("  {}", format!("Source: {}", p.source).dimmed());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_aid_list_all() {
        assert!(PROTOCOLS.len() >= 10, "Should have at least 10 protocols");
    }

    #[test]
    fn test_first_aid_search_choking() {
        let q = "choking";
        let results: Vec<&FirstAidProtocol> = PROTOCOLS
            .iter()
            .filter(|p| {
                p.situation.to_lowercase().contains(q)
                    || p.keywords.iter().any(|k| k.contains(q))
            })
            .collect();
        assert!(!results.is_empty(), "Should find choking protocol");
        assert!(results.len() >= 2, "Should find adult and infant choking");
    }

    #[test]
    fn test_first_aid_search_cpr() {
        let q = "cpr";
        let results: Vec<&FirstAidProtocol> = PROTOCOLS
            .iter()
            .filter(|p| {
                p.situation.to_lowercase().contains(q)
                    || p.keywords.iter().any(|k| k.contains(q))
            })
            .collect();
        assert!(!results.is_empty(), "Should find CPR protocol");
    }

    #[test]
    fn test_first_aid_search_not_found() {
        let q = "xyzzynotaprotocol";
        let results: Vec<&FirstAidProtocol> = PROTOCOLS
            .iter()
            .filter(|p| {
                p.situation.to_lowercase().contains(q)
                    || p.keywords.iter().any(|k| k.contains(q))
            })
            .collect();
        assert!(results.is_empty());
    }

    #[test]
    fn test_first_aid_all_have_steps() {
        for p in PROTOCOLS {
            assert!(!p.steps.is_empty(), "{} should have steps", p.situation);
            assert!(!p.source.is_empty(), "{} should have source", p.situation);
        }
    }

    #[test]
    fn test_first_aid_search_burn() {
        let q = "burn";
        let results: Vec<&FirstAidProtocol> = PROTOCOLS
            .iter()
            .filter(|p| {
                p.situation.to_lowercase().contains(q)
                    || p.keywords.iter().any(|k| k.contains(q))
            })
            .collect();
        assert!(!results.is_empty(), "Should find burn protocol");
    }

    #[test]
    fn test_first_aid_run_json() {
        run(Some("choking"), true);
    }

    #[test]
    fn test_first_aid_run_no_query() {
        run(None, false);
    }
}
