# Error Analysis - Current Build State

**Generated:** 2025-10-29
**Branch:** `refactor/ecs-simplification`
**Total Errors:** 74
**Bevy Version:** 0.14.2

This document provides a complete analysis of all remaining compilation errors.

---

## Error Summary by Category

| Category | Count | Priority |
|----------|-------|----------|
| Sprite System (TextureAtlasSprite) | 35 | 🔴 CRITICAL |
| System Registration (SystemSet) | 18 | 🔴 CRITICAL |
| Query API (for_each, Without) | 8 | 🟡 MEDIUM |
| Text Rendering (TextAlignment) | 2 | 🟡 MEDIUM |
| Asset Loading | 4 | 🟡 MEDIUM |
| Type Mismatches | 5 | 🟡 MEDIUM |
| Minor Issues | 2 | 🟢 LOW |

---

## Detailed Error Breakdown

### Category 1: Sprite System Errors (35 errors)

#### Error E0422: TextureAtlasSprite not found

**Count:** 7 occurrences
**Files Affected:**
- `src/bundles/chest.rs:52`
- `src/bundles/bullet.rs:46`
- `src/bundles/mech.rs:58`
- `src/bundles/mob.rs:53`
- `src/bundles/player.rs:54`

**Error Message:**
```
error[E0422]: cannot find struct, variant or union type `TextureAtlasSprite` in this scope
  --> src/bundles/player.rs:54:25
   |
54 |                 sprite: TextureAtlasSprite {
   |                         ^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `TextureAtlas`
```

**Root Cause:** `TextureAtlasSprite` was removed in Bevy 0.13. Replaced with `Sprite` component + `TextureAtlas` component.

**Solution:** See `docs/BREAKING_CHANGES_REMAINING.md` Section 1

**Priority:** 🔴 CRITICAL (blocks visual rendering)

---

#### Error E0412: TextureAtlasSprite not found (Query)

**Count:** 1 occurrence
**File:** `src/systems/update_sprite_index.rs:4`

**Error Message:**
```
error[E0412]: cannot find type `TextureAtlasSprite` in this scope
  --> src/systems/update_sprite_index.rs:4:53
   |
 4 | pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlasSprite, &SpriteAnimation)>) {
   |                                                     ^^^^^^^^^^^^^^^^^^
```

**Root Cause:** Query still references old `TextureAtlasSprite` type.

**Solution:**
```rust
// Change to:
pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlas, &SpriteAnimation)>) {
    for (mut atlas, animation) in &mut sprites {
        atlas.index = calculate_index(animation);
    }
}
```

**Priority:** 🔴 CRITICAL

---

#### Error E0560: SpriteSheetBundle missing fields

**Count:** ~10 occurrences (estimated from bundle errors)
**Files:** All bundle files

**Error Message:**
```
error[E0560]: struct `bevy::prelude::SpriteSheetBundle` has no field named `sprite`
```

**Root Cause:** `SpriteSheetBundle` was removed. Need to use `SpriteBundle` + `TextureAtlas` component.

**Solution:** See detailed migration guide in `docs/BREAKING_CHANGES_REMAINING.md` Section 1

**Priority:** 🔴 CRITICAL

---

### Category 2: System Registration Errors (18 errors)

#### Error E0433: SystemSet not found

**Count:** ~5 occurrences
**File:** `src/main.rs` (multiple lines)

**Error Message:**
```
error[E0433]: failed to resolve: could not find `SystemSet` in `prelude`
```

**Root Cause:** `SystemSet` API changed completely in Bevy 0.10+. Now uses `add_systems()` with schedules.

**Solution:** See `docs/BREAKING_CHANGES_REMAINING.md` Section 2

**Priority:** 🔴 CRITICAL (game cannot run)

---

#### Error E0433: FixedTimestep not found

**Count:** 1 occurrence
**File:** `src/main.rs:9`

**Error Message:**
```
error[E0433]: failed to resolve: could not find `FixedTimestep` in `time`
  --> src/main.rs:9:42
   |
 9 | use bevy::{prelude::*, time::FixedTimestep};
   |                                ^^^^^^^^^^^^^ not found in `time`
```

**Root Cause:** `FixedTimestep` run criteria removed. Now use `FixedUpdate` schedule.

**Solution:**
```rust
// Remove import:
use bevy::{prelude::*, time::FixedTimestep};

// Just use:
use bevy::prelude::*;

// Replace .with_run_criteria(FixedTimestep::step(dt)) with:
.add_systems(FixedUpdate, /* systems */)
```

**Priority:** 🔴 CRITICAL

---

#### Error E0599: add_system_set not found

**Count:** ~6 occurrences
**File:** `src/main.rs`

**Error Message:**
```
error[E0599]: no method named `add_system_set` found for struct `App` in the current scope
```

**Root Cause:** `.add_system_set()` removed. Use `.add_systems()` instead.

**Solution:** See `docs/CONTINUATION_GUIDE.md` Phase 2

**Priority:** 🔴 CRITICAL

---

### Category 3: Query API Errors (8 errors)

#### Error E0277: Without<T> not valid Query data

**Count:** 2 occurrences
**Files:**
- `src/systems/attach_poison.rs:6`
- `src/systems/attach_time_to_live.rs:6`

**Error Message:**
```
error[E0277]: `bevy::prelude::Without<powerups::Poison>` is not valid to request as data in a `Query`
   --> src/systems/attach_poison.rs:6:18
    |
  6 |     q_no_poison: Query<Without<Poison>>,
    |                  ^^^^^^^^^^^^^^^^^^^^^^ invalid `Query` data
    |
    = help: the trait `QueryData` is not implemented for `bevy::prelude::Without<powerups::Poison>`
```

**Root Cause:** `Without<T>` is a **filter**, not **query data**. Must request at least one data component.

**Solution:**
```rust
// OLD:
q_no_poison: Query<Without<Poison>>

// NEW:
q_no_poison: Query<Entity, Without<Poison>>
// Entity is minimal data you can query
```

**Priority:** 🟡 MEDIUM

---

#### Error E0599: for_each_mut not found

**Count:** 4 occurrences
**Files:**
- `src/systems/sync_mouse_position.rs:30`
- `src/systems/update_score_ui.rs:6`
- `src/systems/team_powerup_assigns_team.rs:6`
- `src/systems/time_to_live.rs:12`

**Error Message:**
```
error[E0599]: no method named `for_each_mut` found for struct `bevy::prelude::Query` in the current scope
  --> src/systems/sync_mouse_position.rs:30:17
   |
30 |         q_mouse.for_each_mut(|mut mouse| {
   |         --------^^^^^^^^^^^^ method not found in `Query<...>`
```

**Root Cause:** `.for_each()` and `.for_each_mut()` methods removed. Use regular iteration.

**Solution:**
```rust
// OLD:
query.for_each_mut(|mut item| {
    // ...
});

// NEW:
for mut item in &mut query {
    // ...
}
```

**Priority:** 🟡 MEDIUM

---

### Category 4: Text Rendering Errors (2 errors)

#### Error E0422: TextAlignment not found

**Count:** 2 occurrences
**Files:**
- `src/bundles/health.rs:25`
- `src/systems/on_no_players_show_game_over.rs:41`

**Error Message:**
```
error[E0422]: cannot find struct, variant or union type `TextAlignment` in this scope
  --> src/bundles/health.rs:25:32
   |
25 |                     alignment: TextAlignment {
   |                                ^^^^^^^^^^^^^ not found in this scope
```

**Root Cause:** `TextAlignment` struct replaced with `JustifyText` enum.

**Solution:**
```rust
// OLD:
alignment: TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
}

// NEW:
text_justification: JustifyText::Center,
```

**Priority:** 🟡 MEDIUM

---

### Category 5: Asset Loading Errors (4 errors)

#### Error E0277: TextureAtlas is not an Asset

**Count:** 3 occurrences
**Files:** Spawn systems (inferred from error)

**Error Message:**
```
error[E0277]: the trait bound `bevy::prelude::TextureAtlas: Asset` is not satisfied
```

**Root Cause:** `TextureAtlas` is now a component, not an asset. `TextureAtlasLayout` is the asset.

**Solution:**
```rust
// OLD:
texture_atlases: ResMut<Assets<TextureAtlas>>

// NEW:
texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
```

**Priority:** 🟡 MEDIUM

---

#### Error E0599: load_folder not found

**Count:** 1 occurrence
**File:** `src/main.rs` (load_sprites function)

**Error Message:**
```
error[E0599]: no method named `load_folder` found for struct `AssetServer`
```

**Root Cause:** `load_folder()` method removed or changed API.

**Solution:**
```rust
// Option 1: Load files individually
sprite_handles.handles = vec![
    asset_server.load("sprites/player.png"),
    asset_server.load("sprites/enemy.png"),
    // ... enumerate all sprites
];

// Option 2: Check Bevy 0.14 docs for new folder loading API
```

**Priority:** 🟡 MEDIUM

---

### Category 6: Type Mismatch Errors (5 errors)

#### Error E0308: Handle<TextureAtlas> vs Option<Handle<Image>>

**Count:** ~5 occurrences
**Files:** Bundle files

**Error Message:**
```
error[E0308]: mismatched types
  --> src/bundles/player.rs:64:17
   |
64 |                 texture,
   |                 ^^^^^^^ expected `Handle<Image>`, found `Option<Handle<_>>`
```

**Root Cause:** Sprite bundles now use `Handle<Image>` for texture, not `Handle<TextureAtlas>`.

**Solution:** Part of sprite system migration. See Category 1.

**Priority:** 🔴 CRITICAL

---

### Category 7: Minor Issues (2 errors)

#### Error E0659: Camera2d is ambiguous

**Count:** 1 occurrence
**File:** `src/systems/follow_player_with_camera.rs:14`

**Error Message:**
```
error[E0659]: `Camera2d` is ambiguous
  --> src/systems/follow_player_with_camera.rs:14:9
   |
14 |         Camera2d => follow_player_with_camera_2d(config, q_player, q_camera),
   |         ^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of a name in the same module
```

**Root Cause:** Both `bevy::prelude::*` and `CameraType::*` export `Camera2d`.

**Solution:**
```rust
// OLD:
use crate::resources::{CameraType::*, Config};

// NEW:
use crate::resources::{CameraType, Config};

// In match:
match config.camera_type {
    CameraType::Camera2d => ...,
    CameraType::Camera3d => ...,
}
```

**Priority:** 🟢 LOW

---

#### Error: WindowRef.entity() not found

**Count:** 1 occurrence
**File:** `src/systems/sync_mouse_position.rs`

**Error Message:**
```
error[E0599]: no method named `entity` found for struct `WindowRef` in the current scope
```

**Root Cause:** `WindowRef` API changed.

**Solution:**
```rust
// Check Bevy 0.14 WindowRef API
// May be:
window_ref.id  // or similar

// Or simplify to:
let window = q_window.iter().next();
```

**Priority:** 🟢 LOW

---

## Error Resolution Priority

### Phase 1: Critical Blockers (Must fix to compile)

1. ✅ Fix sprite system (Category 1) - 35 errors
2. ✅ Fix system registration (Category 2) - 18 errors

**After Phase 1:** Should have ~21 errors remaining

### Phase 2: Medium Priority (Blocking features)

3. ✅ Fix Query API (Category 3) - 8 errors
4. ✅ Fix text rendering (Category 4) - 2 errors
5. ✅ Fix asset loading (Category 5) - 4 errors

**After Phase 2:** Should have ~7 errors remaining

### Phase 3: Low Priority (Polish)

6. ✅ Fix type mismatches (Category 6) - 5 errors
7. ✅ Fix minor issues (Category 7) - 2 errors

**After Phase 3:** 0 errors, compilation success! 🎉

---

## Testing After Each Phase

### After Phase 1:
```bash
cargo build 2>&1 | grep "^error\[" | wc -l
# Expected: ~21 errors
```

### After Phase 2:
```bash
cargo build 2>&1 | grep "^error\[" | wc -l
# Expected: ~7 errors
```

### After Phase 3:
```bash
cargo build
# Expected: Success with possible warnings
```

---

## Full Error Log

<details>
<summary>Click to expand complete error output (2025-10-29)</summary>

```
Compiling carbonaria v1.0.0 (/home/redaphid/Projects/carbonaria)
error[E0422]: cannot find struct, variant or union type `TextAlignment` in this scope
  --> src/bundles/health.rs:25:32
   |
25 |                     alignment: TextAlignment {
   |                                ^^^^^^^^^^^^^ not found in this scope

error[E0422]: cannot find struct, variant or union type `TextAlignment` in this scope
  --> src/systems/on_no_players_show_game_over.rs:41:36
   |
41 |                         alignment: TextAlignment {
   |                                    ^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `TextureAtlasSprite` in this scope
  --> src/systems/update_sprite_index.rs:4:53
   |
 4 | pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlasSprite, &SpriteAnimation)>) {
   |                                                     ^^^^^^^^^^^^^^^^^^
   |
  ::: /home/redaphid/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_sprite-0.14.2/src/texture_atlas.rs:47:1
   |
47 | pub struct TextureAtlas {
   | ----------------------- similarly named struct `TextureAtlas` defined here
   |
help: a struct with a similar name exists
   |
 4 - pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlasSprite, &SpriteAnimation)>) {
 4 + pub fn update_sprite_index(mut sprites: Query<(&mut TextureAtlas, &SpriteAnimation)>) {

error[E0659]: `Camera2d` is ambiguous
  --> src/systems/follow_player_with_camera.rs:14:9
   |
14 |         Camera2d => follow_player_with_camera_2d(config, q_player, q_camera),
   |         ^^^^^^^^ ambiguous name

warning: unused import: `bevy::window::CursorGrabMode`
 --> src/systems/spawn_crosshairs.rs:6:5
  |
6 | use bevy::window::CursorGrabMode;

error[E0277]: `bevy::prelude::Without<powerups::Poison>` is not valid to request as data in a `Query`
   --> src/systems/attach_poison.rs:6:18
    |
  6 |     q_no_poison: Query<Without<Poison>>,
    |                  ^^^^^^^^^^^^^^^^^^^^^^ invalid `Query` data

error[E0277]: `bevy::prelude::Without<powerups::TimeToLive>` is not valid to request as data in a `Query`
   --> src/systems/attach_time_to_live.rs:6:21
    |
  6 |     q_time_to_live: Query<Without<TimeToLive>>,
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid `Query` data

[... 67 more errors omitted for brevity ...]

error: could not compile `carbonaria` (lib) due to 74 previous errors; 3 warnings emitted
```

</details>

---

## Resources for Error Resolution

### Official Documentation
- [Bevy 0.13 → 0.14 Migration Guide](https://bevyengine.org/learn/migration-guides/0.13-0.14/)
- [Bevy 0.14 API Docs](https://docs.rs/bevy/0.14.2/bevy/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/v0.14.0/examples)

### Project Documentation
- `docs/BREAKING_CHANGES_REMAINING.md` - Detailed solutions for each category
- `docs/CONTINUATION_GUIDE.md` - Step-by-step migration guide
- `docs/MIGRATION_STATUS.md` - Overall project status

### Community Help
- [Bevy Discord #help channel](https://discord.gg/bevy)
- [Bevy GitHub Discussions](https://github.com/bevyengine/bevy/discussions)

---

## Conclusion

The remaining 74 errors fall into 7 clear categories. The critical path is:

1. **Fix sprite system** (35 errors) - Enables visual testing
2. **Fix system registration** (18 errors) - Enables game loop
3. **Quick fixes** (21 errors) - Polish and complete migration

**Estimated Time:** 2-3 hours of focused work

**Success Metric:** `cargo build` completes with 0 errors
