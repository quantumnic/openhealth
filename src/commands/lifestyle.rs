use colored::Colorize;
use rusqlite::Connection;

struct LifestyleTip {
    category: &'static str,
    emoji: &'static str,
    title: &'static str,
    advice: &'static str,
    source: &'static str,
}

pub fn run(conn: &Connection, age: Option<u8>, sex: Option<&str>, factors: Option<&str>, json: bool) {
    let tips = generate_tips(age, sex, factors);

    if json {
        let json_tips: Vec<serde_json::Value> = tips
            .iter()
            .map(|t| {
                serde_json::json!({
                    "category": t.category,
                    "title": t.title,
                    "advice": t.advice,
                    "source": t.source,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_tips).unwrap());
        return;
    }

    println!("\n{}", "╔══════════════════════════════════════════════════╗".cyan());
    println!("{}", "║    🌿 Personalized Lifestyle Recommendations    ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════════╝".cyan());

    if let Some(a) = age {
        print!("\n  👤 Age: {}", a);
    }
    if let Some(s) = sex {
        print!("  Sex: {}", s);
    }
    if let Some(f) = factors {
        print!("  Factors: {}", f);
    }
    println!();

    // Also check disease count for context
    let disease_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM diseases", [], |r| r.get(0))
        .unwrap_or(0);
    println!(
        "\n  📊 Based on {} diseases in database and your profile:\n",
        disease_count
    );

    let mut current_cat = "";
    for tip in &tips {
        if tip.category != current_cat {
            current_cat = tip.category;
            println!("  {} {}", tip.emoji, tip.category.bold().underline());
        }
        println!("    {} {}", "•".green(), tip.title.bold());
        println!("      {}", tip.advice);
        println!("      {}", format!("Source: {}", tip.source).dimmed());
        println!();
    }

    println!(
        "  {}",
        "⚠️  These are general guidelines — consult a healthcare provider for personalized advice."
            .yellow()
    );
    println!();
}

fn generate_tips(age: Option<u8>, sex: Option<&str>, factors: Option<&str>) -> Vec<LifestyleTip> {
    let mut tips = Vec::new();
    let factor_list: Vec<&str> = factors
        .map(|f| f.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|s| -> &str { Box::leak(s.into_boxed_str()) })
        .collect();

    // Universal tips
    tips.push(LifestyleTip {
        category: "Physical Activity",
        emoji: "🏃",
        title: "150 minutes of moderate exercise per week",
        advice: "Brisk walking, cycling, or swimming for 30 minutes, 5 days a week. Include 2 days of strength training.",
        source: "WHO Physical Activity Guidelines",
    });

    tips.push(LifestyleTip {
        category: "Nutrition",
        emoji: "🥗",
        title: "Eat at least 5 portions of fruits and vegetables daily",
        advice: "Aim for variety and color. Include leafy greens, berries, citrus, and cruciferous vegetables. Limit processed foods and added sugars.",
        source: "WHO Healthy Diet Fact Sheet",
    });

    tips.push(LifestyleTip {
        category: "Hydration",
        emoji: "💧",
        title: "Adequate daily water intake",
        advice: "Generally 2-3 liters per day for adults. More in hot weather or during exercise. Use 'openhealth hydration' for personalized calculation.",
        source: "European Food Safety Authority",
    });

    tips.push(LifestyleTip {
        category: "Sleep",
        emoji: "😴",
        title: "7-9 hours of quality sleep per night",
        advice: "Maintain consistent sleep/wake times. Avoid screens 1 hour before bed. Keep bedroom cool and dark. Limit caffeine after 2 PM.",
        source: "National Sleep Foundation",
    });

    tips.push(LifestyleTip {
        category: "Mental Health",
        emoji: "🧠",
        title: "Daily stress management practice",
        advice: "10-15 minutes of mindfulness, meditation, or deep breathing. Maintain social connections. Seek help if feeling persistently anxious or depressed.",
        source: "WHO Mental Health Guidelines",
    });

    // Age-specific tips
    if let Some(a) = age {
        if a < 18 {
            tips.push(LifestyleTip {
                category: "Physical Activity",
                emoji: "🏃",
                title: "60 minutes of activity daily for children and adolescents",
                advice: "Mix of aerobic activities, muscle-strengthening, and bone-strengthening exercises. Limit screen time to 2 hours. Encourage outdoor play.",
                source: "WHO Physical Activity Guidelines for Children",
            });
            tips.push(LifestyleTip {
                category: "Nutrition",
                emoji: "🥗",
                title: "Ensure adequate calcium and vitamin D",
                advice: "Critical for bone development. Dairy products, fortified foods, and sunlight exposure. 600 IU vitamin D daily recommended.",
                source: "American Academy of Pediatrics",
            });
        }
        if a >= 40 {
            tips.push(LifestyleTip {
                category: "Screening",
                emoji: "🔬",
                title: "Regular health screenings after 40",
                advice: "Blood pressure annually, cholesterol every 5 years, blood glucose every 3 years. Use 'openhealth screen' for full age-specific recommendations.",
                source: "USPSTF Screening Guidelines",
            });
        }
        if a >= 50 {
            tips.push(LifestyleTip {
                category: "Screening",
                emoji: "🔬",
                title: "Colorectal cancer screening from age 50",
                advice: "Colonoscopy every 10 years or stool-based tests annually. Discuss with doctor which method is appropriate.",
                source: "American Cancer Society",
            });
            tips.push(LifestyleTip {
                category: "Physical Activity",
                emoji: "🏃",
                title: "Include balance and flexibility training",
                advice: "Tai chi, yoga, or balance exercises 3 times per week to prevent falls. Strength training to maintain muscle mass and bone density.",
                source: "WHO Physical Activity Guidelines for Older Adults",
            });
        }
        if a >= 65 {
            tips.push(LifestyleTip {
                category: "Nutrition",
                emoji: "🥗",
                title: "Increased protein and vitamin B12 intake",
                advice: "1.0-1.2g protein per kg body weight daily. B12 supplementation as absorption decreases with age. Ensure adequate vitamin D (800-1000 IU).",
                source: "European Society for Clinical Nutrition",
            });
        }
    }

    // Sex-specific tips
    if let Some(s) = sex {
        let s_lower = s.to_lowercase();
        if s_lower == "female" {
            tips.push(LifestyleTip {
                category: "Women's Health",
                emoji: "♀️",
                title: "Iron-rich foods and folic acid",
                advice: "Include iron-rich foods (red meat, beans, spinach) with vitamin C for absorption. 400 µg folic acid daily if of childbearing age.",
                source: "WHO Iron Supplementation Guidelines",
            });
            if age.is_some_and(|a| a >= 25) {
                tips.push(LifestyleTip {
                    category: "Women's Health",
                    emoji: "♀️",
                    title: "Regular cervical and breast screening",
                    advice: "Cervical screening from age 25 (every 3-5 years). Breast self-exam monthly. Mammography from 40-50 based on risk.",
                    source: "WHO Cancer Prevention Guidelines",
                });
            }
        }
        if s_lower == "male" && age.is_some_and(|a| a >= 50) {
            tips.push(LifestyleTip {
                category: "Men's Health",
                emoji: "♂️",
                title: "Discuss prostate screening with doctor",
                advice: "PSA testing is not universally recommended — discuss individual risk with your healthcare provider, especially if family history exists.",
                source: "USPSTF Prostate Cancer Screening",
            });
        }
    }

    // Factor-specific tips
    for factor in &factor_list {
        match *factor {
            f if f.contains("smok") || f.contains("tobacco") => {
                tips.push(LifestyleTip {
                    category: "Tobacco Cessation",
                    emoji: "🚭",
                    title: "Quit smoking — single most impactful health change",
                    advice: "Use nicotine replacement therapy, varenicline, or bupropion. Combine with behavioral support. Risk of heart disease halves within 1 year of quitting.",
                    source: "WHO Framework Convention on Tobacco Control",
                });
            }
            f if f.contains("obes") || f.contains("overweight") || f.contains("weight") => {
                tips.push(LifestyleTip {
                    category: "Weight Management",
                    emoji: "⚖️",
                    title: "Sustainable weight loss: 0.5-1 kg per week",
                    advice: "500 calorie daily deficit through diet and exercise combined. Avoid crash diets. Focus on whole foods, portion control, and regular physical activity.",
                    source: "WHO Obesity Prevention Guidelines",
                });
            }
            f if f.contains("diabet") || f.contains("blood sugar") || f.contains("glucose") => {
                tips.push(LifestyleTip {
                    category: "Diabetes Management",
                    emoji: "🩸",
                    title: "Blood sugar control through lifestyle",
                    advice: "Low glycemic index diet, regular exercise (150 min/week), weight management. Monitor HbA1c every 3-6 months. Foot checks daily.",
                    source: "International Diabetes Federation",
                });
            }
            f if f.contains("hypertens") || f.contains("blood pressure") => {
                tips.push(LifestyleTip {
                    category: "Blood Pressure",
                    emoji: "❤️",
                    title: "DASH diet and sodium reduction",
                    advice: "Limit sodium to <2g/day. DASH diet (rich in fruits, vegetables, whole grains, low-fat dairy). Regular aerobic exercise. Limit alcohol.",
                    source: "AHA Blood Pressure Guidelines",
                });
            }
            f if f.contains("sedentary") || f.contains("inactive") => {
                tips.push(LifestyleTip {
                    category: "Physical Activity",
                    emoji: "🏃",
                    title: "Break up sitting time every 30 minutes",
                    advice: "Stand up, stretch, or walk for 2-3 minutes every 30 minutes of sitting. Use a standing desk if possible. Take stairs instead of elevators.",
                    source: "American Heart Association",
                });
            }
            f if f.contains("stress") || f.contains("anxiety") => {
                tips.push(LifestyleTip {
                    category: "Mental Health",
                    emoji: "🧠",
                    title: "Evidence-based stress reduction",
                    advice: "Mindfulness-based stress reduction (MBSR): 8-week program shown to reduce anxiety by 30-40%. Regular exercise, social connection, and adequate sleep are equally important.",
                    source: "American Psychological Association",
                });
            }
            f if f.contains("alcohol") => {
                tips.push(LifestyleTip {
                    category: "Alcohol",
                    emoji: "🍷",
                    title: "Moderate alcohol or consider abstaining",
                    advice: "If drinking: max 1 drink/day for women, 2 for men. Recent evidence suggests no safe level of alcohol. Consider alcohol-free days each week.",
                    source: "WHO Global Alcohol Action Plan",
                });
            }
            _ => {}
        }
    }

    tips
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn test_generate_tips_universal() {
        let tips = generate_tips(None, None, None);
        assert!(tips.len() >= 5, "Should have at least 5 universal tips");
    }

    #[test]
    fn test_generate_tips_age_specific() {
        let tips_child = generate_tips(Some(10), None, None);
        let tips_adult = generate_tips(Some(30), None, None);
        assert!(
            tips_child.len() > tips_adult.len(),
            "Children should get age-specific tips"
        );
    }

    #[test]
    fn test_generate_tips_elderly() {
        let tips = generate_tips(Some(70), None, None);
        assert!(
            tips.iter().any(|t| t.title.contains("balance")),
            "Elderly should get balance training tip"
        );
    }

    #[test]
    fn test_generate_tips_sex_specific() {
        let tips_f = generate_tips(Some(30), Some("female"), None);
        let tips_m = generate_tips(Some(30), Some("male"), None);
        assert!(
            tips_f.iter().any(|t| t.category == "Women's Health"),
            "Female should get women's health tips"
        );
        assert!(
            !tips_m.iter().any(|t| t.category == "Women's Health"),
            "Male should NOT get women's health tips"
        );
    }

    #[test]
    fn test_generate_tips_smoking_factor() {
        let tips = generate_tips(None, None, Some("smoking"));
        assert!(
            tips.iter().any(|t| t.title.contains("smoking") || t.title.contains("Quit")),
            "Should include smoking cessation tip"
        );
    }

    #[test]
    fn test_generate_tips_diabetes_factor() {
        let tips = generate_tips(None, None, Some("diabetes"));
        assert!(
            tips.iter().any(|t| t.category == "Diabetes Management"),
            "Should include diabetes management tip"
        );
    }

    #[test]
    fn test_lifestyle_run_json() {
        let conn = db::init_memory_database().unwrap();
        // Just verify it doesn't panic
        run(&conn, Some(30), Some("female"), Some("smoking,obesity"), true);
    }
}
