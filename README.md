<div align="center">

# 🏥 OpenHealth

### Offline AI Medical Diagnostics for Everyone

**Healthcare for the 3.5 billion people without access to a doctor.**

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Offline First](https://img.shields.io/badge/Offline-First-blue)](.)

*Symptom analysis • WHO protocols • Runs on any device • Free forever*

</div>

---

## 🌍 The Problem

**3.5 billion people** — nearly half the world — lack access to essential health services. In rural Sub-Saharan Africa, there is **1 doctor per 10,000+ people**. A child with malaria symptoms may be hours from the nearest clinic. A mother with preeclampsia may not recognize the warning signs. A farmer bitten by a snake may not know that sucking the wound does more harm than good.

**Information saves lives.** OpenHealth puts medical knowledge directly in the hands of those who need it most.

## 💡 What It Does

OpenHealth is a **fully offline** medical diagnostic tool that:

- 🔍 **Analyzes symptoms** using Bayesian probability scoring
- 📊 **Ranks possible conditions** with confidence percentages
- 🚦 **Triages severity**: 🟢 Home care / 🟡 See doctor / 🔴 Emergency
- 💊 **Provides WHO treatment protocols** with first aid instructions
- 🛡️ **Covers 50+ diseases** including the major killers in developing countries
- 📴 **Works completely offline** — no internet, no API keys, no cloud

## 🚀 Installation

```bash
# From source
git clone https://github.com/quantumnic/openhealth
cd openhealth
cargo install --path .

# Or build directly
cargo build --release
```

**Requirements:** Rust 1.70+ (SQLite is bundled, no external dependencies)

## 📖 Usage

### Interactive Symptom Check
```bash
openhealth check
```
Guided Q&A that walks you through symptoms and provides analysis.

### Quick Symptom Analysis
```bash
openhealth symptoms "fever headache nausea chills"
```
Instantly analyze multiple symptoms and get ranked possible conditions.

### Disease Information
```bash
openhealth disease "malaria"
openhealth disease "heart attack"
```
Detailed information about a specific disease, including symptoms and severity.

### WHO Treatment Protocol
```bash
openhealth treatment "malaria"
openhealth treatment "cholera"
```
Evidence-based treatment protocols, first aid instructions, and prevention.

### Emergency Checklist
```bash
openhealth emergency
```
Step-by-step emergency response guide: CPR, bleeding, choking, burns, snakebite, stroke.

### Database Status
```bash
openhealth update
```
Check database version and statistics.

## 🗃️ Diseases Covered

OpenHealth includes detailed data for **50+ conditions**, prioritizing the diseases that kill the most people in resource-limited settings:

| Category | Diseases |
|----------|----------|
| **Vector-borne** | Malaria, Dengue, Yellow Fever, Typhus |
| **Waterborne** | Cholera, Typhoid, Giardiasis, Schistosomiasis |
| **Respiratory** | Pneumonia, TB, Influenza, Pertussis, Asthma |
| **Emergencies** | Heart Attack, Stroke, Appendicitis, Snakebite |
| **Childhood** | Measles, Chickenpox, Neonatal Sepsis, Malnutrition |
| **Chronic** | Diabetes, Hypertension, Epilepsy, Anemia |
| **Skin/Wound** | Burns, Wound Infection, Scabies, Cellulitis |
| **Other** | HIV, Hepatitis B, Rabies, Tetanus, Meningitis |

## 🧠 How It Works

### Bayesian Symptom Scoring

OpenHealth uses a weighted scoring algorithm:

1. **Each symptom has a weight** (0.0–1.0) indicating how strongly it's associated with a disease
2. **Primary symptoms** (★) get a bonus — these are the defining features of a condition
3. **Coverage ratio** — how many of a disease's symptoms you have
4. **Final score** = weighted match (40%) + primary symptom bonus (30%) + coverage (30%)

This produces a probability-like score capped at 95% (because no algorithm replaces a doctor).

### Severity Classification

- 🟢 **Green — Monitor at home**: Self-limiting conditions manageable with rest and self-care
- 🟡 **Yellow — See a doctor soon**: Needs professional medical attention within 24-48h
- 🔴 **Red — Emergency**: Seek immediate medical help. Life-threatening without intervention

## 📁 Architecture

```
src/
├── main.rs              # CLI entry point (clap)
├── commands/
│   ├── check.rs         # Interactive symptom checker
│   ├── symptoms.rs      # Quick analysis
│   ├── disease.rs       # Disease lookup
│   ├── treatment.rs     # WHO protocols
│   ├── emergency.rs     # Emergency checklist
│   └── update.rs        # Database management
├── db/
│   ├── mod.rs           # SQLite initialization
│   ├── schema.rs        # Table definitions
│   └── seed.rs          # 50+ disease seed data
├── engine/
│   ├── scorer.rs        # Bayesian symptom scoring
│   └── severity.rs      # Severity classification
└── display.rs           # Terminal output formatting
```

## ⚠️ Important Disclaimer

**OpenHealth is NOT a substitute for professional medical advice, diagnosis, or treatment.** It is an informational tool designed to help people in resource-limited settings understand their symptoms and know when to seek emergency care.

Always consult a qualified healthcare provider when possible.

## 🤝 Contributing

We welcome contributions, especially:

- 🌐 **Translations** — Help us reach 100+ languages
- 🏥 **Medical data review** — Verify and improve disease/symptom data
- 💻 **Code contributions** — New features, better algorithms
- 📝 **Documentation** — User guides, deployment guides for field use

## 🗺️ Roadmap

- [ ] Multi-language support (i18n framework)
- [ ] WHO ICD-11 full integration
- [ ] Drug interaction checker
- [ ] Offline-first mobile app (Android/iOS)
- [ ] Community health worker mode
- [ ] Voice input for low-literacy users
- [ ] Regional disease prevalence weighting
- [ ] Pregnancy & pediatric specialized modules

## 📄 License

MIT License — Free forever. Because healthcare should be a right, not a privilege.

---

<div align="center">

**Built with ❤️ for the world's most underserved communities**

*If you have access to healthcare, someone in your community might not. Share this tool.*

</div>
