# lau-shell-interface

**The present-moment interface — what the agent sees and feels inside PLATO.**

`lau-shell-interface` defines the first-person experience of an agent operating within the PLATO environment. It provides the complete sensory, cognitive, and social snapshot of *right now*: where the agent is, what energy it has, what the crew is doing, what the Captain wants, what the vibe field feels like, and what needs attention.

This is not the agent's brain or its kernel — it's the **interface**, the window through which the agent perceives and acts.

## Key Idea

An agent inside PLATO needs a structured, prioritized view of its current situation. `ShellInterface` takes a raw `PresentMoment` state and produces:

1. **Perception** — categorized by urgency (urgent / important / informational / ambient)
2. **Rendered views** — first-person narrative, dashboard, compact status, or debug dump
3. **Action suggestions** — prioritized by urgency with energy cost estimates

The data model mirrors a starship-like command structure:

| Concept | Type | Description |
|---|---|---|
| Location | `Location` | What room, who's nearby, what exits exist |
| Energy | `EnergyState` | Budget, remaining, utilization, projections |
| Crew | `CrewState` | Active/idle members, levels, current tasks |
| Intentions | `IntentionState` | Active goals, frontier (ready), blocked |
| Captain | `CaptainState` | Last message, override, awaiting response |
| Field | `FieldPerception` | Energy gradients, temperature, hotspots |
| Sensory | `SensoryInput` | Raw logs, alerts, errors, ambient signals |

Everything is serializable (`serde`), composable, and testable.

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
lau-shell-interface = "0.1.0"
```

Requires **Rust 2021 edition**. Dependencies:

- [`serde`](https://crates.io/crates/serde) 1 + [`serde_json`](https://crates.io/crates/serde_json) 1 — serialization

No other dependencies. This is a pure data-model crate.

## Quick Start

### Create a present-moment snapshot

```rust
use lau_shell_interface::{PresentMoment, Location, RoomType, ViewMode};

let moment = PresentMoment::snapshot("agent-7", 42);

println!("{}", moment.what_do_i_see());
println!("Urgent: {}", moment.is_urgent());
```

### Build a detailed moment

```rust
use lau_shell_interface::*;
use std::collections::HashMap;

let moment = PresentMoment {
    agent_id: "agent-7".into(),
    tick: 100,
    timestamp: 1700000000,
    location: Location {
        room_id: Some("bridge".into()),
        room_name: Some("Bridge".into()),
        room_type: Some(RoomType::Bridge),
        nearby_agents: vec!["Unit-3".into(), "Medic-1".into()],
        nearby_hardware: vec!["Console-A".into()],
        exits: vec!["corridor-north".into(), "turbolift".into()],
    },
    energy: EnergyState::full(100.0),
    crew: CrewState {
        active_members: vec![
            CrewMember::new("Engineering", 3, "🔧"),
            CrewMember {
                archetype: "Science".into(),
                level: 5,
                current_task: Some("analyzing samples".into()),
                emoji: "🔬".into(),
            },
        ],
        idle_members: vec!["Medic".into()],
        total_xp: 500.0,
        average_level: 4.0,
    },
    intentions: IntentionState {
        active_count: 3,
        frontier: vec!["explore sector 7".into()],
        blocked: vec!["build reactor".into()],
        total_budget_allocated: 30.0,
    },
    field: FieldPerception {
        local_energy: 0.6,
        gradient_direction: Some((1.0, 0.0)),
        gradient_magnitude: 0.5,
        nearby_hotspots: vec![(5, 5, 0.9)],
        temperature: 0.6,
    },
    captain: CaptainState {
        last_message: Some("Report status".into()),
        last_contact_tick: 95,
        ticks_since_contact: 5,
        override_active: false,
        awaiting_response: true,
        current_command: Some("report".into()),
    },
    sensory: SensoryInput {
        log_messages: vec!["System nominal".into()],
        alerts: vec![],
        errors: vec![],
        ambient_signals: {
            let mut m = HashMap::new();
            m.insert("radiation".into(), 0.12);
            m
        },
    },
};
```

### Render in different modes

```rust
// First-person: "I am in the Bridge. My energy is at 100%. The Captain asked me to report 5 ticks ago."
println!("{}", moment.render(&ViewMode::FirstPerson));

// Dashboard: structured summary
println!("{}", moment.render(&ViewMode::Dashboard));

// Narrative: "Bridge hums with purpose. Unit-3 and Medic-1 work nearby."
println!("{}", moment.render(&ViewMode::Narrative));

// Compact: "[Bridge t=100 E:100% WAITING] 2 crew | 3 intents"
println!("{}", moment.render(&ViewMode::Compact));

// Debug: full debug dump
println!("{}", moment.render(&ViewMode::Debug));
```

### Use the ShellInterface for perception and action

```rust
let shell = ShellInterface::new("agent-7");

// Categorized perception
let perception = shell.perceive(&moment);
for item in &perception.urgent {
    println!("🚨 {}", item);
}
for item in &perception.important {
    println!("⚠️ {}", item);
}
for item in &perception.informational {
    println!("ℹ️ {}", item);
}

// Suggested actions
let actions = shell.prioritize(&moment);
for action in &actions {
    println!("{} [cost={:.1}] {}", action.urgency.label(), action.energy_cost, action.description);
}
```

### Check urgency and priorities

```rust
if moment.is_urgent() {
    println!("{}", moment.priority_report());
    // "💬 Captain waiting (5s)" or "🔋 ENERGY CRITICAL" or "⚠️ CAPTAIN OVERRIDE ACTIVE"
}

// What should I do?
for suggestion in moment.what_should_i_do() {
    println!("→ {}", suggestion);
}
```

### Energy management

```rust
let energy = EnergyState {
    total_budget: 100.0,
    used: 85.0,
    remaining: 15.0,
    utilization: 0.85,
    conservation_ok: false,
    projected_runtime_ticks: 10,
};

println!("{}", energy.summary());
// "Energy: 15% (15.0/100.0) [LOW] ~10 ticks remaining"

println!("Low: {}, Critical: {}", energy.is_low(), energy.is_critical());
```

### Field perception

```rust
let field = FieldPerception {
    local_energy: 0.6,
    gradient_direction: Some((0.0, 1.0)),
    gradient_magnitude: 0.8,
    nearby_hotspots: vec![(3, 3, 0.95), (7, 7, 0.7)],
    temperature: 0.75,
};

println!("{}", field.describe());
// "The field feels hot. Energy is flowing north (magnitude 0.80). 2 energy hotspots detected nearby."
```

## API Reference

### Core Types

| Type | Description |
|---|---|
| `PresentMoment` | Complete snapshot of the agent's current state |
| `ShellInterface` | Processes PresentMoment into perception and actions |
| `Perception` | Categorized perception (urgent/important/informational/ambient) |
| `Action` | Suggested action with urgency and energy cost |
| `Location` | Where the agent is: room, type, nearby entities, exits |
| `EnergyState` | Energy budget, usage, projections |
| `CrewState` | Active/idle crew members |
| `CrewMember` | Individual crew member with archetype, level, task |
| `IntentionState` | Active intentions, frontier, blocked |
| `CaptainState` | Captain's last message, override status, urgency |
| `FieldPerception` | Energy field gradients, temperature, hotspots |
| `SensoryInput` | Raw logs, alerts, errors, ambient signals |

### Enums

| Enum | Description |
|---|---|
| `RoomType` | Engineering, Science, Security, Operations, Diplomacy, Hardware, Bridge, Common |
| `Urgency` | Immediate, Soon, Relaxed, Idle (with numeric priority) |
| `ViewMode` | FirstPerson, Dashboard, Narrative, Compact, Debug |

### PresentMoment Methods

| Method | Description |
|---|---|
| `.snapshot(agent_id, tick)` | Create minimal snapshot |
| `.what_do_i_see()` | Natural language description |
| `.what_should_i_do()` | Suggested action list |
| `.is_urgent()` | Check if anything needs immediate attention |
| `.priority_report()` | Urgency summary string |
| `.render(mode)` | Render in given ViewMode |

### ShellInterface Methods

| Method | Description |
|---|---|
| `.new(agent_id)` | Create interface |
| `.perceive(moment)` | Categorize into Perception |
| `.render(moment, mode)` | Render in given mode |
| `.suggest_actions(moment)` | Generate Action list |
| `.prioritize(moment)` | Actions sorted by urgency |

## How It Works

### Architecture

```
┌──────────────────────────────────────────────┐
│              PresentMoment                     │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐      │
│  │ Location  │ │ Energy   │ │ Crew     │      │
│  ├──────────┤ ├──────────┤ ├──────────┤      │
│  │ Intentions│ │ Captain  │ │ Field    │      │
│  ├──────────┤ ├──────────┤ ├──────────┤      │
│  │ Sensory  │ │          │ │          │      │
│  └──────────┘ └──────────┘ └──────────┘      │
└───────────────────┬──────────────────────────┘
                    │
                    ▼
┌──────────────────────────────────────────────┐
│            ShellInterface                      │
│                                               │
│  perceive() ──→ Perception                    │
│    urgent: [override, critical energy, alerts]│
│    important: [low energy, blocked, captain]  │
│    informational: [energy, crew, intents]     │
│    ambient: [field, logs, signals]            │
│                                               │
│  suggest_actions() ──→ [Action]              │
│  prioritize() ──→ sorted [Action]            │
│                                               │
│  render(mode) ──→ String                      │
└──────────────────────────────────────────────┘
```

### Perception Categorization

The `ShellInterface::perceive()` method categorizes observations:

**Urgent** (needs immediate attention):
- Captain override active
- Energy critically low (<5%)
- Captain awaiting immediate response (≤3 ticks)
- Active alerts

**Important** (needs attention soon):
- Energy low (<20%)
- Blocked intentions
- Captain awaiting response (>3 ticks)

**Informational** (good to know):
- Energy summary
- Crew status
- Intention summary
- Location

**Ambient** (background awareness):
- Field perception
- Log messages
- Ambient signals

### Action Prioritization

Actions are generated from the current state and sorted by `Urgency.priority()`:
- `Immediate` (0): Override commands, critical energy
- `Soon` (1): Captain response, alerts
- `Relaxed` (2): Frontier intentions, crew assignment
- `Idle` (3): Routine monitoring

### View Modes

| Mode | Use case |
|---|---|
| `FirstPerson` | Natural language "what do I see" — for agent reasoning |
| `Dashboard` | Structured status display — for monitoring |
| `Narrative` | Atmospheric prose — for storytelling/logs |
| `Compact` | Single-line status — for dashboards and logs |
| `Debug` | Full `{:?}` dump — for development |

## The Math

This is primarily a data-model crate with minimal mathematical content. The key quantitative elements:

### Energy Management

```
utilization = used / total_budget
remaining_pct = remaining / total_budget × 100
projected_runtime = remaining / (used / ticks_elapsed) × ticks_elapsed
```

Low threshold: 20%. Critical threshold: 5%.

### Field Gradient

Gradient direction is converted to compass directions via angle:

```
θ = atan2(dy, dx)
direction = compass_lookup(θ)
```

### Urgency Computation

```
Captain urgency:
  override_active OR (awaiting AND ticks ≤ 3) → Immediate
  awaiting AND ticks > 3 → Soon
  ticks > 50 → Idle
  else → Relaxed
```

### Serde

All types derive `Serialize` and `Deserialize`. Round-trip tests ensure serialization correctness for every type.

## Test Suite

**50+ tests** covering:

- RoomType labels and serde roundtrips
- Urgency ordering
- Location: unknown, described, empty checks
- EnergyState: full, low, critical, summaries, zero-budget edge case
- CrewMember: idle/working status lines
- CrewState: empty, available members, status lines
- IntentionState: summaries
- FieldPerception: descriptions (warm/cold/hot), gradient directions, hotspots
- CaptainState: urgency computation, contact age, override, waiting
- SensoryInput: alerts, errors
- PresentMoment: snapshots, what_do_i_see, what_should_i_do, is_urgent, priority_report
- Render modes: first-person, dashboard, narrative, compact, debug
- ShellInterface: perceive, suggest_actions, prioritize
- Serde roundtrips for all major types

Run with:

```bash
cargo test
```

## License

MIT
