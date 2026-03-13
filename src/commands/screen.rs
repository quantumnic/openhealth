use colored::*;

#[derive(Debug, Clone)]
struct Screening {
    name: &'static str,
    test: &'static str,
    start_age: u8,
    end_age: u8,
    frequency: &'static str,
    sex: Option<&'static str>, // None = both, Some("male") or Some("female")
    source: &'static str,
    notes: &'static str,
}

fn get_screenings() -> Vec<Screening> {
    vec![
        Screening { name: "Blood Pressure", test: "Sphygmomanometry", start_age: 18, end_age: 120, frequency: "Every 1-2 years", sex: None, source: "WHO/AHA", notes: "More frequent if elevated or risk factors present" },
        Screening { name: "Cholesterol / Lipid Panel", test: "Fasting lipid profile", start_age: 20, end_age: 120, frequency: "Every 4-6 years", sex: None, source: "AHA/ACC", notes: "More frequent if on statins or high risk. Start at 9-11 for family history." },
        Screening { name: "Diabetes (Type 2)", test: "Fasting glucose or HbA1c", start_age: 35, end_age: 120, frequency: "Every 3 years", sex: None, source: "USPSTF", notes: "Screen earlier if overweight/obese with risk factors. BMI ≥25 triggers earlier screening." },
        Screening { name: "Colorectal Cancer", test: "Colonoscopy / FIT / Cologuard", start_age: 45, end_age: 75, frequency: "Colonoscopy every 10 years, FIT annually", sex: None, source: "USPSTF/ACS", notes: "Start at 40 or earlier with family history. Most cost-effective cancer screening." },
        Screening { name: "Breast Cancer", test: "Mammography", start_age: 40, end_age: 74, frequency: "Every 1-2 years", sex: Some("female"), source: "USPSTF/ACS", notes: "Earlier and with MRI for BRCA carriers or high-risk. Discuss with provider at 40." },
        Screening { name: "Cervical Cancer", test: "Pap smear / HPV co-test", start_age: 21, end_age: 65, frequency: "Pap every 3 years (21-29), Pap+HPV every 5 years (30-65)", sex: Some("female"), source: "USPSTF", notes: "Can stop at 65 if adequate prior screening. HPV vaccination reduces risk." },
        Screening { name: "Prostate Cancer", test: "PSA blood test ± DRE", start_age: 50, end_age: 70, frequency: "Discuss with provider; individualized decision", sex: Some("male"), source: "USPSTF/AUA", notes: "Start at 45 for African ancestry or family history. Shared decision-making recommended." },
        Screening { name: "Lung Cancer", test: "Low-dose CT scan", start_age: 50, end_age: 80, frequency: "Annually", sex: None, source: "USPSTF", notes: "Only for adults with ≥20 pack-year smoking history who currently smoke or quit within 15 years." },
        Screening { name: "Osteoporosis", test: "DEXA bone density scan", start_age: 65, end_age: 120, frequency: "Every 2-5 years", sex: Some("female"), source: "USPSTF", notes: "Men at 70. Earlier for postmenopausal women with risk factors. T-score ≤-2.5 = osteoporosis." },
        Screening { name: "Abdominal Aortic Aneurysm", test: "Abdominal ultrasound", start_age: 65, end_age: 75, frequency: "One-time screening", sex: Some("male"), source: "USPSTF", notes: "For men who have ever smoked. Selective screening for women with risk factors." },
        Screening { name: "Hepatitis C", test: "HCV antibody test", start_age: 18, end_age: 79, frequency: "One-time screening", sex: None, source: "USPSTF", notes: "All adults 18-79. Additional testing for ongoing risk factors (injection drug use)." },
        Screening { name: "Hepatitis B", test: "HBsAg, anti-HBs, anti-HBc", start_age: 18, end_age: 120, frequency: "One-time screening", sex: None, source: "USPSTF", notes: "Screen all adolescents and adults. Vaccinate if susceptible." },
        Screening { name: "HIV", test: "HIV antigen/antibody test", start_age: 15, end_age: 65, frequency: "At least once; annually if high risk", sex: None, source: "USPSTF", notes: "All adolescents and adults 15-65. Pregnant women at each pregnancy." },
        Screening { name: "Depression", test: "PHQ-9 questionnaire", start_age: 12, end_age: 120, frequency: "Annually or at wellness visits", sex: None, source: "USPSTF", notes: "Screen when adequate systems in place for diagnosis, treatment, and follow-up." },
        Screening { name: "Vision / Eye Exam", test: "Comprehensive eye exam", start_age: 40, end_age: 120, frequency: "Every 2-4 years (40-54), every 1-3 years (55-64), every 1-2 years (65+)", sex: None, source: "AAO", notes: "Earlier for diabetes, family history of glaucoma, or African ancestry." },
        Screening { name: "Skin Cancer", test: "Full-body skin exam", start_age: 18, end_age: 120, frequency: "Annual self-exam; provider exam based on risk", sex: None, source: "AAD", notes: "Higher frequency for fair skin, many moles, family history, or prior skin cancer." },
        Screening { name: "Dental Health", test: "Dental exam + cleaning", start_age: 1, end_age: 120, frequency: "Every 6-12 months", sex: None, source: "ADA", notes: "Includes oral cancer screening. Essential for overall health." },
    ]
}

pub fn run(age: Option<u8>, sex: Option<&str>, json: bool) {
    let screenings = get_screenings();
    let age_val = age.unwrap_or(0);
    let sex_val = sex.map(|s| s.to_lowercase());

    let filtered: Vec<&Screening> = screenings.iter().filter(|s| {
        let age_ok = age_val == 0 || (age_val >= s.start_age && age_val <= s.end_age);
        let sex_ok = match (&sex_val, s.sex) {
            (Some(user_sex), Some(screen_sex)) => user_sex == screen_sex,
            _ => true,
        };
        age_ok && sex_ok
    }).collect();

    if json {
        print_json(&filtered, age, sex);
        return;
    }

    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║        🏥  HEALTH SCREENING RECOMMENDATIONS                 ║".bright_cyan().bold());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();

    if let Some(a) = age {
        print!("  👤 Age: {}", a.to_string().bright_white().bold());
        if let Some(s) = sex {
            print!("  |  Sex: {}", s.bright_white().bold());
        }
        println!();
        println!();
    }

    if filtered.is_empty() {
        println!("  No applicable screenings found for the given criteria.");
        return;
    }

    println!("  {} recommended screenings:\n", filtered.len().to_string().bright_yellow().bold());

    for (i, s) in filtered.iter().enumerate() {
        let priority = if age_val > 0 {
            if age_val >= s.start_age && age_val <= s.end_age { "✅" } else { "⏳" }
        } else {
            "📋"
        };

        println!("  {} {} {}", priority, format!("{}.", i + 1).dimmed(), s.name.bright_white().bold());
        println!("     {} {}", "Test:".dimmed(), s.test);
        println!("     {} Ages {}-{}", "When:".dimmed(), s.start_age, if s.end_age >= 120 { "ongoing".to_string() } else { s.end_age.to_string() });
        println!("     {} {}", "How often:".dimmed(), s.frequency.bright_yellow());
        if let Some(sex_req) = s.sex {
            println!("     {} {}", "For:".dimmed(), sex_req.bright_magenta());
        }
        println!("     {} {}", "Source:".dimmed(), s.source.bright_cyan());
        println!("     ℹ️  {}", s.notes.dimmed());
        println!();
    }

    println!("{}", "  ⚠️  These are general guidelines. Consult your healthcare".yellow());
    println!("{}", "     provider for personalized screening recommendations.".yellow());
    println!();
    println!("  {}", "Source: USPSTF, WHO, ACS, AAO, ADA guidelines".dimmed());
}

fn print_json(screenings: &[&Screening], age: Option<u8>, sex: Option<&str>) {
    let items: Vec<serde_json::Value> = screenings.iter().map(|s| {
        serde_json::json!({
            "name": s.name,
            "test": s.test,
            "start_age": s.start_age,
            "end_age": if s.end_age >= 120 { None } else { Some(s.end_age) },
            "frequency": s.frequency,
            "sex": s.sex,
            "source": s.source,
            "notes": s.notes,
        })
    }).collect();

    let output = serde_json::json!({
        "filter_age": age,
        "filter_sex": sex,
        "screening_count": items.len(),
        "screenings": items,
        "disclaimer": "General guidelines only. Consult your healthcare provider for personalized recommendations."
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
