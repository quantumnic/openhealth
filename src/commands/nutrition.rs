use colored::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct NutrientInfo {
    nutrient: &'static str,
    rda: &'static str,
    deficiency_symptoms: Vec<&'static str>,
    food_sources: Vec<&'static str>,
    risk_groups: Vec<&'static str>,
    deficiency_disease: &'static str,
}

fn get_nutrients() -> Vec<NutrientInfo> {
    vec![
        NutrientInfo {
            nutrient: "Vitamin A",
            rda: "700-900 mcg RAE/day",
            deficiency_symptoms: vec!["night blindness", "dry eyes", "dry skin", "frequent infections", "delayed growth"],
            food_sources: vec!["liver", "sweet potatoes", "carrots", "spinach", "eggs", "fortified milk"],
            risk_groups: vec!["children in developing countries", "pregnant/lactating women", "malabsorption disorders"],
            deficiency_disease: "Xerophthalmia / Night Blindness",
        },
        NutrientInfo {
            nutrient: "Vitamin B1 (Thiamine)",
            rda: "1.1-1.2 mg/day",
            deficiency_symptoms: vec!["fatigue", "irritability", "poor memory", "muscle weakness", "leg tingling", "heart failure"],
            food_sources: vec!["whole grains", "pork", "legumes", "seeds", "fortified cereals"],
            risk_groups: vec!["chronic alcoholics", "bariatric surgery patients", "malnutrition"],
            deficiency_disease: "Beriberi / Wernicke-Korsakoff Syndrome",
        },
        NutrientInfo {
            nutrient: "Vitamin B3 (Niacin)",
            rda: "14-16 mg NE/day",
            deficiency_symptoms: vec!["skin rash in sun-exposed areas", "diarrhea", "confusion", "swollen mouth/tongue"],
            food_sources: vec!["meat", "poultry", "fish", "peanuts", "mushrooms", "fortified grains"],
            risk_groups: vec!["chronic alcoholics", "corn-dependent diets", "malabsorption"],
            deficiency_disease: "Pellagra (3 D's: Dermatitis, Diarrhea, Dementia)",
        },
        NutrientInfo {
            nutrient: "Vitamin B12",
            rda: "2.4 mcg/day",
            deficiency_symptoms: vec!["fatigue", "weakness", "numbness/tingling", "pale skin", "sore tongue", "memory problems", "mood changes"],
            food_sources: vec!["meat", "fish", "eggs", "dairy", "fortified cereals", "nutritional yeast"],
            risk_groups: vec!["vegans/vegetarians", "elderly", "pernicious anemia", "gastric surgery patients"],
            deficiency_disease: "Megaloblastic Anemia / Subacute Combined Degeneration",
        },
        NutrientInfo {
            nutrient: "Vitamin C",
            rda: "75-90 mg/day",
            deficiency_symptoms: vec!["bleeding gums", "easy bruising", "slow wound healing", "fatigue", "joint pain", "dry skin"],
            food_sources: vec!["citrus fruits", "strawberries", "bell peppers", "broccoli", "tomatoes", "kiwi"],
            risk_groups: vec!["smokers", "limited fruit/vegetable intake", "malabsorption", "elderly"],
            deficiency_disease: "Scurvy",
        },
        NutrientInfo {
            nutrient: "Vitamin D",
            rda: "600-800 IU/day (15-20 mcg)",
            deficiency_symptoms: vec!["bone pain", "muscle weakness", "fatigue", "depression", "frequent infections", "slow wound healing"],
            food_sources: vec!["sunlight exposure", "fatty fish", "fortified milk", "egg yolks", "fortified cereals", "mushrooms (UV-exposed)"],
            risk_groups: vec!["limited sun exposure", "dark skin", "elderly", "obese", "malabsorption", "northern latitudes"],
            deficiency_disease: "Rickets (children) / Osteomalacia (adults)",
        },
        NutrientInfo {
            nutrient: "Iron",
            rda: "8-18 mg/day (higher for menstruating women)",
            deficiency_symptoms: vec!["fatigue", "weakness", "pale skin", "cold hands/feet", "brittle nails", "cravings for non-food items", "dizziness"],
            food_sources: vec!["red meat", "spinach", "lentils", "fortified cereals", "tofu", "dark chocolate"],
            risk_groups: vec!["menstruating women", "pregnant women", "vegetarians", "frequent blood donors", "infants"],
            deficiency_disease: "Iron-Deficiency Anemia",
        },
        NutrientInfo {
            nutrient: "Iodine",
            rda: "150 mcg/day",
            deficiency_symptoms: vec!["goiter", "fatigue", "weight gain", "cold intolerance", "dry skin", "cognitive impairment"],
            food_sources: vec!["iodized salt", "seaweed", "fish", "dairy", "eggs"],
            risk_groups: vec!["non-iodized salt users", "pregnant women", "regions without iodized salt"],
            deficiency_disease: "Goiter / Cretinism (in children)",
        },
        NutrientInfo {
            nutrient: "Zinc",
            rda: "8-11 mg/day",
            deficiency_symptoms: vec!["poor wound healing", "hair loss", "diarrhea", "loss of taste", "loss of smell", "frequent infections", "skin lesions"],
            food_sources: vec!["oysters", "red meat", "poultry", "beans", "nuts", "whole grains"],
            risk_groups: vec!["vegetarians", "pregnant/lactating women", "alcoholics", "GI disease patients"],
            deficiency_disease: "Acrodermatitis Enteropathica (severe) / Growth Retardation",
        },
        NutrientInfo {
            nutrient: "Folate (Vitamin B9)",
            rda: "400 mcg DFE/day (600 in pregnancy)",
            deficiency_symptoms: vec!["fatigue", "mouth sores", "gray hair", "swollen tongue", "poor growth"],
            food_sources: vec!["dark leafy greens", "legumes", "fortified grains", "asparagus", "avocado", "citrus"],
            risk_groups: vec!["pregnant women", "alcoholics", "malabsorption", "certain medications (methotrexate)"],
            deficiency_disease: "Megaloblastic Anemia / Neural Tube Defects (in pregnancy)",
        },
        NutrientInfo {
            nutrient: "Calcium",
            rda: "1000-1200 mg/day",
            deficiency_symptoms: vec!["muscle cramps", "numbness/tingling in fingers", "brittle nails", "bone fractures", "dental problems"],
            food_sources: vec!["dairy products", "fortified plant milks", "sardines", "leafy greens", "tofu", "almonds"],
            risk_groups: vec!["postmenopausal women", "lactose intolerant", "vegans", "elderly", "vitamin D deficient"],
            deficiency_disease: "Osteoporosis / Osteopenia / Hypocalcemia",
        },
        NutrientInfo {
            nutrient: "Magnesium",
            rda: "310-420 mg/day",
            deficiency_symptoms: vec!["muscle cramps", "tremors", "insomnia", "anxiety", "irregular heartbeat", "nausea", "fatigue"],
            food_sources: vec!["dark chocolate", "avocados", "nuts", "legumes", "whole grains", "seeds", "bananas"],
            risk_groups: vec!["type 2 diabetes", "GI diseases", "alcoholics", "elderly", "diuretic users"],
            deficiency_disease: "Hypomagnesemia / Cardiac Arrhythmias",
        },
    ]
}

pub fn run(query: Option<&str>, as_json: bool) {
    let nutrients = get_nutrients();

    if let Some(q) = query {
        let q_lower = q.to_lowercase();
        let matched: Vec<&NutrientInfo> = nutrients.iter()
            .filter(|n| {
                n.nutrient.to_lowercase().contains(&q_lower)
                    || n.deficiency_disease.to_lowercase().contains(&q_lower)
                    || n.deficiency_symptoms.iter().any(|s| s.to_lowercase().contains(&q_lower))
            })
            .collect();

        if matched.is_empty() {
            if as_json {
                println!("{{\"error\": \"No nutrients found matching '{}'\"}}", q);
            } else {
                println!("{} No nutrients found matching '{}'", "✗".red(), q);
            }
            return;
        }

        if as_json {
            println!("{}", serde_json::to_string_pretty(&matched).unwrap_or_default());
            return;
        }

        for n in &matched {
            print_nutrient_detail(n);
        }
    } else {
        if as_json {
            println!("{}", serde_json::to_string_pretty(&nutrients).unwrap_or_default());
            return;
        }

        println!("{}", "🥗 Nutritional Deficiency Reference".bright_green().bold());
        println!("{}", "━".repeat(60).bright_green());
        println!();

        for n in &nutrients {
            print_nutrient_detail(n);
        }

        println!("{}", "━".repeat(60).bright_green());
        println!("{}", "💡 Tip: Use `openhealth nutrition <query>` to search by nutrient, symptom, or disease.".dimmed());
        println!("{}", "   Example: `openhealth nutrition fatigue` to find deficiencies causing fatigue.".dimmed());
    }
}

fn print_nutrient_detail(n: &NutrientInfo) {
    println!("  💊 {}", n.nutrient.bright_yellow().bold());
    println!("     RDA: {}", n.rda.bright_white());
    println!("     Deficiency: {}", n.deficiency_disease.bright_red());
    println!("     Symptoms: {}", n.deficiency_symptoms.join(", "));
    println!("     Food sources: {}", n.food_sources.join(", ").bright_green());
    println!("     At-risk groups: {}", n.risk_groups.join(", ").dimmed());
    println!();
}

pub fn assess(symptoms: &str, as_json: bool) {
    let nutrients = get_nutrients();
    let input: Vec<String> = symptoms.split(',')
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if input.is_empty() {
        if as_json {
            println!("{{\"error\": \"No symptoms provided\"}}");
        } else {
            println!("{} Please provide symptoms to assess.", "✗".red());
        }
        return;
    }

    #[derive(Serialize)]
    struct DeficiencyMatch<'a> {
        nutrient: &'a str,
        matched_symptoms: Vec<&'a str>,
        total_deficiency_symptoms: usize,
        match_ratio: f64,
        deficiency_disease: &'a str,
        food_sources: &'a [&'a str],
    }

    let mut matches: Vec<DeficiencyMatch> = Vec::new();

    for n in &nutrients {
        let matched: Vec<&str> = n.deficiency_symptoms.iter()
            .filter(|ds| {
                let ds_lower = ds.to_lowercase();
                input.iter().any(|inp| ds_lower.contains(inp) || inp.contains(&ds_lower))
            })
            .copied()
            .collect();

        if !matched.is_empty() {
            let ratio = matched.len() as f64 / n.deficiency_symptoms.len() as f64;
            matches.push(DeficiencyMatch {
                nutrient: n.nutrient,
                matched_symptoms: matched,
                total_deficiency_symptoms: n.deficiency_symptoms.len(),
                match_ratio: ratio,
                deficiency_disease: n.deficiency_disease,
                food_sources: &n.food_sources,
            });
        }
    }

    matches.sort_by(|a, b| b.match_ratio.partial_cmp(&a.match_ratio).unwrap());

    if as_json {
        println!("{}", serde_json::to_string_pretty(&matches).unwrap_or_default());
        return;
    }

    if matches.is_empty() {
        println!("{} No nutritional deficiencies matched your symptoms.", "✗".red());
        println!("   Symptoms checked: {}", input.join(", "));
        return;
    }

    println!("{}", "🥗 Nutritional Deficiency Assessment".bright_green().bold());
    println!("{}", "━".repeat(60).bright_green());
    println!("   Symptoms: {}", input.join(", ").bright_white());
    println!();

    for m in &matches {
        let pct = (m.match_ratio * 100.0) as u32;
        let indicator = if pct >= 50 { "🔴" } else if pct >= 30 { "🟡" } else { "🟢" };
        println!("  {} {} — {}% symptom match",
            indicator, m.nutrient.bright_yellow().bold(), pct);
        println!("     Matched: {}", m.matched_symptoms.join(", ").bright_white());
        println!("     Could indicate: {}", m.deficiency_disease.bright_red());
        println!("     Eat more: {}", m.food_sources.join(", ").bright_green());
        println!();
    }

    println!("{}", "━".repeat(60).bright_green());
    println!("{}", "⚠️  This is informational only. See a healthcare provider for proper testing.".yellow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nutrients_not_empty() {
        assert!(!get_nutrients().is_empty());
    }

    #[test]
    fn test_all_nutrients_have_food_sources() {
        for n in get_nutrients() {
            assert!(!n.food_sources.is_empty(), "{} has no food sources", n.nutrient);
        }
    }

    #[test]
    fn test_all_nutrients_have_deficiency_symptoms() {
        for n in get_nutrients() {
            assert!(!n.deficiency_symptoms.is_empty(), "{} has no deficiency symptoms", n.nutrient);
        }
    }

    #[test]
    fn test_all_nutrients_have_risk_groups() {
        for n in get_nutrients() {
            assert!(!n.risk_groups.is_empty(), "{} has no risk groups", n.nutrient);
        }
    }

    #[test]
    fn test_nutrients_count() {
        assert!(get_nutrients().len() >= 12, "Should have at least 12 nutrients");
    }
}
