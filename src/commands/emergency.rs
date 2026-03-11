use colored::*;

pub fn run() {
    println!("{}", "╔══════════════════════════════════════════════════════════╗".red().bold());
    println!("{}", "║            🚨  EMERGENCY CHECKLIST  🚨                  ║".red().bold());
    println!("{}", "╚══════════════════════════════════════════════════════════╝".red().bold());
    println!();

    println!("{}", "STEP 1: ASSESS THE SITUATION".bold().underline());
    println!("  □ Is the person responsive? (Tap shoulders, call out)");
    println!("  □ Is the person breathing normally?");
    println!("  □ Is there severe bleeding?");
    println!("  □ Are they choking?");
    println!();

    println!("{}", "STEP 2: CALL FOR HELP".bold().underline());
    println!("  📞 Call local emergency number:");
    println!("     • International: 112");
    println!("     • USA/Canada: 911");
    println!("     • UK: 999");
    println!("     • Australia: 000");
    println!("     • India: 108 (ambulance) / 112");
    println!("     • China: 120");
    println!();

    println!("{}", "STEP 3: IMMEDIATE ACTIONS".bold().underline());
    println!();

    println!("  {} Start CPR", "🫀 NOT BREATHING / NO PULSE:".red().bold());
    println!("     • 30 chest compressions (hard, fast, center of chest)");
    println!("     • 2 rescue breaths");
    println!("     • Repeat until help arrives");
    println!("     • Use AED if available");
    println!();

    println!("  {} Apply pressure", "🩸 SEVERE BLEEDING:".red().bold());
    println!("     • Press firmly with clean cloth");
    println!("     • Do NOT remove cloth, add more on top");
    println!("     • Elevate injured limb above heart");
    println!("     • Apply tourniquet only as last resort");
    println!();

    println!("  {} Heimlich maneuver", "🫁 CHOKING:".red().bold());
    println!("     • Stand behind person, fist above navel");
    println!("     • Quick upward thrusts");
    println!("     • If alone: thrust against chair back");
    println!("     • Infant: 5 back blows + 5 chest thrusts");
    println!();

    println!("  {} Cool immediately", "🔥 BURNS:".yellow().bold());
    println!("     • Cool running water for 20 minutes");
    println!("     • Do NOT use ice, butter, or toothpaste");
    println!("     • Cover loosely with clean dressing");
    println!();

    println!("  {} Stay calm", "🐍 SNAKEBITE:".yellow().bold());
    println!("     • Keep still, immobilize bitten limb");
    println!("     • Do NOT cut, suck, or tourniquet");
    println!("     • Remove jewelry near bite");
    println!("     • Get to hospital for antivenom");
    println!();

    println!("  {} Act immediately", "🧠 STROKE (FAST):".red().bold());
    println!("     • Face: Ask to smile — is one side drooping?");
    println!("     • Arms: Raise both — does one drift down?");
    println!("     • Speech: Repeat a sentence — is it slurred?");
    println!("     • Time: Note the time, call emergency NOW");
    println!();

    println!("  {}", "💊 POISONING:".yellow().bold());
    println!("     • Do NOT induce vomiting");
    println!("     • Identify the substance if possible");
    println!("     • Call poison control or emergency services");
    println!();

    println!("{}", "STEP 4: WHILE WAITING FOR HELP".bold().underline());
    println!("  □ Keep the person calm and still");
    println!("  □ Maintain airway (head tilt, chin lift if unconscious)");
    println!("  □ Keep warm (cover with blanket)");
    println!("  □ Do NOT give food or water if surgery may be needed");
    println!("  □ Note time of injury/onset for medical team");
    println!();

    println!("{}", "⚠️  This checklist is for guidance only. Professional medical".yellow());
    println!("{}", "   care should always be sought in emergencies.".yellow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_runs() {
        // Just verify it doesn't panic
        run();
    }
}
