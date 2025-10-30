# Continuation Guide - How to Resume the Migration

This guide provides step-by-step instructions to continue the Bevy 0.14 migration from where we left off.

---

## Current Status

✅ **Completed:**
- Dependencies updated (Bevy 0.9 → 0.14)
- Input API migrated
- Window API migrated
- Event system migrated
- State system migrated
- Ray API migrated

🔴 **Remaining:**
- Sprite system migration (CRITICAL)
- System registration API (CRITICAL)
- Query API fixes
- Text API fixes
- Asset loading fixes

**Errors:** 74 compilation errors
**Branch:** `refactor/ecs-simplification`
**Last Commit:** Event and States derive macros

---

## Prerequisites

1. **Pull latest changes:**
   ```bash
   cd /path/to/carbonaria
   git checkout refactor/ecs-simplification
   git pull origin refactor/ecs-simplification
   ```

2. **Verify Rust version:**
   ```bash
   rustc --version
   # Should show: rustc 1.90.0 or later
   ```

3. **Check current errors:**
   ```bash
   cargo build 2>&1 | grep "^error\[" | wc -l
   # Should show: ~74 errors
   ```

4. **Read documentation:**
   - `docs/MIGRATION_STATUS.md` - Overview
   - `docs/BREAKING_CHANGES_REMAINING.md` - Detailed API changes

---

## Step-by-Step Migration

### Phase 1: Sprite System Migration (90 minutes)

This is the MOST CRITICAL and MOST COMPLEX migration. Take your time.

#### Task 1.1: Fix Player Bundle (15 min)

**File:** `src/bundles/player.rs`

1. **Remove `SpriteSheetBundle` from bundle:**
   ```rust
   // REMOVE this field:
   pub sprite_sheet_bundle: SpriteSheetBundle,
   ```

2. **Update constructor to NOT return sprite components:**
   The bundle should only contain gameplay components, not rendering.

3. **Create a separate spawn helper:**
   ```rust
   impl PlayerBundle {
       pub fn spawn(
           commands: &mut Commands,
           texture: Handle<Image>,
           atlas_layout: Handle<TextureAtlasLayout>,
           position: Vec3,
       ) -> Entity {
           commands.spawn((
               Self::new(),  // Game components only
               SpriteBundle {
                   sprite: Sprite {
                       custom_size: Some(Vec2::new(48.0, 48.0)),
                       ..default()
                   },
                   texture,
                   transform: Transform::from_xyz(position.x, position.y, 10.0),
                   ..default()
               },
               TextureAtlas {
                   index: 0,
                   layout: atlas_layout,
               },
           ))
           .id()
       }
   }
   ```

4. **Compile and check:**
   ```bash
   cargo build 2>&1 | grep "player.rs"
   ```

#### Task 1.2: Fix Mob, Mech, Bullet, Chest Bundles (30 min)

Repeat the same pattern for:
- `src/bundles/mob.rs`
- `src/bundles/mech.rs`
- `src/bundles/bullet.rs`
- `src/bundles/chest.rs`

**For each file:**
1. Remove `SpriteSheetBundle` field
2. Add `spawn()` helper method
3. Adjust sprite sizes appropriately

**Quick reference:**
- Player: 48x48
- Mob: 32x32
- Mech: 64x64
- Bullet: 8x8
- Chest: 32x32

#### Task 1.3: Update spawn_player System (15 min)

**File:** `src/systems/spawn_player.rs`

1. **Change resource type:**
   ```rust
   // OLD:
   mut texture_atlases: ResMut<Assets<TextureAtlas>>,

   // NEW:
   mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
   ```

2. **Create layout instead of atlas:**
   ```rust
   let layout = TextureAtlasLayout::from_grid(
       UVec2::new(48, 48),  // Note: UVec2, not Vec2
       8,  // columns
       4,  // rows
       None,
       None,
   );
   let layout_handle = texture_atlas_layouts.add(layout);
   ```

3. **Use new spawn method:**
   ```rust
   let player_entity = PlayerBundle::spawn(
       &mut commands,
       texture_handle,
       layout_handle,
       Vec3::ZERO,
   );

   // Then add children:
   commands.entity(player_entity).with_children(|parent| {
       // ... gun, UI, etc.
   });
   ```

4. **Test compilation:**
   ```bash
   cargo build 2>&1 | grep "spawn_player"
   ```

#### Task 1.4: Update Other Spawn Systems (20 min)

Apply same pattern to:
- `src/systems/spawn_mobs.rs`
- `src/systems/spawn_mechs.rs`
- `src/systems/spawn_powerups.rs` (if it spawns sprites)

#### Task 1.5: Fix Sprite Animation System (10 min)

**File:** `src/systems/update_sprite_index.rs`

**Change query type:**
```rust
// OLD:
mut sprites: Query<(&mut TextureAtlasSprite, &SpriteAnimation)>

// NEW:
mut sprites: Query<(&mut TextureAtlas, &SpriteAnimation)>
```

**Update index access:**
```rust
// OLD:
sprite.index = ...

// NEW:
atlas.index = ...
```

#### Task 1.6: Verify Sprite System (5 min)

```bash
# Should have significantly fewer errors
cargo build 2>&1 | grep "^error\[E0" | wc -l
```

**Commit your changes:**
```bash
git add -A
git commit -m "Migrate sprite system to Bevy 0.14 TextureAtlas API

- Replace SpriteSheetBundle with SpriteBundle + TextureAtlas component
- Create TextureAtlasLayout for grid definitions
- Update all bundles (Player, Mob, Mech, Bullet, Chest)
- Update spawn systems to use new API
- Fix sprite animation system to use TextureAtlas component

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Phase 2: System Registration API (45 minutes)

This requires rewriting `main.rs` system setup. It's tedious but straightforward.

#### Task 2.1: Backup and Understand Current Structure (5 min)

1. **Create backup:**
   ```bash
   cp src/main.rs src/main.rs.backup
   ```

2. **Identify system groups:**
   - UI systems (run every frame)
   - Powerup systems (fixed timestep, sequential)
   - Game loop systems (fixed timestep, ordered)
   - Cleanup systems (fixed timestep, after game loop)
   - Startup systems (on state enter)

#### Task 2.2: Replace UI Systems (5 min)

**Find this code (around line 36):**
```rust
let ui_system_set = SystemSet::on_update(AppState::InGame)
    .with_system(systems::update_compass)
    // ... more systems
```

**Replace with:**
```rust
// In the App::new() builder chain:
.add_systems(Update, (
    systems::update_compass,
    systems::update_score_ui,
    systems::update_health_ui,
    systems::sync_mouse_position,
    systems::follow_player_with_camera,
    systems::on_no_players_show_game_over,
    systems::on_click_and_no_player_reset,
    systems::spin_spin_me,
    systems::update_sprite_animation_for_always_animate,
    systems::update_sprite_index,
).run_if(in_state(AppState::InGame)))
```

**Remove the old:**
```rust
// DELETE:
.add_system_set(ui_system_set)
```

#### Task 2.3: Replace Powerup Systems (10 min)

**Find the powerup system set (around line 48):**
```rust
let compute_powerups_system_set = SystemSet::on_update(AppState::InGame)
    .label("compute_powerups_system_set")
    .with_system(systems::powerup_defaulter::<Speed>)
    .with_system(systems::powerup_mather::<Speed>.after(...))
    // ... 5 more stat types
    .with_run_criteria(FixedTimestep::step(TIME_STEP));
```

**Replace with:**
```rust
.add_systems(FixedUpdate, (
    // Speed
    (
        systems::powerup_defaulter::<Speed>,
        systems::powerup_mather::<Speed>,
    ).chain(),
    // Health
    (
        systems::powerup_defaulter::<Health>,
        systems::powerup_mather::<Health>,
    ).chain(),
    // RateOfFire
    (
        systems::powerup_defaulter::<RateOfFire>,
        systems::powerup_mather::<RateOfFire>,
    ).chain(),
    // TimeToLive
    (
        systems::powerup_defaulter::<TimeToLive>,
        systems::powerup_mather::<TimeToLive>,
    ).chain(),
    // Poison
    (
        systems::powerup_defaulter::<Poison>,
        systems::powerup_mather::<Poison>,
    ).chain(),
    // AmmoCount
    (
        systems::powerup_defaulter::<AmmoCount>,
        systems::powerup_mather::<AmmoCount>,
    ).chain(),
).run_if(in_state(AppState::InGame)))
```

**Remove old:**
```rust
// DELETE:
.add_system_set(compute_powerups_system_set)
```

**Remove FixedTimestep import:**
```rust
// DELETE from imports:
use bevy::{prelude::*, time::FixedTimestep};

// KEEP:
use bevy::prelude::*;
```

#### Task 2.4: Replace Game Loop Systems (15 min)

This is the most complex because of ordering. Be careful.

**Find game loop system set (around line 70):**
```rust
let game_loop_system_set = SystemSet::on_update(AppState::InGame)
    .label("game_loop_system_set")
    .after("compute_powerups_system_set")
    .with_system(systems::count_ticks)
    .with_system(systems::move_player)
    // ... 30+ systems with complex ordering
    .with_run_criteria(FixedTimestep::step(TIME_STEP));
```

**Replace with (simplified - adjust ordering as needed):**
```rust
.add_systems(FixedUpdate, (
    systems::count_ticks,

    // Movement phase
    (
        systems::move_player,
        systems::move_bullet,
        systems::chasers_follow_other_teams,
    ).before(systems::move_thing),
    systems::move_thing,

    // Aiming/Rotation phase
    (
        systems::player_aimables_aim_at_cursor,
        systems::chaser_aimables_aim_at_other_teams,
    ).before(systems::rotate_thing),
    systems::rotate_thing,

    // Animation
    systems::on_move_event_update_sprite_animation,
    systems::on_move_event_update_3d_rotation,
    systems::on_move_event_advance_3d_walking_animation,

    // Shooting phase
    (
        systems::on_left_click_shoot,
        systems::on_scroll_wheel_switch_ammo,
        systems::on_f_key_switch_ammo,
        systems::calculate_rate_of_fire,
        systems::mob_shoot,
    ).before(systems::shoot_gun),
    systems::shoot_gun,

    // Combat & Effects
    systems::on_chest_hit_pickup,
    systems::poison,
    systems::team_powerup_assigns_team,
    systems::on_0_health_kill,

    // Spawning
    systems::spawn_mobs,
    systems::spawn_mechs,
    systems::spawn_powerups,

    // Time-based effects
    systems::attach_time_to_live,
    systems::time_to_live,
    systems::attach_poison,
).run_if(in_state(AppState::InGame)))
```

**NOTE:** The exact ordering depends on your game logic. Review the old code carefully and preserve critical ordering with `.before()`, `.after()`, or `.chain()`.

#### Task 2.5: Replace Cleanup Systems (5 min)

**Find cleanup system set:**
```rust
let game_loop_cleanup_system_set = SystemSet::on_update(AppState::InGame)
    .label("game_loop_cleanup_system_set")
    .after("game_loop_system_set")
    .with_system(systems::consume_despawn_entity_events)
    .with_run_criteria(FixedTimestep::step(TIME_STEP));
```

**Replace with:**
```rust
.add_systems(FixedUpdate,
    systems::consume_despawn_entity_events
        .run_if(in_state(AppState::InGame))
)
```

**Note:** This should run after game loop systems. Since we can't reference system groups by name, either:
1. Put it at the end of the FixedUpdate tuple (runs after)
2. Use `.after()` with a specific system from game loop

#### Task 2.6: Replace Startup Systems (5 min)

**Find startup system set:**
```rust
let startup_system_set = SystemSet::on_enter(AppState::InGame)
    .with_system(systems::load_sprites)
    // ... more systems
```

**Replace with:**
```rust
.add_systems(OnEnter(AppState::InGame), (
    systems::load_sprites,
    systems::spawn_camera,
    systems::spawn_player,
    systems::load_mech_walking_animation,
    systems::spawn_ui,
    systems::spawn_crosshairs,
    systems::spawn_lights,
))
```

**Remove old:**
```rust
// DELETE:
.add_system_set(startup_system_set)
```

#### Task 2.7: Update State Initialization (2 min)

**Find:**
```rust
.add_state(AppState::InGame)
```

**Replace with:**
```rust
.init_state::<AppState>()
```

**Explanation:** We now use `.init_state::<T>()` which uses the `#[default]` variant we marked earlier.

#### Task 2.8: Verify System Registration (3 min)

```bash
cargo build 2>&1 | grep -i "system"
```

**Look for:**
- `unresolved import` errors (old API usage)
- `cannot find` errors (old types)

**Commit:**
```bash
git add src/main.rs
git commit -m "Migrate system registration to Bevy 0.14 API

- Replace SystemSet with add_systems()
- Use Update and FixedUpdate schedules
- Convert FixedTimestep to FixedUpdate schedule
- Add run_if(in_state()) conditions
- Use .chain() for sequential execution
- Use .before()/.after() for ordering
- Update state initialization to init_state()

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Phase 3: Quick Fixes (30 minutes)

#### Task 3.1: Fix Query API (10 min)

**Find all `.for_each()` calls:**
```bash
grep -r "\.for_each" src/systems/
```

**Replace pattern:**
```rust
// OLD:
query.for_each(|item| { ... });

// NEW:
for item in &query { ... }

// OLD:
query.for_each_mut(|mut item| { ... });

// NEW:
for mut item in &mut query { ... }
```

**Files:**
- `src/systems/sync_mouse_position.rs:30`
- `src/systems/update_score_ui.rs:6`
- `src/systems/team_powerup_assigns_team.rs:6`
- `src/systems/time_to_live.rs:12`

**Fix `Query<Without<T>>`:**

**Files:**
- `src/systems/attach_poison.rs`
- `src/systems/attach_time_to_live.rs`

**Change:**
```rust
// OLD:
q_no_poison: Query<Without<Poison>>

// NEW:
q_no_poison: Query<Entity, Without<Poison>>
```

**Commit:**
```bash
git add -A
git commit -m "Fix Query API changes for Bevy 0.14

- Replace .for_each() with for loop iteration
- Fix Query<Without<T>> to Query<Entity, Without<T>>

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

#### Task 3.2: Fix Text API (10 min)

**Files:**
- `src/bundles/health.rs:25`
- `src/systems/on_no_players_show_game_over.rs:41`

**Change:**
```rust
// OLD:
alignment: TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
}

// NEW:
text_justification: JustifyText::Center,
```

**Note:** You may also need to update the text spawn structure. Check Bevy 0.14 text example.

**Commit:**
```bash
git add -A
git commit -m "Fix Text API changes for Bevy 0.14

- Replace TextAlignment with JustifyText

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

#### Task 3.3: Fix Minor Issues (10 min)

**Camera2d ambiguity:**

**File:** `src/systems/follow_player_with_camera.rs`

**Change:**
```rust
// OLD:
use crate::resources::{CameraType::*, Config};

// NEW:
use crate::resources::{CameraType, Config};

// And in match:
match config.camera_type {
    CameraType::Camera2d => ...,
    CameraType::Camera3d => ...,
}
```

**Remove unused imports:**
```bash
cargo clippy --fix
```

**Commit:**
```bash
git add -A
git commit -m "Fix minor API issues for Bevy 0.14

- Fix Camera2d import ambiguity
- Remove unused imports

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Phase 4: Testing & Validation (30 minutes)

#### Task 4.1: Compilation Test

```bash
cargo build --release
```

**Expected:** 0 errors, possible warnings

**If errors remain:**
1. Read error message carefully
2. Check `docs/BREAKING_CHANGES_REMAINING.md`
3. Search Bevy 0.14 docs
4. Ask for help on Bevy Discord

#### Task 4.2: Launch Test

```bash
cargo run
```

**Check:**
- [ ] Window opens
- [ ] No panic on startup
- [ ] Player sprite renders
- [ ] Background/UI renders

**If crash/panic:**
1. Read panic message
2. Check asset paths
3. Verify texture atlas layouts match sprite sheets

#### Task 4.3: Gameplay Test

**Test checklist:**
- [ ] WASD movement works
- [ ] Mouse aiming works
- [ ] Left-click shooting works
- [ ] Bullets spawn and move
- [ ] Enemies spawn
- [ ] Enemies move toward player
- [ ] Collision detection works
- [ ] Health decreases on hit
- [ ] Score increases on kills
- [ ] UI updates (health bar, score)
- [ ] Game over screen appears on death
- [ ] Click to restart works

**If issues:**
1. Check console for errors
2. Verify system ordering (movement → collision → damage)
3. Check event sending/receiving

#### Task 4.4: Performance Test

```bash
cargo run --release
```

**Monitor:**
- FPS (should be 60)
- Frame time (should be ~16.6ms)
- No stuttering
- No memory leaks over 5 minutes

**If performance issues:**
1. Profile with `cargo flamegraph`
2. Check for unnecessary entity spawns
3. Verify fixed timestep is working

---

### Phase 5: Cleanup & Documentation (15 minutes)

#### Task 5.1: Final Cleanup

```bash
# Run clippy
cargo clippy -- -W clippy::all

# Fix any warnings
cargo clippy --fix

# Format code
cargo fmt
```

#### Task 5.2: Update Documentation

**File:** `docs/MIGRATION_STATUS.md`

Update:
- Status to "COMPLETE"
- Error count to 0
- Testing results
- Any issues encountered

#### Task 5.3: Final Commit

```bash
git add -A
git commit -m "Complete Bevy 0.14 migration

All breaking changes have been addressed:
✅ Sprite system migrated
✅ System registration updated
✅ Query API fixed
✅ Text API updated
✅ All tests passing

Game is now running on Bevy 0.14.2 with no regressions.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

#### Task 5.4: Push to Remote

```bash
git push origin refactor/ecs-simplification
```

---

## Troubleshooting

### Common Issues

#### Issue: "Cannot find TextureAtlasLayout"

**Solution:** Check imports:
```rust
use bevy::prelude::*;
use bevy::sprite::TextureAtlasLayout;  // May need explicit import
```

#### Issue: "Method `entity` not found for `WindowRef`"

**Solution:** Check WindowRef API in Bevy 0.14 docs. May be:
```rust
window_ref.id  // or
q_window.iter().next()  // simpler fallback
```

#### Issue: "Sprites not rendering"

**Checklist:**
1. Verify texture assets exist in `assets/` folder
2. Check console for asset loading errors
3. Verify TextureAtlasLayout grid matches sprite sheet
4. Check Z-index (transform.translation.z)
5. Verify camera is spawned and active

#### Issue: "Systems not running"

**Checklist:**
1. Verify `run_if(in_state(AppState::InGame))` condition
2. Check that AppState is correctly initialized
3. Verify system schedule (Update vs FixedUpdate)
4. Check for system panics in console

---

## Getting Help

### Resources

1. **Bevy Official Docs:**
   - [Migration Guide 0.13 → 0.14](https://bevyengine.org/learn/migration-guides/0.13-0.14/)
   - [Bevy 0.14 Examples](https://github.com/bevyengine/bevy/tree/v0.14.0/examples)

2. **Community:**
   - [Bevy Discord](https://discord.gg/bevy) - #help channel
   - [Bevy GitHub Discussions](https://github.com/bevyengine/bevy/discussions)

3. **Project Docs:**
   - `docs/MIGRATION_STATUS.md` - Current status
   - `docs/BREAKING_CHANGES_COMPLETED.md` - What's done
   - `docs/BREAKING_CHANGES_REMAINING.md` - Detailed API changes

### Questions to Ask

When stuck, provide:
1. Exact error message
2. File and line number
3. Relevant code snippet
4. What you've tried
5. Bevy version (0.14.2)

---

## Success Criteria

Migration is complete when:

- [ ] `cargo build` completes with 0 errors
- [ ] `cargo run` launches without panic
- [ ] All sprites render correctly
- [ ] Player movement works
- [ ] Shooting and combat work
- [ ] UI updates correctly
- [ ] Game runs at 60 FPS
- [ ] No memory leaks
- [ ] All gameplay features work as before

---

## Next Steps After Migration

Once migration is complete, proceed with:

1. **System Consolidation** (Phase 2 of refactoring)
   - Merge 67 system files into 15 modules
   - See `PLAN.md` for details

2. **Powerup System Simplification** (Phase 3)
   - Remove entity spawning for transient operations
   - See `PLAN.md` for details

3. **Testing Infrastructure** (Phase 4)
   - Add unit tests
   - Add integration tests
   - See `PLAN.md` for details

4. **Debug Tools** (Phase 5)
   - System isolation
   - Entity inspector
   - See `PLAN.md` for details

---

## Timeline Estimate

| Phase | Tasks | Time | Cumulative |
|-------|-------|------|------------|
| Sprite System | 6 tasks | 90 min | 90 min |
| System Registration | 8 tasks | 45 min | 135 min (2.25 hrs) |
| Quick Fixes | 3 tasks | 30 min | 165 min (2.75 hrs) |
| Testing | 4 tasks | 30 min | 195 min (3.25 hrs) |
| Cleanup | 4 tasks | 15 min | 210 min (3.5 hrs) |

**Total:** ~3.5 hours for careful, methodical work

---

**Good luck! You've got this! 🚀**
