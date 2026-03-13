use colored::Colorize;
use rusqlite::Connection;
use rand_seed::RngSeed;

/// Simple deterministic PRNG seeded from system time for quiz randomization.
mod rand_seed {
    pub struct RngSeed {
        state: u64,
    }
    impl RngSeed {
        pub fn new() -> Self {
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64;
            Self { state: seed }
        }
        pub fn next_usize(&mut self, max: usize) -> usize {
            // xorshift64
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            (self.state as usize) % max
        }
        pub fn shuffle<T>(&mut self, slice: &mut [T]) {
            for i in (1..slice.len()).rev() {
                let j = self.next_usize(i + 1);
                slice.swap(i, j);
            }
        }
    }
}

struct QuizQuestion {
    question: String,
    options: Vec<String>,
    correct: usize, // 0-indexed
    explanation: String,
}

pub fn run(conn: &Connection, count: usize) {
    println!("{}", "🧠 OpenHealth Medical Knowledge Quiz".bold().cyan());
    println!("{}", "─".repeat(45));
    println!("Answer each question by entering the number (1-4).");
    println!("Type 'q' to quit early.\n");

    let questions = generate_questions(conn, count);
    if questions.is_empty() {
        println!("{}", "Could not generate quiz questions.".red());
        return;
    }

    let mut correct_count = 0;
    let total = questions.len();

    for (i, q) in questions.iter().enumerate() {
        println!(
            "{} {}",
            format!("Question {}/{}:", i + 1, total).bold().yellow(),
            q.question
        );
        for (j, opt) in q.options.iter().enumerate() {
            println!("  {}. {}", j + 1, opt);
        }

        let answer = read_answer();
        match answer {
            None => {
                println!("\n{}\n", "Quiz ended early.".yellow());
                break;
            }
            Some(a) => {
                if a == q.correct {
                    println!("{}", "  ✅ Correct!".green().bold());
                    correct_count += 1;
                } else {
                    println!(
                        "  {} The answer was: {}",
                        "❌ Incorrect.".red().bold(),
                        q.options[q.correct].bold()
                    );
                }
                println!("  💡 {}\n", q.explanation.dimmed());
            }
        }
    }

    // Score summary
    println!("{}", "─".repeat(45));
    let pct = if total > 0 {
        (correct_count as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };
    let grade = match pct {
        90..=100 => "🏆 Excellent!",
        70..=89 => "👍 Good job!",
        50..=69 => "📚 Keep studying!",
        _ => "💪 Don't give up!",
    };
    println!(
        "Score: {}/{} ({}%) — {}",
        correct_count.to_string().bold(),
        total,
        pct,
        grade.bold()
    );
}

fn read_answer() -> Option<usize> {
    print!("  Your answer: ");
    use std::io::Write;
    std::io::stdout().flush().ok();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("q") {
        return None;
    }
    match trimmed.parse::<usize>() {
        Ok(n) if (1..=4).contains(&n) => Some(n - 1),
        _ => {
            println!("  {}", "(Invalid input, counting as wrong)".dimmed());
            Some(99) // will never match correct
        }
    }
}

fn generate_questions(conn: &Connection, count: usize) -> Vec<QuizQuestion> {
    let mut rng = RngSeed::new();
    let mut questions = Vec::new();

    // Gather disease data
    let mut stmt = conn
        .prepare("SELECT d.name, d.severity, d.category, d.contagious, d.description FROM diseases d")
        .unwrap();
    let diseases: Vec<(String, String, String, bool, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                row.get::<_, i32>(3)? != 0,
                row.get(4)?,
            ))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    if diseases.len() < 4 {
        return questions;
    }

    // Gather treatments
    let mut treat_stmt = conn
        .prepare("SELECT d.name, t.first_aid FROM treatments t JOIN diseases d ON d.id = t.disease_id")
        .unwrap();
    let _treatments: Vec<(String, String)> = treat_stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            ))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .filter(|(_, fa)| !fa.is_empty())
        .collect();

    // Gather symptom data
    let mut sym_stmt = conn
        .prepare(
            "SELECT d.name, s.name FROM disease_symptoms ds 
             JOIN diseases d ON d.id = ds.disease_id 
             JOIN symptoms s ON s.id = ds.symptom_id 
             WHERE ds.is_primary = 1"
        )
        .unwrap();
    let _primary_symptoms: Vec<(String, String)> = sym_stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    let mut indices: Vec<usize> = (0..diseases.len()).collect();
    rng.shuffle(&mut indices);

    let mut generated = 0;
    let mut idx_iter = indices.into_iter().cycle();

    // Generate diverse question types
    while generated < count {
        let di = idx_iter.next().unwrap();
        let (ref name, ref severity, ref category, contagious, ref desc) = diseases[di];

        let q_type = rng.next_usize(4);

        let q = match q_type {
            0 => {
                // "What severity is X?"
                let sev_options = ["low", "medium", "high"];
                let correct_idx = sev_options.iter().position(|s| *s == severity.as_str()).unwrap_or(0);
                let mut opts: Vec<String> = sev_options.iter().map(|s| {
                    match *s {
                        "low" => "🟢 Low".to_string(),
                        "medium" => "🟡 Medium".to_string(),
                        "high" => "🔴 High".to_string(),
                        _ => s.to_string(),
                    }
                }).collect();
                opts.push("None of the above".to_string());
                QuizQuestion {
                    question: format!("What is the severity level of {}?", name),
                    options: opts,
                    correct: correct_idx,
                    explanation: format!("{} is classified as {} severity.", name, severity),
                }
            }
            1 => {
                // "Is X contagious?"
                let correct_idx = if contagious { 0 } else { 1 };
                QuizQuestion {
                    question: format!("Is {} contagious?", name),
                    options: vec![
                        "Yes, it is contagious".to_string(),
                        "No, it is not contagious".to_string(),
                        "Only in certain conditions".to_string(),
                        "Unknown".to_string(),
                    ],
                    correct: correct_idx,
                    explanation: format!(
                        "{} is {}.",
                        name,
                        if contagious { "contagious" } else { "not contagious" }
                    ),
                }
            }
            2 => {
                // "Which category does X belong to?"
                let mut cat_opts: Vec<String> = diseases
                    .iter()
                    .map(|(_, _, c, _, _)| c.clone())
                    .filter(|c| !c.is_empty())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                rng.shuffle(&mut cat_opts);
                let correct_cat = category.clone();
                // Ensure correct answer is in options
                cat_opts.retain(|c| c != &correct_cat);
                let mut opts: Vec<String> = cat_opts.into_iter().take(3).collect();
                let insert_pos = rng.next_usize(opts.len() + 1);
                opts.insert(insert_pos, correct_cat.clone());
                QuizQuestion {
                    question: format!("Which medical category does {} belong to?", name),
                    options: opts,
                    correct: insert_pos,
                    explanation: format!("{} belongs to the {} category.", name, correct_cat),
                }
            }
            3 => {
                // "Which disease matches this description?"
                let mut wrong: Vec<String> = diseases
                    .iter()
                    .filter(|(n, _, _, _, _)| n != name)
                    .map(|(n, _, _, _, _)| n.clone())
                    .collect();
                rng.shuffle(&mut wrong);
                let mut opts: Vec<String> = wrong.into_iter().take(3).collect();
                let insert_pos = rng.next_usize(opts.len() + 1);
                opts.insert(insert_pos, name.clone());
                // Truncate description for question
                let short_desc = if desc.len() > 120 {
                    format!("{}…", &desc[..120])
                } else {
                    desc.clone()
                };
                QuizQuestion {
                    question: format!("Which disease matches: \"{}\"", short_desc),
                    options: opts,
                    correct: insert_pos,
                    explanation: format!("The description matches {}.", name),
                }
            }
            _ => continue,
        };

        questions.push(q);
        generated += 1;
    }

    questions
}
