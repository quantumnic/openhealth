/// Severity classification for triage.
#[derive(Debug, Clone, PartialEq)]
pub enum SeverityLevel {
    /// 🟢 Monitor at home — mild condition, self-care appropriate
    Green,
    /// 🟡 See doctor soon — needs medical attention but not emergency
    Yellow,
    /// 🔴 Emergency — seek immediate medical help
    Red,
}

impl SeverityLevel {
    pub fn from_str(s: &str) -> Self {
        match s {
            "high" => SeverityLevel::Red,
            "medium" => SeverityLevel::Yellow,
            _ => SeverityLevel::Green,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            SeverityLevel::Green => "🟢",
            SeverityLevel::Yellow => "🟡",
            SeverityLevel::Red => "🔴",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            SeverityLevel::Green => "Monitor at home",
            SeverityLevel::Yellow => "See a doctor soon",
            SeverityLevel::Red => "EMERGENCY — Seek help immediately",
        }
    }

    pub fn advice(&self) -> &str {
        match self {
            SeverityLevel::Green => "This condition can usually be managed at home with rest and self-care. See a doctor if symptoms worsen or persist beyond a few days.",
            SeverityLevel::Yellow => "This condition needs medical attention. Schedule a visit with a healthcare provider within 24-48 hours, or sooner if symptoms worsen.",
            SeverityLevel::Red => "⚠️  This is a medical emergency. Seek immediate help at the nearest hospital or clinic. Do not delay. Call emergency services if available.",
        }
    }
}

/// Determine overall severity from multiple results.
/// Takes the highest severity from the top matches.
pub fn overall_severity(severities: &[&str]) -> SeverityLevel {
    if severities.contains(&"high") {
        SeverityLevel::Red
    } else if severities.contains(&"medium") {
        SeverityLevel::Yellow
    } else {
        SeverityLevel::Green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_from_str() {
        assert_eq!(SeverityLevel::from_str("high"), SeverityLevel::Red);
        assert_eq!(SeverityLevel::from_str("medium"), SeverityLevel::Yellow);
        assert_eq!(SeverityLevel::from_str("low"), SeverityLevel::Green);
        assert_eq!(SeverityLevel::from_str("unknown"), SeverityLevel::Green);
    }

    #[test]
    fn test_severity_emoji() {
        assert_eq!(SeverityLevel::Red.emoji(), "🔴");
        assert_eq!(SeverityLevel::Yellow.emoji(), "🟡");
        assert_eq!(SeverityLevel::Green.emoji(), "🟢");
    }

    #[test]
    fn test_overall_severity() {
        assert_eq!(overall_severity(&["low", "low"]), SeverityLevel::Green);
        assert_eq!(overall_severity(&["low", "medium"]), SeverityLevel::Yellow);
        assert_eq!(overall_severity(&["low", "high"]), SeverityLevel::Red);
        assert_eq!(overall_severity(&["high", "medium", "low"]), SeverityLevel::Red);
    }

    #[test]
    fn test_severity_labels() {
        assert!(!SeverityLevel::Green.label().is_empty());
        assert!(!SeverityLevel::Yellow.label().is_empty());
        assert!(!SeverityLevel::Red.label().is_empty());
    }

    #[test]
    fn test_severity_advice() {
        assert!(SeverityLevel::Red.advice().contains("emergency"));
    }
}
