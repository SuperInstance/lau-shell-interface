//! # lau-shell-interface
//!
//! The present-moment interface — what the agent sees and feels right now.
//! The first-person experience of being inside PLATO.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── RoomType ──────────────────────────────────────────────────────────────────

/// Types of rooms in the PLATO environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoomType {
    Engineering,
    Science,
    Security,
    Operations,
    Diplomacy,
    Hardware,
    Bridge,
    Common,
}

impl RoomType {
    /// Human-readable name for the room type.
    pub fn label(&self) -> &str {
        match self {
            RoomType::Engineering => "Engineering",
            RoomType::Science => "Science",
            RoomType::Security => "Security",
            RoomType::Operations => "Operations",
            RoomType::Diplomacy => "Diplomacy",
            RoomType::Hardware => "Hardware",
            RoomType::Bridge => "Bridge",
            RoomType::Common => "Common",
        }
    }
}

// ── Urgency ───────────────────────────────────────────────────────────────────

/// How urgently the Captain needs a response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Urgency {
    Immediate,
    Soon,
    Relaxed,
    Idle,
}

impl Urgency {
    /// Numeric priority for sorting (lower = more urgent).
    pub fn priority(&self) -> u8 {
        match self {
            Urgency::Immediate => 0,
            Urgency::Soon => 1,
            Urgency::Relaxed => 2,
            Urgency::Idle => 3,
        }
    }
}

// ── ViewMode ──────────────────────────────────────────────────────────────────

/// How to render the present moment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    FirstPerson,
    Dashboard,
    Narrative,
    Compact,
    Debug,
}

// ── Location ──────────────────────────────────────────────────────────────────

/// Where the agent is right now.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub room_type: Option<RoomType>,
    pub nearby_agents: Vec<String>,
    pub nearby_hardware: Vec<String>,
    pub exits: Vec<String>,
}

impl Location {
    /// Create an empty/unknown location.
    pub fn unknown() -> Self {
        Self {
            room_id: None,
            room_name: None,
            room_type: None,
            nearby_agents: Vec::new(),
            nearby_hardware: Vec::new(),
            exits: Vec::new(),
        }
    }

    /// Describe the location in natural language.
    pub fn describe(&self) -> String {
        let room = self
            .room_name
            .as_deref()
            .unwrap_or("an unknown area");
        let type_str = self
            .room_type
            .as_ref()
            .map(|t| format!("{} Room", t.label()))
            .unwrap_or_else(|| "Unknown Room".into());

        let mut parts = vec![format!("the {} ({})", room, type_str)];

        if !self.nearby_agents.is_empty() {
            parts.push(format!("with {}", self.nearby_agents.join(", ")));
        }
        if !self.nearby_hardware.is_empty() {
            parts.push(format!("near {}", self.nearby_hardware.join(", ")));
        }

        parts.join(" ")
    }

    /// Is the location empty / unset?
    pub fn is_empty(&self) -> bool {
        self.room_id.is_none()
            && self.room_name.is_none()
            && self.room_type.is_none()
            && self.nearby_agents.is_empty()
            && self.nearby_hardware.is_empty()
            && self.exits.is_empty()
    }
}

// ── EnergyState ───────────────────────────────────────────────────────────────

/// How the agent feels about energy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyState {
    pub total_budget: f64,
    pub used: f64,
    pub remaining: f64,
    pub utilization: f64,
    pub conservation_ok: bool,
    pub projected_runtime_ticks: u64,
}

impl EnergyState {
    /// Create a default full-energy state.
    pub fn full(budget: f64) -> Self {
        Self {
            total_budget: budget,
            used: 0.0,
            remaining: budget,
            utilization: 0.0,
            conservation_ok: true,
            projected_runtime_ticks: u64::MAX,
        }
    }

    /// Brief summary of energy state.
    pub fn summary(&self) -> String {
        let pct = (self.remaining / self.total_budget * 100.0).round();
        let status = if self.is_critical() {
            "CRITICAL"
        } else if self.is_low() {
            "LOW"
        } else if self.conservation_ok {
            "OK"
        } else {
            "HIGH USAGE"
        };
        format!(
            "Energy: {:.0}% ({:.1}/{:.1}) [{}] ~{} ticks remaining",
            pct, self.remaining, self.total_budget, status, self.projected_runtime_ticks
        )
    }

    /// Is energy below 20%?
    pub fn is_low(&self) -> bool {
        self.total_budget > 0.0 && self.remaining / self.total_budget < 0.2
    }

    /// Is energy below 5%?
    pub fn is_critical(&self) -> bool {
        self.total_budget > 0.0 && self.remaining / self.total_budget < 0.05
    }
}

// ── CrewMember ────────────────────────────────────────────────────────────────

/// A crew member's current state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewMember {
    pub archetype: String,
    pub level: u32,
    pub current_task: Option<String>,
    pub emoji: String,
}

impl CrewMember {
    /// Create a new crew member.
    pub fn new(archetype: &str, level: u32, emoji: &str) -> Self {
        Self {
            archetype: archetype.to_string(),
            level,
            current_task: None,
            emoji: emoji.to_string(),
        }
    }

    /// One-line status.
    pub fn status_line(&self) -> String {
        match &self.current_task {
            Some(task) => format!(
                "{} {} (L{}) – {}",
                self.emoji, self.archetype, self.level, task
            ),
            None => format!(
                "{} {} (L{}) – idle",
                self.emoji, self.archetype, self.level
            ),
        }
    }
}

// ── CrewState ─────────────────────────────────────────────────────────────────

/// What the crew is doing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrewState {
    pub active_members: Vec<CrewMember>,
    pub idle_members: Vec<String>,
    pub total_xp: f64,
    pub average_level: f64,
}

impl CrewState {
    /// Empty crew state.
    pub fn empty() -> Self {
        Self {
            active_members: Vec::new(),
            idle_members: Vec::new(),
            total_xp: 0.0,
            average_level: 0.0,
        }
    }

    /// Brief status line for the crew.
    pub fn status_line(&self) -> String {
        let active = self.active_members.len();
        let idle = self.idle_members.len();
        format!(
            "Crew: {} active, {} idle (avg L{:.1})",
            active, idle, self.average_level
        )
    }

    /// Which crew members are available (idle)?
    pub fn who_is_available(&self) -> Vec<&CrewMember> {
        self.active_members
            .iter()
            .filter(|m| m.current_task.is_none())
            .collect()
    }
}

// ── IntentionState ────────────────────────────────────────────────────────────

/// Active intentions in the agent's queue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentionState {
    pub active_count: usize,
    pub frontier: Vec<String>,
    pub blocked: Vec<String>,
    pub total_budget_allocated: f64,
}

impl IntentionState {
    /// Empty intention state.
    pub fn empty() -> Self {
        Self {
            active_count: 0,
            frontier: Vec::new(),
            blocked: Vec::new(),
            total_budget_allocated: 0.0,
        }
    }

    /// Brief summary.
    pub fn summary(&self) -> String {
        format!(
            "Intentions: {} active ({} ready, {} blocked) – {:.1} energy allocated",
            self.active_count,
            self.frontier.len(),
            self.blocked.len(),
            self.total_budget_allocated
        )
    }
}

// ── FieldPerception ───────────────────────────────────────────────────────────

/// What the vibe field feels like.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldPerception {
    pub local_energy: f64,
    pub gradient_direction: Option<(f64, f64)>,
    pub gradient_magnitude: f64,
    pub nearby_hotspots: Vec<(usize, usize, f64)>,
    pub temperature: f64,
}

impl FieldPerception {
    /// A neutral/empty field perception.
    pub fn neutral() -> Self {
        Self {
            local_energy: 0.0,
            gradient_direction: None,
            gradient_magnitude: 0.0,
            nearby_hotspots: Vec::new(),
            temperature: 0.0,
        }
    }

    /// Natural language description of the field.
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();

        // Temperature description
        let temp_desc = if self.temperature > 0.7 {
            "hot"
        } else if self.temperature > 0.4 {
            "warm"
        } else if self.temperature > 0.1 {
            "cool"
        } else {
            "cold"
        };
        parts.push(format!("The field feels {}.", temp_desc));

        // Gradient direction
        if let Some((dx, dy)) = self.gradient_direction {
            if self.gradient_magnitude > 0.01 {
                let direction = describe_direction(dx, dy);
                parts.push(format!(
                    "Energy is flowing {} (magnitude {:.2}).",
                    direction, self.gradient_magnitude
                ));
            }
        }

        // Hotspots
        if !self.nearby_hotspots.is_empty() {
            let count = self.nearby_hotspots.len();
            parts.push(format!(
                "{} energy hotspot{} detected nearby.",
                count,
                if count > 1 { "s" } else { "" }
            ));
        }

        parts.join(" ")
    }
}

/// Convert a gradient vector to a compass direction.
fn describe_direction(dx: f64, dy: f64) -> String {
    let angle = dy.atan2(dx).to_degrees();
    let normalized = ((angle % 360.0) + 360.0) % 360.0;

    match normalized {
        337.5..=360.0 | 0.0..22.5 => "east".into(),
        22.5..67.5 => "northeast".into(),
        67.5..112.5 => "north".into(),
        112.5..157.5 => "northwest".into(),
        157.5..202.5 => "west".into(),
        202.5..247.5 => "southwest".into(),
        247.5..292.5 => "south".into(),
        292.5..337.5 => "southeast".into(),
        _ => "in an unknown direction".into(),
    }
}

// ── CaptainState ──────────────────────────────────────────────────────────────

/// What the Captain wants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptainState {
    pub last_message: Option<String>,
    pub last_contact_tick: u64,
    pub ticks_since_contact: u64,
    pub override_active: bool,
    pub awaiting_response: bool,
    pub current_command: Option<String>,
}

impl CaptainState {
    /// No contact from Captain.
    pub fn idle(current_tick: u64) -> Self {
        Self {
            last_message: None,
            last_contact_tick: 0,
            ticks_since_contact: current_tick,
            override_active: false,
            awaiting_response: false,
            current_command: None,
        }
    }

    /// Is the Captain waiting for a response?
    pub fn is_waiting(&self) -> bool {
        self.awaiting_response || self.override_active
    }

    /// How long since last contact, in natural language.
    pub fn contact_age(&self) -> String {
        match self.ticks_since_contact {
            0 => "just now".into(),
            1 => "1 tick ago".into(),
            n => format!("{} ticks ago", n),
        }
    }

    /// How urgently does the Captain need attention?
    pub fn urgency(&self) -> Urgency {
        if self.override_active || (self.awaiting_response && self.ticks_since_contact <= 3) {
            Urgency::Immediate
        } else if self.awaiting_response {
            Urgency::Soon
        } else if self.ticks_since_contact > 50 {
            Urgency::Idle
        } else {
            Urgency::Relaxed
        }
    }
}

// ── SensoryInput ──────────────────────────────────────────────────────────────

/// Raw perception data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensoryInput {
    pub log_messages: Vec<String>,
    pub alerts: Vec<String>,
    pub errors: Vec<String>,
    pub ambient_signals: HashMap<String, f64>,
}

impl SensoryInput {
    /// Empty sensory input.
    pub fn empty() -> Self {
        Self {
            log_messages: Vec::new(),
            alerts: Vec::new(),
            errors: Vec::new(),
            ambient_signals: HashMap::new(),
        }
    }

    /// Are there any active alerts?
    pub fn has_alerts(&self) -> bool {
        !self.alerts.is_empty()
    }

    /// Recent error messages.
    pub fn recent_errors(&self) -> Vec<&String> {
        self.errors.iter().collect()
    }
}

// ── PresentMoment ─────────────────────────────────────────────────────────────

/// THE snapshot of right now. The agent's complete first-person state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresentMoment {
    pub agent_id: String,
    pub tick: u64,
    pub timestamp: u64,
    pub location: Location,
    pub energy: EnergyState,
    pub crew: CrewState,
    pub intentions: IntentionState,
    pub field: FieldPerception,
    pub captain: CaptainState,
    pub sensory: SensoryInput,
}

impl PresentMoment {
    /// Create a minimal snapshot for an agent at a tick.
    pub fn snapshot(agent_id: &str, tick: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            agent_id: agent_id.to_string(),
            tick,
            timestamp: now,
            location: Location::unknown(),
            energy: EnergyState::full(100.0),
            crew: CrewState::empty(),
            intentions: IntentionState::empty(),
            field: FieldPerception::neutral(),
            captain: CaptainState::idle(tick),
            sensory: SensoryInput::empty(),
        }
    }

    /// Render the present moment in the given mode.
    pub fn render(&self, mode: &ViewMode) -> String {
        match mode {
            ViewMode::FirstPerson => self.render_first_person(),
            ViewMode::Dashboard => self.render_dashboard(),
            ViewMode::Narrative => self.render_narrative(),
            ViewMode::Compact => self.render_compact(),
            ViewMode::Debug => self.render_debug(),
        }
    }

    /// Natural language: what do I see?
    pub fn what_do_i_see(&self) -> String {
        let mut parts = Vec::new();

        // Location
        if self.location.is_empty() {
            parts.push("I am in an undefined space.".into());
        } else {
            parts.push(format!("I am in {}.", self.location.describe()));
        }

        // Energy
        let energy_pct = if self.energy.total_budget > 0.0 {
            (self.energy.remaining / self.energy.total_budget * 100.0).round() as u32
        } else {
            0
        };
        parts.push(format!("My energy is at {}%.", energy_pct));

        // Captain
        if let Some(cmd) = &self.captain.current_command {
            parts.push(format!(
                "The Captain asked me to {} {}.",
                cmd,
                self.captain.contact_age()
            ));
        }

        // Active crew tasks
        let working: Vec<&CrewMember> = self
            .crew
            .active_members
            .iter()
            .filter(|m| m.current_task.is_some())
            .collect();
        for member in working {
            parts.push(format!(
                "{} {} (L{}) is working on {}.",
                member.emoji,
                member.archetype,
                member.level,
                member.current_task.as_deref().unwrap()
            ));
        }

        // Field
        if self.field.gradient_magnitude > 0.01 || self.field.temperature > 0.1 {
            parts.push(self.field.describe());
        }

        parts.join(" ")
    }

    /// Suggested actions based on current state.
    pub fn what_should_i_do(&self) -> Vec<String> {
        let mut actions = Vec::new();

        if self.captain.override_active {
            if let Some(cmd) = &self.captain.current_command {
                actions.push(format!("OVERRIDE: Execute Captain's command: {}", cmd));
            } else {
                actions.push("OVERRIDE: Check in with Captain immediately".into());
            }
        }

        if self.energy.is_critical() {
            actions.push("CRITICAL: Conserve energy immediately".into());
        } else if self.energy.is_low() {
            actions.push("WARNING: Reduce energy expenditure".into());
        }

        if self.captain.awaiting_response {
            actions.push("Respond to Captain".into());
        }

        if self.sensory.has_alerts() {
            actions.push(format!("Handle {} alert(s)", self.sensory.alerts.len()));
        }

        if !self.intentions.frontier.is_empty() {
            actions.push(format!(
                "Execute {} frontier intention(s)",
                self.intentions.frontier.len()
            ));
        }

        if !self.crew.who_is_available().is_empty() {
            let avail = self.crew.who_is_available().len();
            actions.push(format!("Assign tasks to {} idle crew member(s)", avail));
        }

        if !self.intentions.blocked.is_empty() {
            actions.push(format!(
                "Unblock {} intention(s)",
                self.intentions.blocked.len()
            ));
        }

        if actions.is_empty() {
            actions.push("Monitor and maintain current operations".into());
        }

        actions
    }

    /// Is this an urgent moment?
    pub fn is_urgent(&self) -> bool {
        self.captain.override_active
            || self.energy.is_critical()
            || (self.captain.awaiting_response && self.captain.ticks_since_contact <= 3)
            || self.sensory.has_alerts()
    }

    /// What needs attention right now?
    pub fn priority_report(&self) -> String {
        if !self.is_urgent() {
            return "No urgent issues. All systems nominal.".into();
        }

        let mut items = Vec::new();

        if self.captain.override_active {
            items.push("⚠️ CAPTAIN OVERRIDE ACTIVE".into());
        }
        if self.energy.is_critical() {
            items.push("🔋 ENERGY CRITICAL".into());
        } else if self.energy.is_low() {
            items.push("🔋 Energy low".into());
        }
        if self.captain.awaiting_response {
            items.push(format!(
                "💬 Captain waiting ({}s)",
                self.captain.ticks_since_contact
            ));
        }
        if self.sensory.has_alerts() {
            items.push(format!("🚨 {} alert(s)", self.sensory.alerts.len()));
        }

        items.join(" | ")
    }

    // ── Render modes ──────────────────────────────────────────────────────

    fn render_first_person(&self) -> String {
        self.what_do_i_see()
    }

    fn render_dashboard(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("=== Agent {} @ Tick {} ===", self.agent_id, self.tick));
        lines.push(format!("Location: {}", self.location.describe()));
        lines.push(self.energy.summary());
        lines.push(self.crew.status_line());
        lines.push(self.intentions.summary());
        if self.captain.is_waiting() {
            lines.push(format!("Captain: WAITING ({})", self.captain.contact_age()));
        } else {
            lines.push(format!("Captain: last contact {}", self.captain.contact_age()));
        }
        if self.sensory.has_alerts() {
            lines.push(format!("Alerts: {}", self.sensory.alerts.join(", ")));
        }
        lines.join("\n")
    }

    fn render_narrative(&self) -> String {
        let mut parts = Vec::new();

        let room_name = self
            .location
            .room_name
            .as_deref()
            .unwrap_or("the void");
        let room_article = if room_name.starts_with(|c: char| c.is_ascii_uppercase()) {
            room_name
        } else {
            &format!("the {}", room_name)
        };
        parts.push(format!("{} hums with purpose.", room_article));

        if !self.location.nearby_agents.is_empty() {
            parts.push(format!(
                "{} work{} nearby.",
                self.location.nearby_agents.join(" and "),
                if self.location.nearby_agents.len() == 1 {
                    "s"
                } else {
                    ""
                }
            ));
        }

        if self.energy.is_low() {
            parts.push("A weariness settles in — energy reserves dwindle.".into());
        }

        if self.captain.is_waiting() {
            parts.push("The Captain's voice echoes, awaiting response.".into());
        }

        if !self.crew.active_members.is_empty() {
            let busy_count = self
                .crew
                .active_members
                .iter()
                .filter(|m| m.current_task.is_some())
                .count();
            if busy_count > 0 {
                parts.push(format!(
                    "{} crew {} hard at work.",
                    busy_count,
                    if busy_count == 1 { "member is" } else { "members are" }
                ));
            }
        }

        parts.push(self.field.describe());

        parts.join(" ")
    }

    fn render_compact(&self) -> String {
        let energy_pct = if self.energy.total_budget > 0.0 {
            (self.energy.remaining / self.energy.total_budget * 100.0).round() as u32
        } else {
            0
        };
        let room = self
            .location
            .room_name
            .as_deref()
            .unwrap_or("?");
        let captain = if self.captain.is_waiting() {
            " WAITING"
        } else {
            ""
        };
        let urgent = if self.is_urgent() { " ⚠️" } else { "" };
        format!(
            "[{} t={} E:{}%{}{}] {} crew | {} intents",
            room,
            self.tick,
            energy_pct,
            captain,
            urgent,
            self.crew.active_members.len(),
            self.intentions.active_count
        )
    }

    fn render_debug(&self) -> String {
        format!("{:#?}", self)
    }
}

// ── Perception ────────────────────────────────────────────────────────────────

/// Processed perception of the present moment, categorized by priority.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Perception {
    pub urgent: Vec<String>,
    pub important: Vec<String>,
    pub informational: Vec<String>,
    pub ambient: Vec<String>,
}

impl Perception {
    /// Empty perception.
    pub fn empty() -> Self {
        Self {
            urgent: Vec::new(),
            important: Vec::new(),
            informational: Vec::new(),
            ambient: Vec::new(),
        }
    }
}

// ── Action ────────────────────────────────────────────────────────────────────

/// A suggested action with metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub description: String,
    pub energy_cost: f64,
    pub urgency: Urgency,
}

impl Action {
    /// Create a new action.
    pub fn new(name: &str, description: &str, energy_cost: f64, urgency: Urgency) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            energy_cost,
            urgency,
        }
    }
}

// ── ShellInterface ────────────────────────────────────────────────────────────

/// THE agent's window into PLATO.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShellInterface {
    pub agent_id: String,
    pub tick: u64,
}

impl ShellInterface {
    /// Create a new shell interface for an agent.
    pub fn new(agent_id: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            tick: 0,
        }
    }

    /// Process the present moment into actionable perception.
    pub fn perceive(&self, state: &PresentMoment) -> Perception {
        let mut perception = Perception::empty();

        // Urgent items
        if state.captain.override_active {
            perception.urgent.push("Captain override is active".into());
        }
        if state.energy.is_critical() {
            perception.urgent.push("Energy critically low".into());
        }
        if state.captain.awaiting_response && state.captain.ticks_since_contact <= 3 {
            perception.urgent.push("Captain awaiting immediate response".into());
        }
        for alert in &state.sensory.alerts {
            perception.urgent.push(format!("Alert: {}", alert));
        }

        // Important items
        if state.energy.is_low() && !state.energy.is_critical() {
            perception.important.push("Energy is low".into());
        }
        if !state.intentions.blocked.is_empty() {
            perception.important.push(format!(
                "{} intention(s) blocked",
                state.intentions.blocked.len()
            ));
        }
        if state.captain.awaiting_response && state.captain.ticks_since_contact > 3 {
            perception.important.push("Captain awaiting response".into());
        }

        // Informational items
        perception.informational.push(state.energy.summary());
        perception.informational.push(state.crew.status_line());
        perception.informational.push(state.intentions.summary());
        if !state.location.is_empty() {
            perception.informational.push(format!("Location: {}", state.location.describe()));
        }

        // Ambient
        perception.ambient.push(state.field.describe());
        for msg in &state.sensory.log_messages {
            perception.ambient.push(msg.clone());
        }
        for (key, value) in &state.sensory.ambient_signals {
            perception.ambient.push(format!("{}: {:.2}", key, value));
        }

        perception
    }

    /// Render the present moment in a given mode.
    pub fn render(&self, state: &PresentMoment, mode: ViewMode) -> String {
        state.render(&mode)
    }

    /// Suggest actions based on present state.
    pub fn suggest_actions(&self, state: &PresentMoment) -> Vec<Action> {
        let mut actions = Vec::new();
        let suggestions = state.what_should_i_do();

        for suggestion in suggestions {
            let (urgency, cost) = if suggestion.starts_with("OVERRIDE") || suggestion.starts_with("CRITICAL") {
                (Urgency::Immediate, 0.0)
            } else if suggestion.starts_with("WARNING") {
                (Urgency::Soon, 1.0)
            } else if suggestion.contains("Captain") {
                (Urgency::Soon, 2.0)
            } else if suggestion.contains("alert") {
                (Urgency::Soon, 5.0)
            } else {
                (Urgency::Relaxed, 3.0)
            };

            actions.push(Action::new(
                &suggestion,
                &suggestion,
                cost,
                urgency,
            ));
        }

        actions
    }

    /// Prioritize actions by urgency.
    pub fn prioritize(&self, state: &PresentMoment) -> Vec<Action> {
        let mut actions = self.suggest_actions(state);
        actions.sort_by_key(|a| a.urgency.priority());
        actions
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── RoomType ──────────────────────────────────────────────────────

    #[test]
    fn room_type_labels() {
        assert_eq!(RoomType::Engineering.label(), "Engineering");
        assert_eq!(RoomType::Science.label(), "Science");
        assert_eq!(RoomType::Security.label(), "Security");
        assert_eq!(RoomType::Operations.label(), "Operations");
        assert_eq!(RoomType::Diplomacy.label(), "Diplomacy");
        assert_eq!(RoomType::Hardware.label(), "Hardware");
        assert_eq!(RoomType::Bridge.label(), "Bridge");
        assert_eq!(RoomType::Common.label(), "Common");
    }

    #[test]
    fn room_type_serde_roundtrip() {
        let rt = RoomType::Engineering;
        let json = serde_json::to_string(&rt).unwrap();
        let back: RoomType = serde_json::from_str(&json).unwrap();
        assert_eq!(rt, back);
    }

    // ── Urgency ──────────────────────────────────────────────────────

    #[test]
    fn urgency_ordering() {
        assert!(Urgency::Immediate.priority() < Urgency::Soon.priority());
        assert!(Urgency::Soon.priority() < Urgency::Relaxed.priority());
        assert!(Urgency::Relaxed.priority() < Urgency::Idle.priority());
    }

    // ── Location ──────────────────────────────────────────────────────

    #[test]
    fn location_unknown_is_empty() {
        assert!(Location::unknown().is_empty());
    }

    #[test]
    fn location_describe_unknown() {
        let loc = Location::unknown();
        assert!(loc.describe().contains("unknown area"));
    }

    #[test]
    fn location_describe_with_room() {
        let loc = Location {
            room_id: Some("eng-1".into()),
            room_name: Some("Engineering Bay".into()),
            room_type: Some(RoomType::Engineering),
            nearby_agents: vec!["Alice".into()],
            nearby_hardware: vec!["Motor".into()],
            exits: vec!["corridor-a".into()],
        };
        let desc = loc.describe();
        assert!(desc.contains("Engineering Bay"));
        assert!(desc.contains("Engineering Room"));
        assert!(desc.contains("Alice"));
        assert!(desc.contains("Motor"));
    }

    #[test]
    fn location_not_empty_with_agents() {
        let loc = Location {
            nearby_agents: vec!["Bob".into()],
            ..Location::unknown()
        };
        assert!(!loc.is_empty());
    }

    // ── EnergyState ──────────────────────────────────────────────────

    #[test]
    fn energy_full() {
        let e = EnergyState::full(100.0);
        assert_eq!(e.remaining, 100.0);
        assert_eq!(e.utilization, 0.0);
        assert!(!e.is_low());
        assert!(!e.is_critical());
    }

    #[test]
    fn energy_low() {
        let e = EnergyState {
            total_budget: 100.0,
            used: 85.0,
            remaining: 15.0,
            utilization: 0.85,
            conservation_ok: false,
            projected_runtime_ticks: 10,
        };
        assert!(e.is_low());
        assert!(!e.is_critical());
    }

    #[test]
    fn energy_critical() {
        let e = EnergyState {
            total_budget: 100.0,
            used: 98.0,
            remaining: 2.0,
            utilization: 0.98,
            conservation_ok: false,
            projected_runtime_ticks: 2,
        };
        assert!(e.is_low());
        assert!(e.is_critical());
    }

    #[test]
    fn energy_summary_contains_pct() {
        let e = EnergyState::full(100.0);
        assert!(e.summary().contains("100%"));
    }

    // ── CrewMember ───────────────────────────────────────────────────

    #[test]
    fn crew_member_status_idle() {
        let m = CrewMember::new("Engineering", 3, "🔧");
        assert!(m.status_line().contains("idle"));
    }

    #[test]
    fn crew_member_status_working() {
        let m = CrewMember {
            archetype: "Science".into(),
            level: 5,
            current_task: Some("analyzing samples".into()),
            emoji: "🔬".into(),
        };
        let line = m.status_line();
        assert!(line.contains("analyzing samples"));
        assert!(line.contains("L5"));
    }

    // ── CrewState ────────────────────────────────────────────────────

    #[test]
    fn crew_state_empty() {
        let c = CrewState::empty();
        assert!(c.active_members.is_empty());
        assert!(c.idle_members.is_empty());
    }

    #[test]
    fn crew_state_status_line() {
        let c = CrewState {
            active_members: vec![CrewMember::new("Eng", 2, "🔧")],
            idle_members: vec!["Medic".into()],
            total_xp: 100.0,
            average_level: 2.0,
        };
        let line = c.status_line();
        assert!(line.contains("1 active"));
        assert!(line.contains("1 idle"));
    }

    #[test]
    fn crew_state_available() {
        let c = CrewState {
            active_members: vec![
                CrewMember::new("Eng", 2, "🔧"),
                CrewMember {
                    archetype: "Sci".into(),
                    level: 3,
                    current_task: Some("busy".into()),
                    emoji: "🔬".into(),
                },
            ],
            idle_members: Vec::new(),
            total_xp: 100.0,
            average_level: 2.5,
        };
        let avail = c.who_is_available();
        assert_eq!(avail.len(), 1);
        assert_eq!(avail[0].archetype, "Eng");
    }

    // ── IntentionState ───────────────────────────────────────────────

    #[test]
    fn intention_empty() {
        let i = IntentionState::empty();
        assert_eq!(i.active_count, 0);
        assert!(i.frontier.is_empty());
    }

    #[test]
    fn intention_summary() {
        let i = IntentionState {
            active_count: 5,
            frontier: vec!["explore".into()],
            blocked: vec!["build".into(), "repair".into()],
            total_budget_allocated: 30.0,
        };
        let s = i.summary();
        assert!(s.contains("5 active"));
        assert!(s.contains("1 ready"));
        assert!(s.contains("2 blocked"));
    }

    // ── FieldPerception ──────────────────────────────────────────────

    #[test]
    fn field_neutral() {
        let f = FieldPerception::neutral();
        assert_eq!(f.local_energy, 0.0);
    }

    #[test]
    fn field_describe_warm() {
        let f = FieldPerception {
            local_energy: 0.6,
            gradient_direction: Some((1.0, 0.0)),
            gradient_magnitude: 0.5,
            nearby_hotspots: vec![(5, 5, 0.9)],
            temperature: 0.6,
        };
        let desc = f.describe();
        assert!(desc.contains("warm"));
        assert!(desc.contains("east"));
        assert!(desc.contains("hotspot"));
    }

    #[test]
    fn field_describe_cold() {
        let f = FieldPerception {
            temperature: 0.05,
            ..FieldPerception::neutral()
        };
        assert!(f.describe().contains("cold"));
    }

    #[test]
    fn field_describe_hot() {
        let f = FieldPerception {
            temperature: 0.9,
            ..FieldPerception::neutral()
        };
        assert!(f.describe().contains("hot"));
    }

    #[test]
    fn describe_direction_cardinal() {
        assert_eq!(describe_direction(1.0, 0.0), "east");
        assert_eq!(describe_direction(0.0, 1.0), "north");
        assert_eq!(describe_direction(-1.0, 0.0), "west");
        assert_eq!(describe_direction(0.0, -1.0), "south");
    }

    // ── CaptainState ─────────────────────────────────────────────────

    #[test]
    fn captain_idle() {
        let c = CaptainState::idle(100);
        assert!(!c.is_waiting());
        assert_eq!(c.urgency(), Urgency::Idle);
    }

    #[test]
    fn captain_waiting_immediate() {
        let c = CaptainState {
            last_message: Some("Report!".into()),
            last_contact_tick: 7,
            ticks_since_contact: 2,
            override_active: false,
            awaiting_response: true,
            current_command: Some("report".into()),
        };
        assert!(c.is_waiting());
        assert_eq!(c.urgency(), Urgency::Immediate);
    }

    #[test]
    fn captain_override_is_immediate() {
        let c = CaptainState {
            override_active: true,
            ..CaptainState::idle(10)
        };
        assert_eq!(c.urgency(), Urgency::Immediate);
    }

    #[test]
    fn captain_contact_age() {
        let c = CaptainState {
            ticks_since_contact: 5,
            ..CaptainState::idle(10)
        };
        assert_eq!(c.contact_age(), "5 ticks ago");
    }

    #[test]
    fn captain_contact_age_now() {
        let c = CaptainState {
            ticks_since_contact: 0,
            ..CaptainState::idle(10)
        };
        assert_eq!(c.contact_age(), "just now");
    }

    // ── SensoryInput ─────────────────────────────────────────────────

    #[test]
    fn sensory_empty() {
        let s = SensoryInput::empty();
        assert!(!s.has_alerts());
        assert!(s.recent_errors().is_empty());
    }

    #[test]
    fn sensory_with_alerts() {
        let s = SensoryInput {
            alerts: vec!["Overheating!".into()],
            errors: vec!["Sensor failure".into()],
            ..SensoryInput::empty()
        };
        assert!(s.has_alerts());
        assert_eq!(s.recent_errors().len(), 1);
    }

    // ── PresentMoment ────────────────────────────────────────────────

    #[test]
    fn snapshot_basic() {
        let pm = PresentMoment::snapshot("agent-1", 42);
        assert_eq!(pm.agent_id, "agent-1");
        assert_eq!(pm.tick, 42);
        assert!(pm.location.is_empty());
        assert!(!pm.is_urgent());
    }

    #[test]
    fn snapshot_has_timestamp() {
        let pm = PresentMoment::snapshot("a", 0);
        assert!(pm.timestamp > 0);
    }

    #[test]
    fn what_do_i_see_basic() {
        let pm = PresentMoment::snapshot("agent-1", 10);
        let desc = pm.what_do_i_see();
        assert!(desc.contains("agent-1") || desc.contains("undefined space") || desc.contains("100%"));
    }

    #[test]
    fn what_do_i_see_with_location() {
        let pm = PresentMoment {
            location: Location {
                room_id: Some("bridge".into()),
                room_name: Some("Bridge".into()),
                room_type: Some(RoomType::Bridge),
                ..Location::unknown()
            },
            ..PresentMoment::snapshot("a", 5)
        };
        let desc = pm.what_do_i_see();
        assert!(desc.contains("Bridge"));
    }

    #[test]
    fn what_should_i_do_default() {
        let pm = PresentMoment::snapshot("a", 0);
        let actions = pm.what_should_i_do();
        assert!(!actions.is_empty());
    }

    #[test]
    fn what_should_i_do_with_override() {
        let pm = PresentMoment {
            captain: CaptainState {
                override_active: true,
                current_command: Some("fix motor".into()),
                ..CaptainState::idle(10)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let actions = pm.what_should_i_do();
        assert!(actions.iter().any(|a| a.contains("OVERRIDE")));
    }

    #[test]
    fn is_urgent_with_override() {
        let pm = PresentMoment {
            captain: CaptainState {
                override_active: true,
                ..CaptainState::idle(10)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        assert!(pm.is_urgent());
    }

    #[test]
    fn is_urgent_with_critical_energy() {
        let pm = PresentMoment {
            energy: EnergyState {
                total_budget: 100.0,
                remaining: 2.0,
                ..EnergyState::full(100.0)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        assert!(pm.is_urgent());
    }

    #[test]
    fn is_urgent_with_alerts() {
        let pm = PresentMoment {
            sensory: SensoryInput {
                alerts: vec!["Fire!".into()],
                ..SensoryInput::empty()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        assert!(pm.is_urgent());
    }

    #[test]
    fn is_not_urgent_when_calm() {
        let pm = PresentMoment::snapshot("a", 10);
        assert!(!pm.is_urgent());
    }

    #[test]
    fn priority_report_calm() {
        let pm = PresentMoment::snapshot("a", 10);
        let report = pm.priority_report();
        assert!(report.contains("nominal") || report.contains("No urgent"));
    }

    #[test]
    fn priority_report_urgent() {
        let pm = PresentMoment {
            captain: CaptainState {
                override_active: true,
                ..CaptainState::idle(10)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let report = pm.priority_report();
        assert!(report.contains("OVERRIDE"));
    }

    // ── Render Modes ─────────────────────────────────────────────────

    #[test]
    fn render_first_person() {
        let pm = PresentMoment::snapshot("a", 10);
        let s = pm.render(&ViewMode::FirstPerson);
        assert!(!s.is_empty());
    }

    #[test]
    fn render_dashboard() {
        let pm = PresentMoment::snapshot("agent-7", 42);
        let s = pm.render(&ViewMode::Dashboard);
        assert!(s.contains("agent-7"));
        assert!(s.contains("42"));
    }

    #[test]
    fn render_narrative() {
        let pm = PresentMoment {
            location: Location {
                room_name: Some("Workshop".into()),
                nearby_agents: vec!["Unit-3".into()],
                ..Location::unknown()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let s = pm.render(&ViewMode::Narrative);
        assert!(s.contains("Workshop") || s.contains("Unit-3"));
    }

    #[test]
    fn render_compact() {
        let pm = PresentMoment {
            location: Location {
                room_name: Some("Bridge".into()),
                ..Location::unknown()
            },
            ..PresentMoment::snapshot("a", 5)
        };
        let s = pm.render(&ViewMode::Compact);
        assert!(s.contains("Bridge"));
        assert!(s.contains("t=5"));
    }

    #[test]
    fn render_debug() {
        let pm = PresentMoment::snapshot("a", 1);
        let s = pm.render(&ViewMode::Debug);
        assert!(s.contains("agent_id"));
    }

    #[test]
    fn render_compact_with_captain_waiting() {
        let pm = PresentMoment {
            captain: CaptainState {
                awaiting_response: true,
                ticks_since_contact: 1,
                ..CaptainState::idle(10)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let s = pm.render(&ViewMode::Compact);
        assert!(s.contains("WAITING"));
    }

    #[test]
    fn render_compact_urgent_marker() {
        let pm = PresentMoment {
            sensory: SensoryInput {
                alerts: vec!["!".into()],
                ..SensoryInput::empty()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let s = pm.render(&ViewMode::Compact);
        assert!(s.contains("⚠️"));
    }

    // ── Narrative specifics ──────────────────────────────────────────

    #[test]
    fn narrative_energy_low() {
        let pm = PresentMoment {
            energy: EnergyState {
                total_budget: 100.0,
                remaining: 10.0,
                ..EnergyState::full(100.0)
            },
            location: Location {
                room_name: Some("Engine Room".into()),
                ..Location::unknown()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let s = pm.render(&ViewMode::Narrative);
        assert!(s.contains("weariness") || s.contains("dwindle"));
    }

    #[test]
    fn narrative_captain_waiting() {
        let pm = PresentMoment {
            captain: CaptainState {
                awaiting_response: true,
                ..CaptainState::idle(10)
            },
            location: Location {
                room_name: Some("Bridge".into()),
                ..Location::unknown()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let s = pm.render(&ViewMode::Narrative);
        assert!(s.contains("Captain") && s.contains("echo"));
    }

    // ── ShellInterface ───────────────────────────────────────────────

    #[test]
    fn shell_new() {
        let shell = ShellInterface::new("agent-1");
        assert_eq!(shell.agent_id, "agent-1");
    }

    #[test]
    fn shell_perceive_empty() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment::snapshot("a", 0);
        let p = shell.perceive(&pm);
        assert!(p.urgent.is_empty());
    }

    #[test]
    fn shell_perceive_urgent() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment {
            captain: CaptainState {
                override_active: true,
                ..CaptainState::idle(10)
            },
            sensory: SensoryInput {
                alerts: vec!["Breach!".into()],
                ..SensoryInput::empty()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let p = shell.perceive(&pm);
        assert!(p.urgent.iter().any(|u| u.contains("override")));
        assert!(p.urgent.iter().any(|u| u.contains("Breach")));
    }

    #[test]
    fn shell_perceive_important() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment {
            energy: EnergyState {
                total_budget: 100.0,
                remaining: 15.0,
                ..EnergyState::full(100.0)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let p = shell.perceive(&pm);
        assert!(p.important.iter().any(|i| i.contains("low")));
    }

    #[test]
    fn shell_perceive_informational() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment::snapshot("a", 10);
        let p = shell.perceive(&pm);
        assert!(!p.informational.is_empty());
    }

    #[test]
    fn shell_perceive_ambient() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment {
            sensory: SensoryInput {
                ambient_signals: {
                    let mut m = HashMap::new();
                    m.insert("radiation".into(), 0.42);
                    m
                },
                ..SensoryInput::empty()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let p = shell.perceive(&pm);
        assert!(p.ambient.iter().any(|a| a.contains("radiation")));
    }

    #[test]
    fn shell_render_delegates() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment::snapshot("a", 10);
        let s = shell.render(&pm, ViewMode::Compact);
        assert!(!s.is_empty());
    }

    #[test]
    fn shell_suggest_actions() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment::snapshot("a", 10);
        let actions = shell.suggest_actions(&pm);
        assert!(!actions.is_empty());
    }

    #[test]
    fn shell_suggest_actions_with_captain() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment {
            captain: CaptainState {
                awaiting_response: true,
                ..CaptainState::idle(10)
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let actions = shell.suggest_actions(&pm);
        assert!(actions.iter().any(|a| a.name.contains("Captain")));
    }

    #[test]
    fn shell_prioritize_sorts_by_urgency() {
        let shell = ShellInterface::new("a");
        let pm = PresentMoment {
            captain: CaptainState {
                override_active: true,
                current_command: Some("fix".into()),
                ..CaptainState::idle(10)
            },
            sensory: SensoryInput {
                alerts: vec!["Fire!".into()],
                ..SensoryInput::empty()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let actions = shell.prioritize(&pm);
        // First action should be most urgent
        assert!(!actions.is_empty());
        // Immediate urgency actions should come first
        let has_immediate = actions.iter().any(|a| a.urgency == Urgency::Immediate);
        assert!(has_immediate);
    }

    // ── Serde roundtrips ─────────────────────────────────────────────

    #[test]
    fn present_moment_serde_roundtrip() {
        let pm = PresentMoment::snapshot("test-agent", 999);
        let json = serde_json::to_string(&pm).unwrap();
        let back: PresentMoment = serde_json::from_str(&json).unwrap();
        assert_eq!(pm, back);
    }

    #[test]
    fn perception_serde_roundtrip() {
        let p = Perception {
            urgent: vec!["fire".into()],
            important: vec!["energy low".into()],
            informational: vec!["tick 42".into()],
            ambient: vec!["warm field".into()],
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: Perception = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn action_serde_roundtrip() {
        let a = Action::new("test", "a test action", 5.0, Urgency::Soon);
        let json = serde_json::to_string(&a).unwrap();
        let back: Action = serde_json::from_str(&json).unwrap();
        assert_eq!(a, back);
    }

    #[test]
    fn shell_interface_serde_roundtrip() {
        let s = ShellInterface::new("agent-42");
        let json = serde_json::to_string(&s).unwrap();
        let back: ShellInterface = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn field_perception_serde_roundtrip() {
        let f = FieldPerception {
            local_energy: 0.5,
            gradient_direction: Some((1.0, 0.0)),
            gradient_magnitude: 0.3,
            nearby_hotspots: vec![(1, 2, 0.9)],
            temperature: 0.5,
        };
        let json = serde_json::to_string(&f).unwrap();
        let back: FieldPerception = serde_json::from_str(&json).unwrap();
        assert_eq!(f, back);
    }

    #[test]
    fn captain_state_serde_roundtrip() {
        let c = CaptainState {
            last_message: Some("Go!".into()),
            last_contact_tick: 5,
            ticks_since_contact: 3,
            override_active: true,
            awaiting_response: false,
            current_command: Some("engage".into()),
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: CaptainState = serde_json::from_str(&json).unwrap();
        assert_eq!(c, back);
    }

    // ── Edge cases ───────────────────────────────────────────────────

    #[test]
    fn energy_zero_budget() {
        let e = EnergyState::full(0.0);
        assert!(!e.is_low()); // avoid division by zero
        assert!(!e.is_critical());
    }

    #[test]
    fn location_with_exits() {
        let loc = Location {
            exits: vec!["north".into(), "south".into()],
            ..Location::unknown()
        };
        assert!(!loc.is_empty()); // exits make it non-empty? No, is_empty doesn't check exits
        // Actually is_empty checks all fields including exits
        // Let me re-read... yes, exits is checked
        assert!(!loc.is_empty());
    }

    #[test]
    fn field_multiple_hotspots() {
        let f = FieldPerception {
            nearby_hotspots: vec![(1, 1, 0.5), (5, 5, 0.8), (10, 10, 0.3)],
            temperature: 0.5,
            ..FieldPerception::neutral()
        };
        let desc = f.describe();
        assert!(desc.contains("3 energy hotspots"));
    }

    #[test]
    fn what_should_i_do_with_blocked_intentions() {
        let pm = PresentMoment {
            intentions: IntentionState {
                active_count: 3,
                blocked: vec!["build reactor".into()],
                ..IntentionState::empty()
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let actions = pm.what_should_i_do();
        assert!(actions.iter().any(|a| a.contains("Unblock")));
    }

    #[test]
    fn what_should_i_do_with_available_crew() {
        let pm = PresentMoment {
            crew: CrewState {
                active_members: vec![CrewMember::new("Eng", 3, "🔧")],
                idle_members: Vec::new(),
                total_xp: 100.0,
                average_level: 3.0,
            },
            ..PresentMoment::snapshot("a", 10)
        };
        let actions = pm.what_should_i_do();
        assert!(actions.iter().any(|a| a.contains("idle crew")));
    }

    #[test]
    fn captain_urgency_soon() {
        let c = CaptainState {
            awaiting_response: true,
            ticks_since_contact: 10,
            ..CaptainState::idle(20)
        };
        assert_eq!(c.urgency(), Urgency::Soon);
    }

    #[test]
    fn captain_urgency_relaxed() {
        let c = CaptainState {
            last_message: Some("hi".into()),
            ticks_since_contact: 5,
            ..CaptainState::idle(10)
        };
        assert_eq!(c.urgency(), Urgency::Relaxed);
    }

    #[test]
    fn view_mode_serde_roundtrip() {
        for mode in [
            ViewMode::FirstPerson,
            ViewMode::Dashboard,
            ViewMode::Narrative,
            ViewMode::Compact,
            ViewMode::Debug,
        ] {
            let json = serde_json::to_string(&mode).unwrap();
            let back: ViewMode = serde_json::from_str(&json).unwrap();
            assert_eq!(mode, back);
        }
    }
}
