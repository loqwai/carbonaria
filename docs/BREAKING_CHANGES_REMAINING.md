# Breaking Changes Remaining - Bevy 0.9 → 0.14

This document details all remaining API migrations needed to complete the Bevy 0.14 upgrade.

**Current Status:** 74 compilation errors remaining
**Estimated Time:** 2-3 hours

---

## Priority Order

1. 🔴 **CRITICAL:** [Sprite System Migration](#1-sprite-system-migration) (1-1.5 hours)
2. 🔴 **CRITICAL:** [System Registration API](#2-system-registration-api) (30-45 min)
3. 🟡 **MEDIUM:** [Query API Changes](#3-query-api-changes) (20 min)
4. 🟡 **MEDIUM:** [Text Rendering API](#4-text-rendering-api) (15 min)
5. 🟡 **MEDIUM:** [Asset Loading API](#5-asset-loading-api) (20 min)
6. 🟢 **LOW:** [Minor Fixes](#6-minor-fixes) (15 min)

---

## 1. Sprite System Migration

### Overview

**Bevy Version:** 0.13+
**Complexity:** ⚠️ **VERY HIGH**
**Estimated Time:** 1-1.5 hours
**Files Affected:** ~15

This is the **most complex and critical** migration. Bevy completely redesigned sprite rendering:
- `SpriteSheetBundle` removed
- `TextureAtlasSprite` removed
- New pattern: `SpriteBundle` + `TextureAtlas` component + `Sprite` component

**Why This Is Blocking:** Cannot test game visually until sprites render. All entities (player, enemies, bullets) use sprites.

### Key Changes

| Old API | New API |
|---------|---------|
| `SpriteSheetBundle` | `(SpriteBundle, TextureAtlas)` |
| `TextureAtlasSprite` | `Sprite` component + `TextureAtlas` component |
| `TextureAtlasSprite::new(index)` | `Sprite { ..default() }` + `TextureAtlas { index, layout }` |
| `sprite.index = 5` | `atlas.index = 5` |
| `sprite.custom_size` | `sprite.custom_size` (unchanged) |
| `sprite.color` | `sprite.color` (unchanged) |
| `sprite.flip_x / flip_y` | `sprite.flip_x / flip_y` (unchanged) |

### Detailed Migration Guide

#### Step 1: Understand the New Pattern

**OLD PATTERN (Bevy 0.9-0.12):**
```rust
commands.spawn(SpriteSheetBundle {
    sprite: TextureAtlasSprite {
        index: 0,
        custom_size: Some(Vec2::new(32.0, 32.0)),
        ..default()
    },
    texture_atlas: texture_atlas_handle,
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..default()
});
```

**NEW PATTERN (Bevy 0.13+):**
```rust
commands.spawn((
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        texture: texture_handle,  // Changed: now single image handle
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
    TextureAtlas {
        index: 0,
        layout: atlas_layout_handle,  // New: Handle<TextureAtlasLayout>
    },
));
```

**Key Differences:**
1. Bundle is now a tuple: `(SpriteBundle, TextureAtlas)`
2. Sprite index moved from `sprite.index` → `TextureAtlas { index }`
3. Atlas layout is separate: `Handle<TextureAtlasLayout>` (created once, reused)
4. Texture is now a single image handle on the sprite

---

#### Step 2: Fix Bundle Definitions

**Files to Update:**
- `src/bundles/player.rs`
- `src/bundles/mob.rs`
- `src/bundles/mech.rs`
- `src/bundles/bullet.rs`
- `src/bundles/chest.rs`

**Example: Player Bundle**

**File:** `src/bundles/player.rs`

**BEFORE:**
```rust
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_sheet_bundle: SpriteSheetBundle,  // ❌ Removed
    pub health: Health,
    pub speed: Speed,
    // ... other components
}

impl PlayerBundle {
    pub fn new(texture_atlas: Handle<TextureAtlas>, ...) -> Self {
        Self {
            player: Player,
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {  // ❌ Removed
                    index: 0,
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                texture_atlas,  // ❌ Type changed
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            health: Health(100.0),
            speed: Speed(5.0),
            // ... other components
        }
    }
}
```

**AFTER (Option 1: Separate Components):**
```rust
use bevy::prelude::*;

// Note: No longer a Bundle - just individual components
pub struct PlayerComponents;

impl PlayerComponents {
    pub fn spawn(
        commands: &mut Commands,
        texture: Handle<Image>,
        atlas_layout: Handle<TextureAtlasLayout>,
        position: Vec3,
    ) -> Entity {
        commands.spawn((
            // Core marker
            Player,

            // Sprite rendering
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                texture,  // ✅ Handle<Image>
                transform: Transform::from_xyz(position.x, position.y, 10.0),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: atlas_layout,  // ✅ Handle<TextureAtlasLayout>
            },

            // Game stats
            Health(100.0),
            Speed(5.0),
            RateOfFire(0.1),
            Team(0),
            Points(0),

            // Components for other systems
            SpriteAnimation {
                num_angles: 8,
                num_frames_per_angle: 4,
                frames_to_advance_per_tick: 0.1,
                current_angle: 0,
                current_frame: 0.0,
            },

            // Physics
            Collider::ball(16.0),
            RigidBody::KinematicPositionBased,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id()
    }
}
```

**AFTER (Option 2: Keep Bundle Pattern):**
```rust
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub speed: Speed,
    pub rate_of_fire: RateOfFire,
    pub team: Team,
    pub points: Points,
    pub sprite_animation: SpriteAnimation,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    // Note: sprite and transform NOT included - added separately
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            player: Player,
            health: Health(100.0),
            speed: Speed(5.0),
            rate_of_fire: RateOfFire(0.1),
            team: Team(0),
            points: Points(0),
            sprite_animation: SpriteAnimation {
                num_angles: 8,
                num_frames_per_angle: 4,
                frames_to_advance_per_tick: 0.1,
                current_angle: 0,
                current_frame: 0.0,
            },
            collider: Collider::ball(16.0),
            rigid_body: RigidBody::KinematicPositionBased,
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

// Usage:
commands.spawn((
    PlayerBundle::new(),
    SpriteBundle { /* ... */ },
    TextureAtlas { /* ... */ },
));
```

**Recommendation:** Use **Option 2** (keep bundle pattern) as it requires minimal changes to spawn systems.

---

#### Step 3: Update Spawn Systems

**Files to Update:**
- `src/systems/spawn_player.rs`
- `src/systems/spawn_mobs.rs`
- `src/systems/spawn_mechs.rs`
- Possibly others that spawn visual entities

**BEFORE:**
```rust
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(48.0, 48.0),
        8,  // columns
        4,  // rows
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(PlayerBundle::new(texture_atlas_handle, /* ... */))
        .with_children(|parent| {
            // Spawn children (gun, UI, etc.)
        });
}
```

**AFTER:**
```rust
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the sprite sheet image
    let texture_handle = asset_server.load("player.png");

    // Create the atlas layout (defines grid structure)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(48, 48),  // ✅ Changed: UVec2 instead of Vec2
        8,  // columns
        4,  // rows
        None,  // padding
        None,  // offset
    );
    let layout_handle = texture_atlas_layouts.add(layout);

    commands
        .spawn((
            PlayerBundle::new(),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(48.0, 48.0)),
                    ..default()
                },
                texture: texture_handle,  // ✅ Single image handle
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: layout_handle,  // ✅ Layout handle
            },
        ))
        .with_children(|parent| {
            // Spawn children (gun, UI, etc.)
        });
}
```

**Key Changes:**
1. `ResMut<Assets<TextureAtlas>>` → `ResMut<Assets<TextureAtlasLayout>>`
2. `TextureAtlas::from_grid()` → `TextureAtlasLayout::from_grid()`
3. Grid size: `Vec2` → `UVec2` (unsigned integers)
4. Add layout to assets: `texture_atlas_layouts.add(layout)`
5. Spawn with tuple: `(PlayerBundle, SpriteBundle, TextureAtlas)`

---

#### Step 4: Fix Sprite Animation System

**File:** `src/systems/update_sprite_index.rs`

This system updates sprite frames for animation.

**BEFORE:**
```rust
use bevy::prelude::*;
use crate::components::SpriteAnimation;

pub fn update_sprite_index(
    mut sprites: Query<(&mut TextureAtlasSprite, &SpriteAnimation)>  // ❌ Removed
) {
    for (mut sprite, animation) in &mut sprites {
        sprite.index = calculate_index(animation);  // ❌ Wrong component
    }
}
```

**AFTER:**
```rust
use bevy::prelude::*;
use crate::components::SpriteAnimation;

pub fn update_sprite_index(
    mut sprites: Query<(&mut TextureAtlas, &SpriteAnimation)>  // ✅ TextureAtlas component
) {
    for (mut atlas, animation) in &mut sprites {
        atlas.index = calculate_index(animation);  // ✅ Update atlas.index
    }
}

fn calculate_index(animation: &SpriteAnimation) -> usize {
    let frame = animation.current_frame as usize;
    let angle = animation.current_angle;
    angle * animation.num_frames_per_angle + frame
}
```

**Key Changes:**
1. Query component: `TextureAtlasSprite` → `TextureAtlas`
2. Index field: `sprite.index` → `atlas.index`
3. Other sprite properties (color, flip, etc.) now on `Sprite` component

---

#### Step 5: Optimize Asset Loading (Optional but Recommended)

**Problem:** Creating a new `TextureAtlasLayout` for each entity is inefficient.

**Solution:** Create layouts once at startup, store in a resource.

**File:** `src/resources/mod.rs` (or create new file)

```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteAtlases {
    pub player_layout: Handle<TextureAtlasLayout>,
    pub mob_layout: Handle<TextureAtlasLayout>,
    pub mech_layout: Handle<TextureAtlasLayout>,
    pub bullet_layout: Handle<TextureAtlasLayout>,
}

// In a startup system:
fn setup_sprite_atlases(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(48, 48), 8, 4, None, None)
    );

    let mob_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 4, None, None)
    );

    // ... other layouts

    commands.insert_resource(SpriteAtlases {
        player_layout,
        mob_layout,
        mech_layout,
        bullet_layout,
    });
}

// Usage in spawn systems:
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlases: Res<SpriteAtlases>,  // ✅ Reuse layout
) {
    commands.spawn((
        PlayerBundle::new(),
        SpriteBundle {
            texture: asset_server.load("player.png"),
            // ...
        },
        TextureAtlas {
            index: 0,
            layout: atlases.player_layout.clone(),  // ✅ Clone handle
        },
    ));
}
```

---

### Summary of Sprite System Changes

**Files Requiring Changes:**

| File | Change Type | Priority |
|------|-------------|----------|
| `src/bundles/player.rs` | Bundle refactor | 🔴 Critical |
| `src/bundles/mob.rs` | Bundle refactor | 🔴 Critical |
| `src/bundles/mech.rs` | Bundle refactor | 🔴 Critical |
| `src/bundles/bullet.rs` | Bundle refactor | 🔴 Critical |
| `src/bundles/chest.rs` | Bundle refactor | 🔴 Critical |
| `src/systems/spawn_player.rs` | Spawn logic | 🔴 Critical |
| `src/systems/spawn_mobs.rs` | Spawn logic | 🔴 Critical |
| `src/systems/spawn_mechs.rs` | Spawn logic | 🔴 Critical |
| `src/systems/spawn_powerups.rs` | Spawn logic | 🟡 Medium |
| `src/systems/update_sprite_index.rs` | Query type | 🔴 Critical |
| `src/main.rs` | Asset loading | 🟡 Medium |

**Estimated Time:** 1-1.5 hours if done carefully

**Testing:** After this migration, run `cargo build`. If it compiles, run `cargo run` and verify sprites render.

---

## 2. System Registration API

### Overview

**Bevy Version:** 0.10+
**Complexity:** ⚠️ **HIGH**
**Estimated Time:** 30-45 minutes
**Files Affected:** 1-2 (but ~200 lines of changes)

Bevy completely redesigned system scheduling. The old `SystemSet` builder pattern is replaced with schedules and system combinators.

### Key Changes

| Old API | New API |
|---------|---------|
| `SystemSet::on_update(State)` | `Update` schedule with `run_if(in_state())` |
| `.with_system(sys)` | `app.add_systems(Schedule, sys)` |
| `.label("name")` | System in named set or `.in_set()` |
| `.before(label)` / `.after(label)` | `.before(sys)` / `.after(sys)` or use tuples |
| `FixedTimestep::step(dt)` | `FixedUpdate` schedule (runs at fixed rate) |
| `.with_run_criteria(criteria)` | `.run_if(condition)` |

### Migration Strategy

**Current Code Structure (main.rs, lines 33-150):**
```rust
// Old pattern
let ui_system_set = SystemSet::on_update(AppState::InGame)
    .with_system(systems::update_compass)
    .with_system(systems::update_score_ui)
    // ... many more systems

let game_loop_system_set = SystemSet::on_update(AppState::InGame)
    .label("game_loop_system_set")
    .with_system(systems::count_ticks)
    .with_system(systems::move_player.after(...))
    // ... many more systems
    .with_run_criteria(FixedTimestep::step(TIME_STEP))

App::new()
    .add_system_set(ui_system_set)
    .add_system_set(game_loop_system_set)
    // ...
```

**New Pattern:**
```rust
App::new()
    // UI systems (run every frame)
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

    // Game logic (runs at fixed 60 FPS)
    .add_systems(FixedUpdate, (
        // Powerup computation
        (
            systems::powerup_defaulter::<Speed>,
            systems::powerup_mather::<Speed>,
        ).chain(),
        (
            systems::powerup_defaulter::<Health>,
            systems::powerup_mather::<Health>,
        ).chain(),
        // ... other powerup systems

        // Game loop
        systems::count_ticks,
        (
            systems::move_player,
            systems::move_bullet,
        ).chain().before(systems::move_thing),
        systems::move_thing,
        // ... rest of game loop
    ).run_if(in_state(AppState::InGame)))

    // Startup systems
    .add_systems(OnEnter(AppState::InGame), (
        systems::load_sprites,
        systems::spawn_camera,
        systems::spawn_player,
        systems::spawn_ui,
        systems::spawn_crosshairs,
        systems::spawn_lights,
    ))
```

### Detailed Migration

#### Step 1: Understand Schedules

Bevy 0.10+ has multiple built-in schedules:

- **`Startup`** - Runs once at app start
- **`Update`** - Runs every frame (variable timestep)
- **`FixedUpdate`** - Runs at fixed rate (default 64 Hz)
- **`OnEnter(State)`** - Runs when entering a state
- **`OnExit(State)`** - Runs when exiting a state

#### Step 2: Convert UI Systems

**BEFORE:**
```rust
let ui_system_set = SystemSet::on_update(AppState::InGame)
    .with_system(systems::update_compass)
    .with_system(systems::update_score_ui)
    .with_system(systems::update_health_ui)
    // ... 8 more systems
```

**AFTER:**
```rust
app.add_systems(Update, (
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

**Key Points:**
- Tuple syntax: `(sys1, sys2, sys3)`
- State condition: `.run_if(in_state(AppState::InGame))`
- Systems run in parallel by default (no ordering)

#### Step 3: Convert Powerup Systems

**BEFORE:**
```rust
let compute_powerups_system_set = SystemSet::on_update(AppState::InGame)
    .label("compute_powerups_system_set")
    .with_system(systems::powerup_defaulter::<Speed>)
    .with_system(systems::powerup_mather::<Speed>.after(systems::powerup_defaulter::<Speed>))
    .with_system(systems::powerup_defaulter::<Health>)
    .with_system(systems::powerup_mather::<Health>.after(systems::powerup_defaulter::<Health>))
    // ... 4 more stat types
    .with_run_criteria(FixedTimestep::step(TIME_STEP));
```

**AFTER:**
```rust
app.add_systems(FixedUpdate, (
    // Speed powerups
    (
        systems::powerup_defaulter::<Speed>,
        systems::powerup_mather::<Speed>,
    ).chain(),

    // Health powerups
    (
        systems::powerup_defaulter::<Health>,
        systems::powerup_mather::<Health>,
    ).chain(),

    // RateOfFire powerups
    (
        systems::powerup_defaulter::<RateOfFire>,
        systems::powerup_mather::<RateOfFire>,
    ).chain(),

    // TimeToLive powerups
    (
        systems::powerup_defaulter::<TimeToLive>,
        systems::powerup_mather::<TimeToLive>,
    ).chain(),

    // Poison powerups
    (
        systems::powerup_defaulter::<Poison>,
        systems::powerup_mather::<Poison>,
    ).chain(),

    // AmmoCount powerups
    (
        systems::powerup_defaulter::<AmmoCount>,
        systems::powerup_mather::<AmmoCount>,
    ).chain(),
).run_if(in_state(AppState::InGame)))
```

**Key Points:**
- `.chain()` forces sequential execution (defaulter → mather)
- Fixed timestep: Use `FixedUpdate` schedule instead of run criteria
- Separate tuples for each stat type

#### Step 4: Convert Game Loop Systems

**BEFORE:**
```rust
let game_loop_system_set = SystemSet::on_update(AppState::InGame)
    .label("game_loop_system_set")
    .after("compute_powerups_system_set")
    .with_system(systems::count_ticks)
    .with_system(systems::move_player)
    .with_system(systems::move_bullet)
    .with_system(systems::move_thing.after(systems::move_player).after(systems::move_bullet))
    // ... 30+ more systems with complex ordering
    .with_run_criteria(FixedTimestep::step(TIME_STEP));
```

**AFTER (Simplified Example):**
```rust
app.add_systems(FixedUpdate, (
    systems::count_ticks,

    // Movement phase
    (
        systems::move_player,
        systems::move_bullet,
    ).before(systems::move_thing),
    systems::move_thing,

    // Rotation phase
    (
        systems::player_aimables_aim_at_cursor,
        systems::chaser_aimables_aim_at_other_teams,
    ).before(systems::rotate_thing),
    systems::rotate_thing,

    // Shooting phase
    (
        systems::on_left_click_shoot,
        systems::mob_shoot,
    ).before(systems::shoot_gun),
    systems::shoot_gun,

    // Combat
    systems::poison,
    systems::on_0_health_kill,

    // Spawning
    systems::spawn_mobs,
    systems::spawn_mechs,
    systems::spawn_powerups,

    // TTL
    systems::time_to_live,
).run_if(in_state(AppState::InGame)).after(/* powerup systems */))
```

**Ordering Combinators:**
- `.chain()` - Sequential execution A → B → C
- `.before(sys)` - Run before specific system
- `.after(sys)` - Run after specific system
- No combinator - Parallel execution

#### Step 5: Convert Cleanup Systems

**BEFORE:**
```rust
let game_loop_cleanup_system_set = SystemSet::on_update(AppState::InGame)
    .after("game_loop_system_set")
    .with_system(systems::consume_despawn_entity_events)
    .with_run_criteria(FixedTimestep::step(TIME_STEP));
```

**AFTER:**
```rust
app.add_systems(FixedUpdate,
    systems::consume_despawn_entity_events
        .run_if(in_state(AppState::InGame))
        .after(/* game loop systems */)
)
```

#### Step 6: Convert Startup Systems

**BEFORE:**
```rust
let startup_system_set = SystemSet::on_enter(AppState::InGame)
    .with_system(systems::load_sprites)
    .with_system(systems::spawn_camera)
    .with_system(systems::spawn_player)
    .with_system(systems::load_mech_walking_animation)
    .with_system(systems::spawn_ui)
    .with_system(systems::spawn_crosshairs)
    .with_system(systems::spawn_lights);
```

**AFTER:**
```rust
app.add_systems(OnEnter(AppState::InGame), (
    systems::load_sprites,
    systems::spawn_camera,
    systems::spawn_player,
    systems::load_mech_walking_animation,
    systems::spawn_ui,
    systems::spawn_crosshairs,
    systems::spawn_lights,
))
```

---

### Complete main.rs Example

See `docs/CONTINUATION_GUIDE.md` for full `main.rs` rewrite example.

---

## 3. Query API Changes

### Query::for_each() Removed

**Error:** `no method named for_each found for struct Query`

**BEFORE:**
```rust
query.for_each(|item| {
    // process item
});

query.for_each_mut(|mut item| {
    // mutate item
});
```

**AFTER:**
```rust
for item in &query {
    // process item
}

for mut item in &mut query {
    // mutate item
}
```

**Files Affected:**
- `src/systems/sync_mouse_position.rs` (line 30)
- `src/systems/update_score_ui.rs` (line 6)
- `src/systems/team_powerup_assigns_team.rs` (line 6)
- `src/systems/time_to_live.rs` (line 12)

---

### Query<Without<T>> Invalid

**Error:** `Without<T>` is not valid to request as data in a `Query`

**BEFORE:**
```rust
fn system(q: Query<Without<Poison>>) {  // ❌ Wrong
    // ...
}
```

**AFTER:**
```rust
fn system(q: Query<Entity, Without<Poison>>) {  // ✅ Correct
    // Must query something (Entity is minimal)
}
```

**Explanation:** `Without<T>` is a **filter**, not **data**. Queries must request at least one data item.

**Files Affected:**
- `src/systems/attach_poison.rs` (line 6)
- `src/systems/attach_time_to_live.rs` (line 6)

---

## 4. Text Rendering API

### TextAlignment Structure Changed

**Error:** `cannot find struct TextAlignment in this scope`

**BEFORE:**
```rust
alignment: TextAlignment {
    vertical: VerticalAlign::Center,
    horizontal: HorizontalAlign::Center,
}
```

**AFTER:**
```rust
// TextAlignment removed, use JustifyText instead
text_style: TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size: 60.0,
    color: Color::WHITE,
},
text_justification: JustifyText::Center,  // ✅ New field
```

**Files Affected:**
- `src/bundles/health.rs` (line 25)
- `src/systems/on_no_players_show_game_over.rs` (line 41)

---

## 5. Asset Loading API

### Load Folder Removed

**Error:** `method load_folder not found`

**BEFORE:**
```rust
fn load_sprites(mut sprite_handles: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("sprites").unwrap();
}
```

**AFTER:**
```rust
fn load_sprites(mut sprite_handles: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    // Option 1: Load specific files
    sprite_handles.handles = vec![
        asset_server.load("sprites/player.png"),
        asset_server.load("sprites/enemy.png"),
        // ... list all sprites
    ];

    // Option 2: Use AssetServer::load_folder (if still needed)
    // Check Bevy 0.14 docs for current API
}
```

---

## 6. Minor Fixes

### Camera2d Import Ambiguity

**Error:** `Camera2d is ambiguous`

**File:** `src/systems/follow_player_with_camera.rs`

**BEFORE:**
```rust
use bevy::prelude::*;
use crate::resources::{CameraType::*, Config};

match config.camera_type {
    Camera2d => ...,  // ❌ Ambiguous
    Camera3d => ...,
}
```

**AFTER:**
```rust
use bevy::prelude::*;
use crate::resources::{CameraType, Config};  // ✅ Don't glob import

match config.camera_type {
    CameraType::Camera2d => ...,  // ✅ Fully qualified
    CameraType::Camera3d => ...,
}
```

---

### WindowRef.entity() Method

**Error:** `no method named entity found`

**BEFORE:**
```rust
q_window.get(window_ref.entity()).ok()  // ❌ Wrong method
```

**AFTER:**
```rust
// Check Bevy 0.14 WindowRef API - may be:
q_window.get(window_ref.id).ok()  // or
q_window.iter().next()  // simpler
```

---

## Summary Checklist

Use this to track migration progress:

- [ ] **Sprite System** (1-1.5 hours)
  - [ ] Update PlayerBundle
  - [ ] Update MobBundle
  - [ ] Update MechBundle
  - [ ] Update BulletBundle
  - [ ] Update ChestBundle
  - [ ] Update spawn_player system
  - [ ] Update spawn_mobs system
  - [ ] Update spawn_mechs system
  - [ ] Update update_sprite_index system
  - [ ] Create SpriteAtlases resource (optional optimization)

- [ ] **System Registration** (30-45 min)
  - [ ] Convert UI systems to Update schedule
  - [ ] Convert powerup systems to FixedUpdate
  - [ ] Convert game loop systems to FixedUpdate
  - [ ] Convert cleanup systems
  - [ ] Convert startup systems to OnEnter
  - [ ] Add state run conditions
  - [ ] Fix system ordering with .before()/.after()/.chain()

- [ ] **Query API** (20 min)
  - [ ] Replace .for_each() with for loops
  - [ ] Fix Query<Without<T>> to Query<Entity, Without<T>>

- [ ] **Text API** (15 min)
  - [ ] Replace TextAlignment with JustifyText
  - [ ] Update health bundle
  - [ ] Update game over system

- [ ] **Asset Loading** (20 min)
  - [ ] Replace load_folder() or list files explicitly

- [ ] **Minor Fixes** (15 min)
  - [ ] Fix Camera2d import ambiguity
  - [ ] Fix WindowRef.entity() calls

- [ ] **Testing** (30 min)
  - [ ] cargo build succeeds
  - [ ] cargo run launches game
  - [ ] Sprites render correctly
  - [ ] Movement works
  - [ ] Shooting works
  - [ ] UI updates
  - [ ] No performance regression

**Total Estimated Time:** 2-3 hours

---

## Next Steps

See `docs/CONTINUATION_GUIDE.md` for step-by-step instructions to continue the migration.
