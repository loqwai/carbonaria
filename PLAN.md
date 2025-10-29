# Carbonaria Bevy ECS Refactoring Plan

## Executive Summary

This document outlines a comprehensive refactoring of the Carbonaria roguelike game to:
1. Simplify over-engineered patterns while maintaining flexibility for multiplayer/replay
2. Improve code organization and clarity
3. Update to latest Bevy and dependencies
4. Add comprehensive testing infrastructure
5. Provide debug capabilities for isolated system execution

**Estimated Effort:** 3-4 hours
**Risk Level:** Low (architecture improvements, no gameplay changes)
**Target Bevy Version:** Latest stable (0.15+)

---

## Current Architecture Analysis

### Overview

**Entity Count:** 67 individual system files
**Key Patterns:**
- Event-driven architecture (MoveEvent, ShootEvent, DamageEvent, etc.)
- Generic powerup system using `Math<T>` with entity hierarchies
- Fixed 60 FPS timestep for deterministic gameplay
- System sets with explicit dependencies
- Bundle pattern for entity composition

### Directory Structure

```
src/
├── main.rs                    # App setup, system scheduling (5 system sets)
├── bundles/                   # Entity bundle definitions (9 files)
├── components/                # Component definitions (2 files)
├── events/                    # Custom event types (1 file)
├── resources/                 # Global resources (2 files)
└── systems/                   # Game logic (67 files) ← PRIMARY REFACTOR TARGET
```

### System Execution Flow

```
Frame N:
  UI_SYSTEM_SET (every frame)
    ├── update_compass, update_score_ui, update_health_ui
    ├── follow_player_with_camera
    ├── sprite animations
    └── sync_mouse_position

  COMPUTE_POWERUPS_SYSTEM_SET (60 FPS fixed)
    ├── powerup_defaulter<Speed>     → Reset stats to defaults
    ├── powerup_mather<Speed>        → Apply Math<T> modifiers
    ├── (repeated for Health, RateOfFire, TimeToLive, Poison, AmmoCount)

  GAME_LOOP_SYSTEM_SET (60 FPS fixed, AFTER powerups)
    ├── count_ticks
    ├── Movement Phase:
    │   ├── move_player            → Emit MoveEvent from input
    │   ├── move_bullet            → Emit MoveEvent for bullets
    │   ├── move_thing             → Apply Speed to Transform
    ├── Rotation Phase:
    │   ├── player_aimables_aim_at_cursor
    │   ├── chaser_aimables_aim_at_other_teams
    │   ├── rotate_thing           → Apply RotateEvent
    ├── Shooting Phase:
    │   ├── on_left_click_shoot    → Emit ShootEvent
    │   ├── mob_shoot
    │   ├── shoot_gun              → Create bullets
    ├── Combat/Effects:
    │   ├── poison
    │   ├── on_0_health_kill
    ├── Spawning:
    │   ├── spawn_mobs, spawn_mechs, spawn_powerups
    └── TTL/Duration:
        ├── time_to_live

  GAME_LOOP_CLEANUP_SYSTEM_SET (AFTER game loop)
    └── consume_despawn_entity_events
```

---

## Issues Identified

### 1. ⚠️ Over-Engineering: Math<T> Powerup System

**Current Implementation:**
```rust
// To decrement TTL by 1, a child entity is spawned!
commands.entity(entity).with_children(|parent| {
    parent.spawn(Math::add(TimeToLive(-1)));
});

// Later, powerup_mather system queries all children and applies modifications
```

**Problem:**
- Spawning entities for arithmetic operations is excessive
- Creates query complexity (Parent/Children traversal)
- Difficult to debug (where is my -1 coming from?)
- Entity cleanup overhead

**Justification for Math<T>:**
- Avoids mutation conflicts when multiple systems modify same stat
- Composable modifiers (stack multiple powerups)
- Generic implementation reduces code duplication

**Solution:**
- **Keep** Math<T> for *persistent modifiers* (equipped items, active powerups)
- **Replace** with direct mutations for *transient operations* (TTL countdown, tick counters)

### 2. ⚠️ Deep Entity Hierarchies

**Current Pattern:**
```
Player Entity
├── PlayerModel (3D visual)
├── Compass (UI component)
├── HealthUI
├── LaserGun Entity
│   └── Math::add(AmmoCount(5))    // Gun's ammo modifier
├── Math::add(AmmoCount(5))        // Player's base ammo
├── Math::add(Speed(7.5))          // Speed modifier
└── Math::add(Health(750.0))       // Health modifier
```

**Issues:**
- Hard to query ("which entity is the gun?")
- Cleanup complexity (orphaned children?)
- Not clear which modifiers belong where
- Difficult to inspect in debugger

**Solution:**
- Flatten where possible
- Use marker components to identify children
- Document hierarchy pattern clearly

### 3. ⚠️ 67 System Files - Too Granular

**Examples of trivial systems:**
- `count_ticks.rs` (3 lines) - increments frame counter
- `sync_mouse_position.rs` (5 lines) - updates mouse resource
- `on_f_key_switch_ammo.rs` (10 lines) - handles one key

**Problems:**
- High cognitive overhead (where is system X?)
- Difficult to see related logic together
- IDE navigation tedium

**Solution:**
Group into 15 logical modules based on domain

### 4. ✅ Event-Driven Architecture - KEEP

**Pattern:**
```rust
// Frame N: Emit events
move_events.send(MoveEvent { who: player, direction });

// Frame N or N+1: Apply events
transform.translation += direction * speed;
```

**Why it's good:**
- Deterministic for multiplayer/replay (user confirmed this requirement)
- Decouples input from physics
- Enables event logging/debugging
- Allows multiple systems to emit movement

**Keep this pattern** - it's justified for your use case.

### 5. ❌ Missing: Test Infrastructure

**Current State:** No visible test files

**Needed:**
- Unit tests for core systems
- Integration tests for gameplay scenarios
- Test helpers for entity spawning
- Mock input injection
- Deterministic RNG for tests

### 6. ❌ Missing: Debug Capabilities

**Current State:** No way to run systems in isolation

**Needed:**
- CLI flags to enable/disable system sets
- Single-step system execution
- Entity inspector
- Event logging
- System performance profiling

---

## Refactoring Plan

### Phase 1: Foundation & Updates

#### Task 1.1: Create Git Branch
```bash
git checkout -b refactor/ecs-simplification
```

#### Task 1.2: Update Dependencies
Update `Cargo.toml` to latest stable versions:
- Bevy: 0.15+ (or latest)
- bevy_rapier2d/3d
- clap
- rand

**Commit:** "Update dependencies to latest versions"

#### Task 1.3: Fix Breaking Changes
Address any API changes from Bevy updates:
- System parameter changes
- Query API updates
- Transform/GlobalTransform changes
- Asset loading API
- Plugin trait changes

**Commit:** "Fix breaking changes from Bevy 0.15 update"

---

### Phase 2: Testing Infrastructure

#### Task 2.1: Create Test Module Structure
```
tests/
├── common/
│   ├── mod.rs              # Test helpers
│   ├── builders.rs         # Entity builders for tests
│   └── assertions.rs       # Custom assertions
├── systems/
│   ├── movement_test.rs
│   ├── combat_test.rs
│   ├── powerup_test.rs
│   └── spawning_test.rs
└── integration/
    ├── gameplay_test.rs
    └── determinism_test.rs
```

#### Task 2.2: Test Helpers
Create test utilities:
```rust
// tests/common/mod.rs
pub struct TestApp {
    app: App,
}

impl TestApp {
    pub fn new() -> Self {
        let mut app = App::new();
        // Add minimal plugins for testing
        app.add_plugins(MinimalPlugins);
        Self { app }
    }

    pub fn spawn_player(&mut self) -> Entity { /* ... */ }
    pub fn spawn_mob(&mut self) -> Entity { /* ... */ }
    pub fn run_system<S: System>(&mut self, system: S) { /* ... */ }
    pub fn tick(&mut self, frames: usize) { /* ... */ }
}
```

#### Task 2.3: Example Test Coverage
- Movement system tests (verify events → transform updates)
- Combat tests (damage calculation, death)
- Powerup tests (stat modification correctness)
- Spawning tests (interval logic, entity counts)
- Determinism tests (same input → same output)

**Commit:** "Add comprehensive testing infrastructure"

---

### Phase 3: Debug Mode

#### Task 3.1: Add Debug CLI Flags
Extend `Config` resource:
```rust
#[derive(Resource, Parser)]
pub struct Config {
    // ... existing fields

    #[arg(long)]
    pub debug_mode: bool,

    #[arg(long)]
    pub disable_systems: Vec<String>,  // e.g., --disable-systems spawning,ai

    #[arg(long)]
    pub step_mode: bool,  // Pause after each system

    #[arg(long)]
    pub log_events: bool,  // Print all events to console
}
```

#### Task 3.2: System Isolation
Create debug plugin:
```rust
// src/debug.rs
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world.resource::<Config>();

        if config.debug_mode {
            app.add_systems(Update, (
                log_events_system,
                entity_inspector_system,
                system_profiler,
            ));
        }

        if config.step_mode {
            app.add_systems(Update, step_mode_controller);
        }
    }
}
```

#### Task 3.3: Entity Inspector
Add runtime entity inspection:
```rust
// Press 'I' to toggle inspector
fn entity_inspector_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &Name, &Transform, &Health)>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        for (entity, name, transform, health) in &query {
            println!("{:?}: {} at {:?}, HP: {}",
                entity, name, transform.translation, health.0);
        }
    }
}
```

**Commit:** "Add debug mode with system isolation and entity inspection"

---

### Phase 4: System Organization

#### Task 4.1: Create Module Structure
```
src/systems/
├── mod.rs                 # Public exports, system set definitions
├── input.rs               # All keyboard/mouse input handlers
├── movement.rs            # move_player, move_thing, move_bullet, chasers
├── combat.rs              # shooting, damage, death, collision
├── spawning.rs            # spawn_player, spawn_mobs, spawn_mechs, spawn_powerups
├── powerups.rs            # powerup_defaulter, powerup_mather, pickup logic
├── effects.rs             # poison, time_to_live, attach_* systems
├── ai.rs                  # chaser logic, aiming systems
├── ui.rs                  # update_compass, update_health_ui, update_score_ui
├── animation.rs           # sprite animations, 3D animations
├── physics.rs             # rotate_thing, collision handling
└── lifecycle.rs           # count_ticks, despawn_entity, cleanup
```

#### Task 4.2: Consolidation Strategy
For each module:
1. Group related systems
2. Add module-level documentation
3. Keep systems as separate functions (testability)
4. Update imports in main.rs

**Example (movement.rs):**
```rust
//! Movement systems handling player input, AI pathfinding, and physics application.
//!
//! Execution order:
//! 1. move_player - Reads keyboard input, emits MoveEvent
//! 2. move_bullet - Physics simulation for projectiles
//! 3. chasers_follow_other_teams - AI pathfinding
//! 4. move_thing - Applies Speed component to Transform based on MoveEvent

use bevy::prelude::*;
use crate::{components::*, events::*};

/// Reads WASD input and emits MoveEvent for player entity
pub fn move_player(/* ... */) { /* ... */ }

/// Applies velocity physics to bullet entities
pub fn move_bullet(/* ... */) { /* ... */ }

/// AI system: entities with Chases component pursue other teams
pub fn chasers_follow_other_teams(/* ... */) { /* ... */ }

/// Core movement application: MoveEvent + Speed → Transform update
pub fn move_thing(/* ... */) { /* ... */ }
```

**Commits:** One per module (e.g., "Consolidate movement systems")

---

### Phase 5: Simplify Powerup System

#### Task 5.1: Refactor Transient Operations
**Target systems:**
- `time_to_live.rs`
- `poison.rs`
- `count_ticks.rs`

**Before:**
```rust
// time_to_live.rs
pub fn time_to_live(mut commands: Commands, q: Query<(Entity, &TimeToLive)>) {
    for (entity, ttl) in &q {
        // Spawn child entity to decrement!
        commands.entity(entity).with_children(|parent| {
            parent.spawn(Math::add(TimeToLive(-1)));
        });

        if ttl.0 <= 0 {
            despawn_events.send(DespawnEvent { entity });
        }
    }
}
```

**After:**
```rust
// effects.rs
pub fn time_to_live(
    mut commands: Commands,
    mut q: Query<(Entity, &mut TimeToLive)>,
    mut despawn_events: EventWriter<DespawnEvent>,
) {
    for (entity, mut ttl) in &mut q {
        ttl.0 -= 1;  // Direct mutation

        if ttl.0 <= 0 {
            despawn_events.send(DespawnEvent { entity });
        }
    }
}
```

**Rationale:**
- TTL is never modified by multiple systems in same frame
- No mutation conflicts possible
- Clearer intent
- Better performance (no entity spawn/despawn)

#### Task 5.2: Document Math<T> Usage
Add clear guidelines:
```rust
// src/components/powerups.rs

/// Generic component for applying stat modifications via entity hierarchy.
///
/// # Usage
///
/// Use Math<T> for PERSISTENT modifiers that:
/// - Come from pickups/powerups
/// - Stack with other effects
/// - May be removed dynamically (despawn the modifier entity)
///
/// Example:
/// ```rust
/// // Player picks up speed powerup
/// commands.entity(player).with_children(|parent| {
///     parent.spawn((
///         Math::multiply(Speed(1.5)),  // 50% speed boost
///         TimeToLive(600),             // Lasts 10 seconds
///         Name::new("SpeedBoost"),
///     ));
/// });
/// ```
///
/// # When NOT to use Math<T>
///
/// Do NOT use for transient operations like:
/// - Frame counters (use direct mutation)
/// - Tick-based decrements (use `&mut TimeToLive`)
/// - One-time calculations (compute directly)
```

**Commit:** "Simplify powerup system for transient operations"

---

### Phase 6: Flatten Entity Hierarchies

#### Task 6.1: Add Marker Components
```rust
// src/components/mod.rs

/// Marker component for identifying player's weapon child entity
#[derive(Component)]
pub struct PlayerWeapon;

/// Marker component for player's 3D visual representation
#[derive(Component)]
pub struct PlayerModel;

/// Marker component for UI elements attached to entities
#[derive(Component)]
pub struct EntityUI;
```

#### Task 6.2: Document Hierarchy
Add ASCII diagram to bundle files:
```rust
// src/bundles/player.rs

/// Player entity hierarchy:
/// ```
/// Player (PlayerBundle)
/// ├── PlayerModel (PlayerModelBundle) - 3D visual, syncs with parent transform
/// ├── Compass (CompassBundle, EntityUI) - Minimap UI element
/// ├── HealthUI (HealthBundle, EntityUI) - Health bar
/// └── LaserGun (LaserGunBundle, PlayerWeapon) - Weapon with independent cooldown
///     └── AmmoModifier (Math<AmmoCount>) - Magazine size upgrade
/// ```
pub struct PlayerBundle { /* ... */ }
```

#### Task 6.3: Attach Modifiers Directly
Move stat modifiers from children to player entity:
```rust
// Before: nested children
commands.spawn(PlayerBundle::new())
    .with_children(|parent| {
        parent.spawn(Math::add(Speed(7.5)));
        parent.spawn(Math::add(Health(750.0)));
    });

// After: direct components
commands.spawn((
    PlayerBundle::new(),
    StatModifiers {
        speed: Math::add(Speed(7.5)),
        health: Math::add(Health(750.0)),
    },
));
```

**Commit:** "Flatten entity hierarchies and add marker components"

---

### Phase 7: Documentation

#### Task 7.1: Architecture Guide
Create `docs/ARCHITECTURE.md`:
- ECS pattern overview
- System execution flow diagram
- Event-driven architecture rationale
- Math<T> pattern explanation
- Entity hierarchy patterns
- Testing guidelines

#### Task 7.2: System Documentation
Add module-level docs to each `systems/*.rs` file:
- Purpose of systems in module
- Execution order requirements
- Dependencies on other systems
- Example usage

#### Task 7.3: Developer Guide
Create `docs/DEVELOPMENT.md`:
- How to add new systems
- How to add new components
- How to write tests
- How to use debug mode
- Common patterns and anti-patterns

**Commit:** "Add comprehensive architecture documentation"

---

### Phase 8: Final Cleanup

#### Task 8.1: Run Tests
```bash
cargo test --all
```

#### Task 8.2: Run Game
```bash
cargo run --release
```

Verify:
- Game runs without errors
- Gameplay is unchanged
- Performance is same or better

#### Task 8.3: Run with Debug Mode
```bash
cargo run -- --debug-mode --log-events
```

Verify debug features work.

#### Task 8.4: Clean Up Dead Code
- Remove unused imports
- Remove commented code
- Run clippy: `cargo clippy -- -W clippy::all`

**Commit:** "Final cleanup and verification"

---

## Testing Strategy

### Unit Tests

Test individual systems in isolation:

```rust
// tests/systems/movement_test.rs
#[test]
fn test_move_player_emits_event() {
    let mut app = TestApp::new();
    let player = app.spawn_player();

    // Simulate WASD input
    app.press_key(KeyCode::KeyW);
    app.run_system(move_player);

    let events = app.read_events::<MoveEvent>();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].who, player);
    assert_eq!(events[0].direction, Vec3::Y);
}

#[test]
fn test_move_thing_applies_speed() {
    let mut app = TestApp::new();
    let entity = app.world.spawn((
        Transform::default(),
        Speed(10.0),
    )).id();

    app.send_event(MoveEvent {
        who: entity,
        direction: Vec3::X,
    });

    app.run_system(move_thing);

    let transform = app.world.get::<Transform>(entity).unwrap();
    assert_eq!(transform.translation.x, 10.0);
}
```

### Integration Tests

Test full gameplay scenarios:

```rust
// tests/integration/gameplay_test.rs
#[test]
fn test_player_shoots_and_kills_mob() {
    let mut app = TestApp::new();
    let player = app.spawn_player();
    let mob = app.spawn_mob_at(Vec3::new(100.0, 0.0, 0.0));

    // Aim at mob and shoot
    app.set_cursor_position(Vec2::new(100.0, 0.0));
    app.press_mouse_button(MouseButton::Left);

    // Run game loop until bullet hits
    for _ in 0..60 {
        app.tick(1);
        if app.world.get::<Health>(mob).unwrap().0 <= 0.0 {
            break;
        }
    }

    // Mob should be dead
    assert!(!app.entity_exists(mob));

    // Player should have score
    let score = app.world.get::<Points>(player).unwrap();
    assert!(score.0 > 0);
}
```

### Determinism Tests

Verify replay capability:

```rust
// tests/integration/determinism_test.rs
#[test]
fn test_deterministic_simulation() {
    let input_sequence = vec![
        Input::KeyPress(KeyCode::KeyW),
        Input::MouseClick(Vec2::new(50.0, 50.0)),
        Input::KeyPress(KeyCode::KeyD),
    ];

    let result1 = run_simulation(&input_sequence, 100);
    let result2 = run_simulation(&input_sequence, 100);

    assert_eq!(result1.player_position, result2.player_position);
    assert_eq!(result1.mob_count, result2.mob_count);
    assert_eq!(result1.score, result2.score);
}
```

---

## Debug Mode Usage

### CLI Examples

```bash
# Enable debug mode with event logging
cargo run -- --debug-mode --log-events

# Disable AI and spawning systems for testing player mechanics
cargo run -- --disable-systems ai,spawning

# Step mode: press Space to advance one frame at a time
cargo run -- --step-mode

# Combine flags
cargo run -- --debug-mode --step-mode --disable-systems ai
```

### Runtime Commands

When debug mode is enabled:

- **I** - Toggle entity inspector (prints all entities to console)
- **P** - Toggle system profiler (shows system execution times)
- **E** - Dump event queue
- **Space** - Advance one frame (in step mode)
- **F3** - Toggle on-screen debug overlay

---

## Success Criteria

### Functional Requirements
- [ ] All existing gameplay works identically
- [ ] Game runs at same or better performance
- [ ] No visual regressions
- [ ] Deterministic behavior maintained (same inputs → same outputs)

### Code Quality
- [ ] System files consolidated (67 → ~15 modules)
- [ ] All modules have documentation
- [ ] Math<T> usage clarified and simplified
- [ ] Entity hierarchies documented with diagrams

### Testing
- [ ] ≥80% test coverage for core systems
- [ ] All tests pass
- [ ] Determinism tests verify replay capability

### Debug Capabilities
- [ ] Can run with `--debug-mode` flag
- [ ] Can disable system sets via CLI
- [ ] Entity inspector works
- [ ] Event logging works

### Documentation
- [ ] ARCHITECTURE.md complete
- [ ] DEVELOPMENT.md complete
- [ ] All modules have doc comments
- [ ] Onboarding guide for new contributors

---

## Migration Notes

### Breaking Changes to Expect

**Bevy 0.15 (if applicable):**
- `Transform` component changes
- System parameter ordering
- Asset loading API updates
- Plugin trait modifications

**Dependency Updates:**
- `bevy_rapier` API changes
- `clap` v4+ derive macro changes

### Rollback Plan

If refactoring introduces bugs:
1. Branch is separate from main
2. All changes are committed incrementally
3. Can cherry-pick specific improvements
4. Can abandon branch and keep lessons learned

---

## Post-Refactoring Roadmap

### Future Enhancements
1. **Replay System** - Record input events, play back deterministically
2. **Network Multiplayer** - Event-driven architecture enables this
3. **Modding Support** - Plugin system for custom systems/components
4. **Visual Entity Debugger** - GUI tool like Unity inspector
5. **Performance Profiling** - Identify bottleneck systems

### Technical Debt Addressed
- ✅ System organization
- ✅ Testing infrastructure
- ✅ Debug capabilities
- ✅ Documentation
- ✅ Over-engineering simplified

---

## Timeline

**Total Estimated Time:** 3-4 hours

| Phase | Tasks | Est. Time | Commits |
|-------|-------|-----------|---------|
| 1. Foundation | Branch, deps, breaking changes | 30 min | 2 |
| 2. Testing | Infrastructure, helpers, example tests | 45 min | 3 |
| 3. Debug Mode | CLI flags, inspector, isolation | 30 min | 2 |
| 4. System Org | Consolidate 67 → 15 modules | 60 min | 15 |
| 5. Powerups | Simplify Math<T>, document | 20 min | 2 |
| 6. Hierarchies | Flatten, markers, docs | 20 min | 1 |
| 7. Documentation | Guides, module docs | 30 min | 1 |
| 8. Cleanup | Tests, verification | 15 min | 1 |

**Total Commits:** ~27

---

## Conclusion

This refactoring will transform Carbonaria from a functional but over-engineered codebase into a well-organized, testable, and maintainable Bevy project. The event-driven architecture is preserved (necessary for multiplayer/replay), but unnecessary complexity is removed. Comprehensive testing and debug tools will make future development faster and more confident.

**Key Principles:**
1. **Simplify without sacrificing flexibility**
2. **Document patterns clearly**
3. **Test everything**
4. **Make debugging easy**
5. **Commit frequently**

Let's build something great! 🚀
