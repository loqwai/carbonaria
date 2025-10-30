# Bevy 0.14 Migration Status

**Project:** Carbonaria Roguelike
**Branch:** `refactor/ecs-simplification`
**Started:** 2025-10-29
**Last Updated:** 2025-10-29
**Status:** 🟡 **IN PROGRESS** (40% complete)

---

## Quick Summary

We are migrating from **Bevy 0.9.1 → Bevy 0.14.2** as part of a broader ECS refactoring effort. The migration involves fixing **134 compilation errors** caused by breaking API changes across input, window management, sprite rendering, system scheduling, events, and states.

**Current Progress:**
- ✅ Dependencies updated (Bevy, bevy_rapier2d, bevy-inspector-egui, clap)
- ✅ Rust toolchain updated (1.71 → 1.90)
- ✅ Input API migrated (`Input<T>` → `ButtonInput<T>`)
- ✅ Window API migrated (`Windows` resource → `Query<&Window>`)
- ✅ Event system migrated (added `#[derive(Event)]`)
- ✅ State system migrated (added `#[derive(States)]`)
- ✅ Ray API updated (`Ray` → `Ray3d`)
- 🟡 Sprite system migration (IN PROGRESS - 0% complete)
- 🔴 System registration API (NOT STARTED)
- 🔴 Text API updates (NOT STARTED)
- 🔴 Asset loading updates (NOT STARTED)

**Commits Made:** 3
**Errors Remaining:** ~80 (down from 134)
**Estimated Time to Complete:** 2-3 hours

---

## Dependency Updates

### Updated Crates

| Crate | Old Version | New Version | Major Changes |
|-------|-------------|-------------|---------------|
| `bevy` | 0.9.1 | 0.14.2 | ECS, rendering, input, windowing |
| `bevy_rapier2d` | 0.20.0 | 0.27.0 | Physics API updates |
| `bevy-inspector-egui` | 0.17.0 | 0.25.2 | Inspector UI changes |
| `clap` | 4.1.8 | 4.5.51 | CLI parsing improvements |
| `rand` | 0.8.5 | 0.8.5 | No changes |

### Rust Toolchain

- **Previous:** `rustc 1.71.0` (2023-07-12)
- **Updated:** `rustc 1.90.0` (2025-09-14)
- **Reason:** Bevy 0.14 dependencies require Rust 2024 edition support

---

## Completed Migrations

### 1. ✅ Input System (`Input<T>` → `ButtonInput<T>`)

**Affected Files:** 3
- `src/systems/move_player.rs`
- `src/systems/on_click_and_no_player_reset.rs`
- `src/systems/on_f_key_switch_ammo.rs`

**Key Changes:**
- Resource type: `Res<Input<KeyCode>>` → `Res<ButtonInput<KeyCode>>`
- Resource type: `Res<Input<MouseButton>>` → `Res<ButtonInput<MouseButton>>`
- KeyCode enum: `KeyCode::A` → `KeyCode::KeyA` (all letter keys)
- Event reading: `event.key_code` now returns `KeyCode` directly, not `Option<KeyCode>`
- Event iteration: `keyboard_input.iter()` → `keyboard_input.read()`
- State checking: `event.state` → `event.state.is_pressed()`

**Commit:** `4156591`

---

### 2. ✅ Window System (`Windows` → `Query<&Window>`)

**Affected Files:** 3
- `src/systems/sync_mouse_position.rs`
- `src/systems/spawn_crosshairs.rs`
- `src/systems/resize_window.rs`

**Key Changes:**
- Resource removed: `Res<Windows>` is now `Query<&Window>`
- Window access: `windows.get_primary()` → `q_window.get_single()` or `q_window.iter().next()`
- Cursor visibility: `window.set_cursor_visibility(false)` → `window.cursor.visible = false`
- Window mode: `window.set_maximized(true)` → `window.mode = WindowMode::Windowed`
- Render targets: `RenderTarget::Window(id)` now uses `WindowRef` instead of `WindowId`

**Commit:** `4156591`

---

### 3. ✅ Event System (Derive Macro Required)

**Affected Files:** 1
- `src/events/mod.rs`

**Key Changes:**
All custom event types now require `#[derive(Event)]`:
```rust
// Before
pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}

// After
#[derive(Event)]
pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}
```

**Events Updated:**
- `MoveEvent`
- `RotateEvent`
- `DespawnEvent`
- `ShootEvent`
- `DamagerHitEvent`

**Commit:** `1e952ee`

---

### 4. ✅ State System (Derive Macro Required)

**Affected Files:** 1
- `src/main.rs`

**Key Changes:**
State enums now require `#[derive(States, Default)]`:
```rust
// Before
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    InGame,
}

// After
#[derive(Clone, Debug, Hash, Eq, PartialEq, States, Default)]
pub enum AppState {
    #[default]
    InGame,
}
```

**State Transition API:**
- Old: `app_state.restart().unwrap()`
- New: `app_state.set(AppState::InGame)` where `app_state: ResMut<NextState<AppState>>`

**Commit:** `1e952ee`

---

### 5. ✅ Ray API (`Ray` → `Ray3d`)

**Affected Files:** 1
- `src/systems/sync_mouse_position.rs`

**Key Changes:**
- Type renamed: `Ray` → `Ray3d`
- Direction field: `ray.direction` is now a reference, requires dereferencing with `*ray.direction`
- Camera method: `camera.viewport_to_world()` now returns `Ray3d` directly

**Commit:** `4156591`

---

## In-Progress Migrations

### 🟡 Sprite System Refactor

**Status:** NOT STARTED
**Complexity:** ⚠️ **HIGH** (most complex migration)
**Estimated Time:** 1-1.5 hours
**Affected Files:** ~15 files

**Why This Is Critical:**
The sprite system is fundamental to the game's rendering. All visual entities (player, enemies, bullets, powerups) use the old `SpriteSheetBundle` + `TextureAtlasSprite` pattern which has been completely redesigned in Bevy 0.14.

**Files Requiring Changes:**
1. `src/bundles/player.rs` - Player sprite initialization
2. `src/bundles/mob.rs` - Enemy sprite initialization
3. `src/bundles/mech.rs` - Mech sprite initialization
4. `src/bundles/bullet.rs` - Bullet sprite initialization
5. `src/bundles/chest.rs` - Chest/powerup sprite initialization
6. `src/systems/update_sprite_index.rs` - Sprite animation system
7. `src/systems/spawn_*.rs` - Various spawn systems (may need texture atlas handle changes)
8. `src/main.rs` - Asset loading for texture atlases

**See:** `docs/BREAKING_CHANGES_REMAINING.md` for detailed migration guide

---

## Remaining Migrations

### 🔴 System Registration API

**Status:** NOT STARTED
**Complexity:** ⚠️ **HIGH** (requires rewriting main.rs system setup)
**Estimated Time:** 30-45 minutes
**Affected Files:** 1-2 files

Bevy 0.10+ completely redesigned how systems are registered and scheduled. The old `SystemSet` builder pattern is replaced with a more functional approach using tuples and system combinators.

**See:** `docs/BREAKING_CHANGES_REMAINING.md` for detailed migration guide

---

### 🔴 Text Rendering API

**Status:** NOT STARTED
**Complexity:** 🟢 **LOW**
**Estimated Time:** 15-20 minutes
**Affected Files:** 3 files

`TextAlignment` structure has changed. Needs updates in UI systems.

**See:** `docs/BREAKING_CHANGES_REMAINING.md` for detailed migration guide

---

### 🔴 Asset Loading API

**Status:** NOT STARTED
**Complexity:** 🟡 **MEDIUM**
**Estimated Time:** 20-30 minutes
**Affected Files:** 2-3 files

- `Handle<TextureAtlas>` → `Handle<TextureAtlasLayout>`
- `load_folder()` API changed
- Asset handles may need different storage

**See:** `docs/BREAKING_CHANGES_REMAINING.md` for detailed migration guide

---

### 🔴 Minor API Updates

**Status:** NOT STARTED
**Complexity:** 🟢 **TRIVIAL**
**Estimated Time:** 10-15 minutes
**Affected Files:** 2-3 files

- Camera2d import ambiguity
- Query filter syntax changes
- Minor type updates

**See:** `docs/BREAKING_CHANGES_REMAINING.md` for details

---

## Error Analysis

### Error Counts by Category

| Category | Errors | Status |
|----------|--------|--------|
| Sprite System | ~35 | 🔴 Not Started |
| System Registration | ~25 | 🔴 Not Started |
| Text Rendering | ~8 | 🔴 Not Started |
| Asset Loading | ~6 | 🔴 Not Started |
| Camera/Query Issues | ~6 | 🔴 Not Started |
| **TOTAL** | **~80** | **40% Complete** |

### Error Breakdown

**Current Build Output:**
```bash
$ cargo build 2>&1 | grep "^error\[" | wc -l
80
```

**Error Types:**
1. `E0422: cannot find struct TextureAtlasSprite` (×7) - Bundle files
2. `E0422: cannot find struct TextAlignment` (×2) - Text UI files
3. `E0412: cannot find type TextureAtlasSprite` (×1) - Update system
4. `E0659: Camera2d is ambiguous` (×1) - Import conflict
5. `E0277: trait bound not satisfied` (×many) - System registration, assets, queries

**See:** `docs/ERROR_ANALYSIS.md` for complete error listing and solutions

---

## Testing Strategy

### Pre-Migration Testing (Completed)

Before starting the migration, we verified:
- ✅ Git repository is clean (all changes committed)
- ✅ Project builds on Bevy 0.9.1
- ✅ Created feature branch `refactor/ecs-simplification`

### Post-Migration Testing (TODO)

Once all errors are fixed:

1. **Compilation Test**
   ```bash
   cargo build --release
   ```
   - Should complete without errors
   - Should complete without warnings (goal)

2. **Game Launch Test**
   ```bash
   cargo run
   ```
   - Game window should open
   - No runtime panics
   - Splash screen / initial state loads

3. **Core Gameplay Test**
   - Player spawns correctly
   - WASD movement works
   - Mouse aiming works
   - Left-click shooting works
   - Enemies spawn and move
   - Collision detection works
   - Health/damage system works
   - UI updates (score, health bar)

4. **Advanced Features Test**
   - Powerups spawn and apply effects
   - Weapon switching (F/V keys)
   - Different ammo types work
   - Mech spawning works
   - 3D models render correctly (if using 3D mode)
   - Camera following works

5. **Performance Test**
   - Game runs at 60 FPS
   - No memory leaks over 5 minutes
   - Entity count remains stable

**See:** `docs/TESTING_STRATEGY.md` for complete testing checklist

---

## Git History

### Commits on `refactor/ecs-simplification`

```
94058c9 - Add comprehensive refactoring plan
1e952ee - Add Event derive to custom events and States to AppState
4156591 - Fix Input and Windows API breaking changes for Bevy 0.14
```

### Files Modified (Total: 10)

**Configuration:**
- `Cargo.toml` - Dependency updates
- `Cargo.lock` - Dependency resolution

**Documentation:**
- `PLAN.md` - Overall refactoring plan (850 lines)

**Source Code:**
- `src/main.rs` - AppState derive macros
- `src/events/mod.rs` - Event derive macros
- `src/systems/move_player.rs` - Input API
- `src/systems/on_click_and_no_player_reset.rs` - Input + State API
- `src/systems/on_f_key_switch_ammo.rs` - Input API
- `src/systems/sync_mouse_position.rs` - Window + Ray API
- `src/systems/spawn_crosshairs.rs` - Window API
- `src/systems/resize_window.rs` - Window API

### Lines Changed

```
 11 files changed, 3979 insertions(+), 1537 deletions(-)
```

---

## Next Steps for Continuation

### Immediate Next Task: Fix Sprite System

**Priority:** 🔴 **CRITICAL** (blocks all visual testing)

The sprite system migration is the most complex remaining task but also the most important. Without it, the game cannot render any entities.

**Recommended Approach:**
1. Start with one bundle (e.g., `player.rs`)
2. Fix the bundle definition
3. Test compilation
4. Apply same pattern to other bundles
5. Fix the sprite animation system
6. Test game launch

**Time Estimate:** 1-1.5 hours for careful, methodical work

**See:** `docs/CONTINUATION_GUIDE.md` for step-by-step instructions

---

### After Sprite System: Fix System Registration

**Priority:** 🔴 **CRITICAL** (required to run game)

The system registration API change affects `main.rs` and requires rewriting how all systems are scheduled.

**Time Estimate:** 30-45 minutes

---

### After System Registration: Remaining Minor Fixes

**Priority:** 🟡 **MEDIUM** (polish and features)

Text rendering, asset loading, and misc fixes.

**Time Estimate:** 45-60 minutes combined

---

### Final Steps: Testing & Documentation

**Priority:** 🟢 **LOW** (but important)

Run full testing suite, document any behavioral changes, update main PLAN.md.

**Time Estimate:** 30 minutes

---

## Architecture Context

### Why This Migration Matters

This migration is **Phase 1** of a larger refactoring effort to:

1. **Update Dependencies** (THIS PHASE)
   - Bevy 0.9 → 0.14 (5 major versions)
   - Access modern Bevy features
   - Improved performance and ergonomics

2. **Consolidate Systems** (Next Phase)
   - 67 system files → 15 modules
   - Better organization
   - Easier to navigate

3. **Simplify Powerup System** (Later Phase)
   - Remove over-engineered `Math<T>` entity spawning
   - Direct mutations for transient operations
   - Keep hierarchy for persistent modifiers

4. **Add Testing** (Later Phase)
   - Unit tests for systems
   - Integration tests for gameplay
   - Determinism tests for replay

5. **Add Debug Tools** (Later Phase)
   - System isolation
   - Entity inspector
   - Event logging

**Critical Decision:** We must complete the Bevy migration before any other refactoring. System consolidation and powerup simplification require a working, compiling codebase on modern Bevy.

---

## Risks & Mitigation

### Risk 1: Behavioral Changes

**Risk:** New Bevy version may have subtle behavioral differences (timing, physics, rendering)

**Mitigation:**
- Extensive gameplay testing post-migration
- Compare frame-by-frame if possible
- Document any intentional behavior changes

### Risk 2: Performance Regression

**Risk:** Bevy 0.14 may have different performance characteristics

**Mitigation:**
- Profile before/after migration
- Test at 60 FPS target
- Monitor entity counts and memory usage

### Risk 3: Incomplete Migration

**Risk:** Some errors may be subtle and only appear at runtime

**Mitigation:**
- Comprehensive testing checklist
- Play-test all features
- Test edge cases (no players, game over, etc.)

### Risk 4: Asset Compatibility

**Risk:** Sprite atlases or 3D models may need regeneration

**Mitigation:**
- Test all visual assets load correctly
- Check 3D model animations
- Verify texture atlas indices haven't shifted

---

## Resources

### Official Bevy Migration Guides

- [Bevy 0.9 → 0.10 Migration Guide](https://bevyengine.org/learn/migration-guides/0.9-0.10/)
- [Bevy 0.10 → 0.11 Migration Guide](https://bevyengine.org/learn/migration-guides/0.10-0.11/)
- [Bevy 0.11 → 0.12 Migration Guide](https://bevyengine.org/learn/migration-guides/0.11-0.12/)
- [Bevy 0.12 → 0.13 Migration Guide](https://bevyengine.org/learn/migration-guides/0.12-0.13/)
- [Bevy 0.13 → 0.14 Migration Guide](https://bevyengine.org/learn/migration-guides/0.13-0.14/)

### Key API Documentation

- [Bevy 0.14 Input Documentation](https://docs.rs/bevy/0.14.2/bevy/input/index.html)
- [Bevy 0.14 Window Documentation](https://docs.rs/bevy/0.14.2/bevy/window/index.html)
- [Bevy 0.14 Sprite Documentation](https://docs.rs/bevy/0.14.2/bevy/sprite/index.html)
- [Bevy 0.14 ECS Documentation](https://docs.rs/bevy/0.14.2/bevy/ecs/index.html)

### Related Documentation Files

- `PLAN.md` - Overall refactoring plan (this migration is Phase 1)
- `docs/BREAKING_CHANGES_COMPLETED.md` - Detailed guide of completed migrations
- `docs/BREAKING_CHANGES_REMAINING.md` - Detailed guide of remaining work
- `docs/CONTINUATION_GUIDE.md` - Step-by-step guide to continue
- `docs/ERROR_ANALYSIS.md` - Full error log and solutions
- `docs/TESTING_STRATEGY.md` - Comprehensive testing checklist

---

## Questions & Troubleshooting

### Q: Can I test the game before finishing all migrations?

**A:** No. The game will not compile until all sprite system errors are fixed. The sprite system is fundamental to rendering.

### Q: Can I work on other refactoring tasks in parallel?

**A:** Not recommended. Wait until the migration is complete and the game compiles. Making architectural changes on a broken codebase is risky.

### Q: What if I encounter an error not documented here?

**A:**
1. Check the official Bevy migration guides (links above)
2. Search the error message on GitHub Issues
3. Check the Bevy Discord #help channel
4. Document your solution for future reference

### Q: How do I know if the migration is successful?

**A:**
1. `cargo build` completes with 0 errors
2. `cargo run` launches the game without panics
3. All core gameplay features work (movement, shooting, enemies)
4. UI updates correctly
5. No performance regressions

### Q: Should I upgrade to Bevy 0.15 while I'm at it?

**A:** **NO.** Stick with 0.14 for now. One major migration at a time. Once the refactoring is complete and stable, consider upgrading to 0.15 as a separate task.

---

## Timeline

| Date | Task | Status |
|------|------|--------|
| 2025-10-29 | Create PLAN.md | ✅ |
| 2025-10-29 | Update dependencies | ✅ |
| 2025-10-29 | Fix Input API | ✅ |
| 2025-10-29 | Fix Window API | ✅ |
| 2025-10-29 | Fix Event/State API | ✅ |
| 2025-10-29 | Write documentation | 🟡 IN PROGRESS |
| TBD | Fix Sprite System | 🔴 NOT STARTED |
| TBD | Fix System Registration | 🔴 NOT STARTED |
| TBD | Fix Text/Asset APIs | 🔴 NOT STARTED |
| TBD | Testing & Validation | 🔴 NOT STARTED |
| TBD | Complete Refactoring | 🔴 NOT STARTED |

**Estimated Completion:** 2-3 additional hours of focused work

---

## Status Legend

- ✅ **COMPLETE** - Fully implemented and committed
- 🟡 **IN PROGRESS** - Currently being worked on
- 🔴 **NOT STARTED** - Queued for future work
- ⚠️ **BLOCKED** - Cannot proceed until dependency is resolved
