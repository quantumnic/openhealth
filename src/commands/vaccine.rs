use colored::Colorize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Vaccine {
    name: &'static str,
    abbreviation: &'static str,
    prevents: &'static str,
    schedule: &'static str,
    age_group: &'static str,
    doses: u8,
    booster: &'static str,
    contraindications: &'static str,
    side_effects: &'static str,
    source: &'static str,
}

fn get_vaccines() -> Vec<Vaccine> {
    vec![
        Vaccine {
            name: "BCG (Bacillus Calmette-Guérin)",
            abbreviation: "BCG",
            prevents: "Tuberculosis (severe forms: TB meningitis, miliary TB)",
            schedule: "Birth or as soon as possible after birth",
            age_group: "neonates",
            doses: 1,
            booster: "Not routinely recommended",
            contraindications: "HIV-positive infants with symptoms, immunodeficiency, low birth weight (<2000g)",
            side_effects: "Local ulcer at injection site, lymphadenopathy, rare disseminated BCG",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Hepatitis B",
            abbreviation: "HepB",
            prevents: "Hepatitis B virus infection, cirrhosis, hepatocellular carcinoma",
            schedule: "Birth dose within 24h, then at 6 weeks, 10 weeks, 14 weeks",
            age_group: "neonates",
            doses: 3,
            booster: "Not routinely needed; check anti-HBs in high-risk adults",
            contraindications: "Severe allergic reaction to previous dose or yeast",
            side_effects: "Injection site soreness, mild fever, fatigue",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Oral Polio Vaccine",
            abbreviation: "OPV",
            prevents: "Poliomyelitis (all 3 serotypes)",
            schedule: "Birth, 6 weeks, 10 weeks, 14 weeks + boosters",
            age_group: "infants",
            doses: 4,
            booster: "At 18 months and school entry",
            contraindications: "Immunodeficiency (use IPV instead), HIV-positive with severe immunosuppression",
            side_effects: "Very rare vaccine-associated paralytic polio (1 in 2.7 million doses)",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Inactivated Polio Vaccine",
            abbreviation: "IPV",
            prevents: "Poliomyelitis",
            schedule: "14 weeks (with or replacing 3rd OPV dose)",
            age_group: "infants",
            doses: 1,
            booster: "As part of combination vaccines at 18 months and 4-6 years",
            contraindications: "Severe allergic reaction to previous dose, neomycin, streptomycin, polymyxin B",
            side_effects: "Injection site redness, mild fever",
            source: "WHO EPI",
        },
        Vaccine {
            name: "DTP (Diphtheria-Tetanus-Pertussis)",
            abbreviation: "DTP/DTaP",
            prevents: "Diphtheria, tetanus, pertussis (whooping cough)",
            schedule: "6 weeks, 10 weeks, 14 weeks",
            age_group: "infants",
            doses: 3,
            booster: "18 months, 4-6 years (DTP); Td/Tdap every 10 years for adults",
            contraindications: "Encephalopathy within 7 days of previous dose, progressive neurological disorder",
            side_effects: "Fever, injection site swelling, irritability, rare febrile seizures",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Measles-Mumps-Rubella",
            abbreviation: "MMR",
            prevents: "Measles, mumps, rubella (German measles)",
            schedule: "9-12 months (MCV1), 15-18 months (MCV2)",
            age_group: "infants",
            doses: 2,
            booster: "None routinely; catch-up for unvaccinated adults",
            contraindications: "Pregnancy, severe immunodeficiency, recent blood product, anaphylaxis to neomycin/gelatin",
            side_effects: "Fever, mild rash 7-12 days post-vaccine, transient joint pain, very rare febrile seizures",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Pneumococcal Conjugate Vaccine",
            abbreviation: "PCV",
            prevents: "Pneumococcal pneumonia, meningitis, sepsis",
            schedule: "6 weeks, 10 weeks, 14 weeks (or 2+1 schedule)",
            age_group: "infants",
            doses: 3,
            booster: "9-15 months; PPSV23 for adults 65+ and high-risk",
            contraindications: "Severe allergic reaction to previous dose or diphtheria toxoid",
            side_effects: "Injection site reactions, mild fever, irritability",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Rotavirus",
            abbreviation: "RV",
            prevents: "Rotavirus gastroenteritis (severe diarrhea in infants)",
            schedule: "6 weeks, 10 weeks (RV1: 2 doses) or +14 weeks (RV5: 3 doses)",
            age_group: "infants",
            doses: 2,
            booster: "None",
            contraindications: "History of intussusception, SCID, uncorrected congenital GI malformation",
            side_effects: "Mild diarrhea, vomiting, irritability, very rare intussusception (1-2 per 100,000)",
            source: "WHO EPI",
        },
        Vaccine {
            name: "Human Papillomavirus",
            abbreviation: "HPV",
            prevents: "HPV types 16/18 (cervical, anal, oropharyngeal cancers), types 6/11 (genital warts)",
            schedule: "9-14 years (1-2 doses); 15+ years (3 doses over 6 months)",
            age_group: "children",
            doses: 2,
            booster: "None currently recommended",
            contraindications: "Pregnancy (defer), severe allergic reaction to previous dose or yeast",
            side_effects: "Injection site pain, mild fever, dizziness, syncope (observe 15 min post-vaccine)",
            source: "WHO",
        },
        Vaccine {
            name: "Influenza (Seasonal)",
            abbreviation: "Flu",
            prevents: "Seasonal influenza (A and B strains)",
            schedule: "Annually, ideally before flu season (October-November in Northern Hemisphere)",
            age_group: "all",
            doses: 1,
            booster: "Annual revaccination (strain changes yearly)",
            contraindications: "Severe egg allergy (for egg-based; cell-based/recombinant available), GBS within 6 weeks of prior flu vaccine",
            side_effects: "Injection site soreness, low-grade fever, myalgia for 1-2 days",
            source: "WHO",
        },
        Vaccine {
            name: "Yellow Fever",
            abbreviation: "YF",
            prevents: "Yellow fever",
            schedule: "9-12 months in endemic areas; before travel to endemic areas",
            age_group: "all",
            doses: 1,
            booster: "Single dose provides lifelong immunity (WHO 2013)",
            contraindications: "Age <6 months, severe immunodeficiency, thymus disorder, severe egg allergy, pregnancy (relative)",
            side_effects: "Mild headache, myalgia, low fever; very rare viscerotropic/neurotropic disease",
            source: "WHO",
        },
        Vaccine {
            name: "COVID-19 (mRNA / Viral Vector)",
            abbreviation: "COVID",
            prevents: "SARS-CoV-2 infection, severe disease, hospitalization, death",
            schedule: "Primary series: 2 doses 3-8 weeks apart (mRNA); updated boosters per current guidance",
            age_group: "all",
            doses: 2,
            booster: "Updated boosters recommended annually, especially for high-risk groups",
            contraindications: "Severe allergic reaction to previous dose or PEG/polysorbate components",
            side_effects: "Injection site pain, fatigue, headache, myalgia, fever, rare myocarditis (young males, mRNA)",
            source: "WHO",
        },
        Vaccine {
            name: "Tetanus-Diphtheria (Adult)",
            abbreviation: "Td/Tdap",
            prevents: "Tetanus, diphtheria; Tdap also prevents pertussis",
            schedule: "Tdap once for adults, then Td every 10 years; Tdap in each pregnancy (27-36 weeks)",
            age_group: "adults",
            doses: 1,
            booster: "Every 10 years; wound management: Td if >5 years since last dose",
            contraindications: "Encephalopathy after previous pertussis vaccine (Tdap); severe allergic reaction",
            side_effects: "Injection site pain, mild fever, headache, fatigue",
            source: "WHO",
        },
        Vaccine {
            name: "Meningococcal Conjugate",
            abbreviation: "MenACWY",
            prevents: "Meningococcal meningitis and septicemia (serogroups A, C, W, Y)",
            schedule: "9-12 months (endemic areas); 11-12 years with booster at 16 (US)",
            age_group: "children",
            doses: 2,
            booster: "At 16 years; additional doses for high-risk (asplenia, complement deficiency, travel)",
            contraindications: "Severe allergic reaction to previous dose or vaccine component",
            side_effects: "Injection site reactions, mild fever, headache, fatigue",
            source: "WHO",
        },
        Vaccine {
            name: "Varicella (Chickenpox)",
            abbreviation: "VAR",
            prevents: "Varicella (chickenpox) and reduces risk of shingles later in life",
            schedule: "12-15 months, 4-6 years",
            age_group: "children",
            doses: 2,
            booster: "Catch-up for susceptible adults (2 doses, 4-8 weeks apart)",
            contraindications: "Pregnancy, severe immunodeficiency, recent blood products, anaphylaxis to gelatin/neomycin",
            side_effects: "Injection site reactions, mild fever, mild varicella-like rash (5%)",
            source: "WHO",
        },
        Vaccine {
            name: "Hepatitis A",
            abbreviation: "HepA",
            prevents: "Hepatitis A virus infection",
            schedule: "12-23 months (2 doses, 6 months apart); travelers to endemic areas",
            age_group: "children",
            doses: 2,
            booster: "Not routinely needed after 2-dose series",
            contraindications: "Severe allergic reaction to previous dose or vaccine component",
            side_effects: "Injection site soreness, headache, fatigue, mild fever",
            source: "WHO",
        },
        Vaccine {
            name: "Shingles (Recombinant Zoster)",
            abbreviation: "RZV",
            prevents: "Herpes zoster (shingles) and postherpetic neuralgia",
            schedule: "50 years and older (2 doses, 2-6 months apart)",
            age_group: "elderly",
            doses: 2,
            booster: "None currently; 2-dose series provides long-lasting protection",
            contraindications: "Severe allergic reaction to vaccine component, active shingles episode (defer)",
            side_effects: "Injection site pain/swelling (common), fatigue, myalgia, headache, fever, GI symptoms",
            source: "WHO/CDC",
        },
        Vaccine {
            name: "Typhoid",
            abbreviation: "TCV/Vi",
            prevents: "Typhoid fever (Salmonella typhi)",
            schedule: "6 months+ (TCV conjugate) or 2+ years (Vi polysaccharide); travelers to endemic areas",
            age_group: "all",
            doses: 1,
            booster: "Vi polysaccharide: every 3 years if ongoing risk. TCV: not yet established.",
            contraindications: "Severe allergic reaction to previous dose",
            side_effects: "Injection site pain, mild fever, headache",
            source: "WHO",
        },
        Vaccine {
            name: "Rabies (Pre-Exposure)",
            abbreviation: "RabV",
            prevents: "Rabies virus infection",
            schedule: "Days 0, 7 (2-dose intradermal or intramuscular); or 3 doses for high-risk",
            age_group: "all",
            doses: 2,
            booster: "Based on antibody titer for ongoing risk (veterinarians, lab workers); post-exposure: 1-2 booster doses",
            contraindications: "None absolute for post-exposure; pre-exposure: severe allergic reaction to previous dose",
            side_effects: "Injection site pain, headache, nausea, dizziness, rare allergic reactions",
            source: "WHO",
        },
    ]
}

pub fn run(age_group: Option<&str>, name: Option<&str>, json: bool) {
    let vaccines = get_vaccines();

    let filtered: Vec<&Vaccine> = if let Some(search) = name {
        let search_lower = search.to_lowercase();
        vaccines.iter().filter(|v| {
            v.name.to_lowercase().contains(&search_lower)
                || v.abbreviation.to_lowercase().contains(&search_lower)
                || v.prevents.to_lowercase().contains(&search_lower)
        }).collect()
    } else if let Some(ag) = age_group {
        let ag_lower = ag.to_lowercase();
        vaccines.iter().filter(|v| v.age_group.to_lowercase() == ag_lower || v.age_group == "all").collect()
    } else {
        vaccines.iter().collect()
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
        return;
    }

    if filtered.is_empty() {
        println!("{}", "No vaccines found matching your query.".yellow());
        return;
    }

    println!("{}", "═══════════════════════════════════════════════".blue().bold());
    println!("{}", " 💉  VACCINATION REFERENCE".blue().bold());
    println!("{}", "═══════════════════════════════════════════════".blue().bold());
    println!();

    for v in &filtered {
        println!("{} ({})", v.name.green().bold(), v.abbreviation.cyan());
        println!("  {} {}", "Prevents:".white().bold(), v.prevents);
        println!("  {} {}", "Schedule:".white().bold(), v.schedule);
        println!("  {} {} dose(s)", "Doses:".white().bold(), v.doses);
        println!("  {} {}", "Booster:".white().bold(), v.booster);
        println!("  {} {}", "Age group:".white().bold(), v.age_group);
        println!("  {} {}", "Side effects:".white().bold(), v.side_effects);
        println!("  {} {}", "Contraindications:".yellow().bold(), v.contraindications);
        println!("  {} {}", "Source:".dimmed(), v.source);
        println!();
    }

    println!("{}", "───────────────────────────────────────────────".dimmed());
    println!("{}", "⚠  This is a reference guide. Always consult local immunization schedules".yellow());
    println!("{}", "   and a healthcare provider for personalized recommendations.".yellow());
    println!("{}", format!("   {} vaccines in database", filtered.len()).dimmed());
}
