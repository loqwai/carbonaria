# Breaking Changes Completed - Bevy 0.9 → 0.14

This document provides detailed information about all breaking API changes that have been successfully migrated in the Carbonaria codebase.

---

## Table of Contents

1. [Input System Migration](#1-input-system-migration)
2. [Window System Migration](#2-window-system-migration)
3. [Event System Migration](#3-event-system-migration)
4. [State System Migration](#4-state-system-migration)
5. [Ray API Migration](#5-ray-api-migration)
6. [KeyCode Enum Changes](#6-keycode-enum-changes)

---

## 1. Input System Migration

### Overview

**Bevy Version:** 0.10+
**Complexity:** 🟢 Low
**Files Affected:** 3

Bevy renamed the `Input<T>` resource to `ButtonInput<T>` for clarity. This affects all keyboard and mouse input handling.

### Changes Required

| Old API | New API |
|---------|---------|
| `Res<Input<KeyCode>>` | `Res<ButtonInput<KeyCode>>` |
| `Res<Input<MouseButton>>` | `Res<ButtonInput<MouseButton>>` |

### Migration Examples

#### Example 1: Player Movement System

**File:** `src/systems/move_player.rs`

**Before:**
```rust
use bevy::prelude::*;
use crate::{components::Player, events::MoveEvent};

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,  // ❌ Old API
    players: Query<Entity, With<Player>>,
    mut move_events: EventWriter<MoveEvent>,
) {
    for player in players.iter() {
        let mut directions: Vec<Vec3> = Vec::new();

        if keyboard_input.pressed(KeyCode::A) {  // ❌ Old KeyCode
            directions.push(Vec3::new(-1.0, 0.0, 0.0));
        }
        // ...
    }
}
```

**After:**
```rust
use bevy::prelude::*;
use crate::{components::Player, events::MoveEvent};

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,  // ✅ New API
    players: Query<Entity, With<Player>>,
    mut move_events: EventWriter<MoveEvent>,
) {
    for player in players.iter() {
        let mut directions: Vec<Vec3> = Vec::new();

        if keyboard_input.pressed(KeyCode::KeyA) {  // ✅ New KeyCode
            directions.push(Vec3::new(-1.0, 0.0, 0.0));
        }
        // ...
    }
}
```

**Key Points:**
- Resource type changes: `Input` → `ButtonInput`
- KeyCode enum changes: See [KeyCode section](#6-keycode-enum-changes) below
- Method names unchanged: `.pressed()`, `.just_pressed()`, `.just_released()` work the same

---

#### Example 2: Mouse Button Input

**File:** `src/systems/on_click_and_no_player_reset.rs`

**Before:**
```rust
use bevy::prelude::*;
use crate::{components::Player, AppState};

pub fn on_click_and_no_player_reset(
    players: Query<&Player>,
    buttons: Res<Input<MouseButton>>,  // ❌ Old API
    mut app_state: ResMut<State<AppState>>,  // ❌ Old State API (see below)
) {
    if buttons.pressed(MouseButton::Left) && players.is_empty() {
        app_state.restart().unwrap()  // ❌ Old State API
    }
}
```

**After:**
```rust
use bevy::prelude::*;
use crate::{components::Player, AppState};

pub fn on_click_and_no_player_reset(
    players: Query<&Player>,
    buttons: Res<ButtonInput<MouseButton>>,  // ✅ New API
    mut app_state: ResMut<NextState<AppState>>,  // ✅ New State API
) {
    if buttons.pressed(MouseButton::Left) && players.is_empty() {
        app_state.set(AppState::InGame);  // ✅ New State API
    }
}
```

**Key Points:**
- `Input<MouseButton>` → `ButtonInput<MouseButton>`
- Mouse button enum unchanged: `MouseButton::Left` still works
- State API also changed (see [State System section](#4-state-system-migration))

---

#### Example 3: KeyboardInput Event Handling

**File:** `src/systems/on_f_key_switch_ammo.rs`

**Before:**
```rust
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

pub fn on_f_key_switch_ammo(
    players: Query<&Children, With<Player>>,
    mut guns: Query<&mut ActiveAmmo>,
    mut keyboard_input: EventReader<KeyboardInput>,
) {
    for event in keyboard_input.iter() {  // ❌ Old iteration
        match event.state {
            bevy::input::ButtonState::Pressed => match event.key_code {
                Some(KeyCode::F) => {  // ❌ Option<KeyCode>
                    // Handle F key
                }
                _ => {}
            },
            _ => {}
        }
    }
}
```

**After:**
```rust
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

pub fn on_f_key_switch_ammo(
    players: Query<&Children, With<Player>>,
    mut guns: Query<&mut ActiveAmmo>,
    mut keyboard_input: EventReader<KeyboardInput>,
) {
    for event in keyboard_input.read() {  // ✅ New iteration
        if event.state.is_pressed() {  // ✅ New state check
            match event.key_code {  // ✅ Direct KeyCode (not Option)
                KeyCode::KeyF => {  // ✅ New KeyCode variant
                    // Handle F key
                }
                _ => {}
            }
        }
    }
}
```

**Key Points:**
- Event iteration: `.iter()` → `.read()` (Bevy 0.11+)
- State checking: `match event.state` → `event.state.is_pressed()`
- KeyCode field: `event.key_code` is now `KeyCode` directly, not `Option<KeyCode>`
- KeyCode variants: `KeyCode::F` → `KeyCode::KeyF`

---

### Why This Change?

Bevy renamed `Input` to `ButtonInput` to:
1. **Avoid confusion** with generic "input" term (which includes axes, gestures, etc.)
2. **Clarify semantics** - this specifically handles button-like inputs (digital on/off)
3. **Prepare for future input systems** - gamepad axes, touch gestures will have different types

---

## 2. Window System Migration

### Overview

**Bevy Version:** 0.11+
**Complexity:** 🟡 Medium
**Files Affected:** 3

Bevy replaced the `Windows` resource (plural) with a component-based system where windows are entities with `Window` components. This enables better multi-window support and more consistent ECS patterns.

### Changes Required

| Old API | New API |
|---------|---------|
| `Res<Windows>` | `Query<&Window>` or `Query<&mut Window>` |
| `windows.get_primary()` | `q_window.get_single()` or `q_window.iter().next()` |
| `windows.get(id)` | `q_window.get(entity)` |
| `window.set_cursor_visibility(bool)` | `window.cursor.visible = bool` |
| `window.set_maximized(bool)` | `window.mode = WindowMode::Windowed/Maximized` |

### Migration Examples

#### Example 1: Mouse Position Synchronization

**File:** `src/systems/sync_mouse_position.rs`

**Before:**
```rust
use bevy::{prelude::*, render::camera::RenderTarget, window::Windows};
use crate::components::MousePos;

pub fn sync_mouse_position(
    wnds: Res<Windows>,  // ❌ Old resource
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_mouse: Query<&mut Transform, With<MousePos>>,
) {
    let (camera, camera_transform) = q_camera.single();

    // Get the window
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()  // ❌ Get by WindowId
    } else {
        wnds.get_primary().unwrap()  // ❌ Get primary window
    };

    // Get cursor position
    if let Some(screen_pos) = wnd.cursor_position() {
        let Some(ray) = camera.viewport_to_world(camera_transform, screen_pos) else { return; };
        // ...
    }
}
```

**After:**
```rust
use bevy::{prelude::*, render::camera::RenderTarget};
use crate::components::MousePos;

pub fn sync_mouse_position(
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_window: Query<&Window>,  // ✅ Query windows as components
    mut q_mouse: Query<&mut Transform, With<MousePos>>,
) {
    let (camera, camera_transform) = q_camera.single();

    // Get the window
    let window = match camera.target {
        RenderTarget::Window(window_ref) => {
            q_window.get(window_ref.entity()).ok()  // ✅ Get by Entity
        }
        _ => q_window.iter().next(),  // ✅ Get any window
    };

    let Some(window) = window else { return; };

    // Get cursor position
    if let Some(screen_pos) = window.cursor_position() {
        let Some(ray) = camera.viewport_to_world(camera_transform, screen_pos) else { return; };
        // ...
    }
}
```

**Key Points:**
- `Res<Windows>` → `Query<&Window>` (windows are now components)
- `RenderTarget::Window(id)` now contains `WindowRef` with `.entity()` method
- `windows.get_primary()` → `q_window.iter().next()` (assumes single window)
- Better error handling with `Option` instead of `.unwrap()`

---

#### Example 2: Cursor Visibility

**File:** `src/systems/spawn_crosshairs.rs`

**Before:**
```rust
use bevy::prelude::*;

pub fn spawn_crosshairs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,  // ❌ Mutable access to Windows
    config: Res<Config>,
) {
    // Spawn crosshair sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("crosshairs.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(MousePos);

    // Hide system cursor
    let window = windows.get_primary_mut().unwrap();  // ❌ Get primary mutable
    window.set_cursor_visibility(false);  // ❌ Old method
}
```

**After:**
```rust
use bevy::prelude::*;

pub fn spawn_crosshairs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_window: Query<&mut Window>,  // ✅ Mutable query
    config: Res<Config>,
) {
    // Spawn crosshair sprite
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("crosshairs.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(MousePos);

    // Hide system cursor
    if let Ok(mut window) = q_window.get_single_mut() {  // ✅ Get single mutable
        window.cursor.visible = false;  // ✅ New field access
    }
}
```

**Key Points:**
- Mutable access: `ResMut<Windows>` → `Query<&mut Window>`
- Cursor API: `set_cursor_visibility(bool)` → `window.cursor.visible = bool`
- Better error handling with `if let Ok(...)`

---

#### Example 3: Window Mode

**File:** `src/systems/resize_window.rs`

**Before:**
```rust
use bevy::prelude::*;

pub fn resize_window(mut windows: ResMut<Windows>) {  // ❌ Old resource
    for window in windows.iter_mut() {  // ❌ Iterator over windows
        window.set_maximized(true);  // ❌ Old method
    }
}
```

**After:**
```rust
use bevy::prelude::*;
use bevy::window::WindowMode;

pub fn resize_window(mut q_window: Query<&mut Window>) {  // ✅ Query
    for mut window in q_window.iter_mut() {  // ✅ Query iterator
        window.mode = WindowMode::Windowed;  // ✅ Direct field assignment
    }
}
```

**Key Points:**
- Multi-window iteration: `windows.iter_mut()` → `q_window.iter_mut()`
- Window mode: `set_maximized(bool)` → `window.mode = WindowMode::*`
- Available modes: `Windowed`, `Maximized`, `Fullscreen`, `BorderlessFullscreen`

---

### Why This Change?

Bevy moved to component-based windows to:
1. **Consistent ECS patterns** - Windows are entities like everything else
2. **Better multi-window support** - Each window is a separate entity
3. **More flexible queries** - Can filter windows by components
4. **Enable future features** - Window-specific components (focus, minimization state, etc.)

---

## 3. Event System Migration

### Overview

**Bevy Version:** 0.11+
**Complexity:** 🟢 Low
**Files Affected:** 1

Bevy now requires all custom event types to derive the `Event` trait. This enables compile-time verification and improves type safety.

### Changes Required

All custom event structs must have `#[derive(Event)]`:

```rust
#[derive(Event)]
pub struct MyEvent {
    // fields...
}
```

### Migration Examples

**File:** `src/events/mod.rs`

**Before:**
```rust
use bevy::prelude::*;

// ❌ No derive macro
pub struct DamagerHitEvent {
    pub damager: Entity,
    pub target: Entity,
}

// ❌ No derive macro
pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}

// ❌ No derive macro
pub struct RotateEvent {
    pub who: Entity,
    pub rotation: Quat,
}

// ❌ No derive macro
pub struct DespawnEvent {
    pub entity: Entity,
}

// ❌ No derive macro
pub struct ShootEvent {
    pub gun: Entity,
}
```

**After:**
```rust
use bevy::prelude::*;

#[derive(Event)]  // ✅ Added
pub struct DamagerHitEvent {
    pub damager: Entity,
    pub target: Entity,
}

#[derive(Event)]  // ✅ Added
pub struct MoveEvent {
    pub who: Entity,
    pub direction: Vec3,
}

#[derive(Event)]  // ✅ Added
pub struct RotateEvent {
    pub who: Entity,
    pub rotation: Quat,
}

#[derive(Event)]  // ✅ Added
pub struct DespawnEvent {
    pub entity: Entity,
}

#[derive(Event)]  // ✅ Added
pub struct ShootEvent {
    pub gun: Entity,
}
```

### Event Usage (Unchanged)

Event readers and writers work the same way:

```rust
// Sending events
fn system_that_sends(mut events: EventWriter<MoveEvent>) {
    events.send(MoveEvent {
        who: entity,
        direction: Vec3::X,
    });
}

// Reading events
fn system_that_receives(mut events: EventReader<MoveEvent>) {
    for event in events.read() {  // Note: .iter() → .read() in Bevy 0.11+
        println!("Move event: {:?}", event);
    }
}
```

**Note:** Event reading uses `.read()` instead of `.iter()` in Bevy 0.11+.

### Why This Change?

The `Event` derive macro enables:
1. **Compile-time verification** - Ensures type is suitable for events
2. **Better error messages** - Clear errors if Event is not derived
3. **Future features** - Enables event ordering, filtering, replay systems
4. **Type safety** - Prevents accidental use of non-event types

---

## 4. State System Migration

### Overview

**Bevy Version:** 0.11+
**Complexity:** 🟡 Medium
**Files Affected:** 2

Bevy completely redesigned the state system for better type safety and ergonomics. States now use derive macros and a different transition API.

### Changes Required

| Old API | New API |
|---------|---------|
| `#[derive(Clone, Debug, Hash, Eq, PartialEq)]` | `#[derive(Clone, Debug, Hash, Eq, PartialEq, States, Default)]` |
| No default variant | `#[default]` attribute required |
| `ResMut<State<T>>` | `ResMut<NextState<T>>` (for transitions) |
| `state.set(value).unwrap()` | `state.set(value)` (no Result) |
| `state.push(value).unwrap()` | Stack states removed |
| `state.pop().unwrap()` | Stack states removed |
| `state.restart().unwrap()` | `state.set(current_state)` |

### Migration Examples

#### Example 1: State Definition

**File:** `src/main.rs`

**Before:**
```rust
use bevy::prelude::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]  // ❌ Missing States, Default
pub enum AppState {
    InGame,  // ❌ No default marker
}

fn main() {
    App::new()
        .add_state(AppState::InGame)  // ❌ Old state initialization
        // ...
        .run();
}
```

**After:**
```rust
use bevy::prelude::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq, States, Default)]  // ✅ Added States, Default
pub enum AppState {
    #[default]  // ✅ Marked default variant
    InGame,
}

fn main() {
    App::new()
        .init_state::<AppState>()  // ✅ New state initialization
        // ...
        .run();
}
```

**Key Points:**
- Must derive `States` and `Default`
- One variant must be marked `#[default]`
- Initialization: `add_state()` → `init_state::<T>()`

---

#### Example 2: State Transitions

**File:** `src/systems/on_click_and_no_player_reset.rs`

**Before:**
```rust
use bevy::prelude::*;
use crate::{components::Player, AppState};

pub fn on_click_and_no_player_reset(
    players: Query<&Player>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut app_state: ResMut<State<AppState>>,  // ❌ Old State type
) {
    if buttons.pressed(MouseButton::Left) && players.is_empty() {
        app_state.restart().unwrap();  // ❌ Old restart method
    }
}
```

**After:**
```rust
use bevy::prelude::*;
use crate::{components::Player, AppState};

pub fn on_click_and_no_player_reset(
    players: Query<&Player>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut app_state: ResMut<NextState<AppState>>,  // ✅ NextState for transitions
) {
    if buttons.pressed(MouseButton::Left) && players.is_empty() {
        app_state.set(AppState::InGame);  // ✅ Explicit state transition
    }
}
```

**Key Points:**
- Use `NextState<T>` for transitions (not `State<T>`)
- `restart()` removed - use `set(current_state)` instead
- `.set()` no longer returns `Result` - always succeeds

---

#### Example 3: Reading Current State

**Before:**
```rust
fn system(state: Res<State<AppState>>) {
    match state.current() {  // ❌ Old method
        AppState::InGame => { /* ... */ }
    }
}
```

**After:**
```rust
fn system(state: Res<State<AppState>>) {
    match state.get() {  // ✅ New method
        AppState::InGame => { /* ... */ }
    }
}
```

**Key Points:**
- Current state: `.current()` → `.get()`
- Reading state: Use `Res<State<T>>`
- Transitioning state: Use `ResMut<NextState<T>>`

---

### Why This Change?

Bevy redesigned states to:
1. **Better ergonomics** - Clearer separation between reading and transitioning
2. **Type safety** - `NextState` prevents accidental state reads during transitions
3. **Simplified API** - Removed error-prone stack states
4. **Future-proof** - Enables state transition events, enter/exit hooks

---

## 5. Ray API Migration

### Overview

**Bevy Version:** 0.13+
**Complexity:** 🟢 Low
**Files Affected:** 1

Bevy renamed `Ray` to `Ray3d` for clarity (distinguishing from potential `Ray2d`). The `direction` field also changed to a reference.

### Changes Required

| Old API | New API |
|---------|---------|
| `Ray` | `Ray3d` |
| `ray.direction` | `*ray.direction` (dereference) |

### Migration Example

**File:** `src/systems/sync_mouse_position.rs`

**Before:**
```rust
use bevy::prelude::*;

fn intersect_plane(ray: Ray, plane_origin: Vec3, plane_normal: Vec3) -> Option<f32> {  // ❌ Ray
    let denominator = plane_normal.dot(ray.direction);  // ❌ Direct access
    if denominator.abs() > f32::EPSILON {
        let distance = (plane_origin - ray.origin).dot(plane_normal) / denominator;
        if distance >= f32::EPSILON {
            return Some(distance);
        }
    }
    None
}

fn get_point_on_plane(ray: Ray, plane_origin: Vec3, plane_normal: Vec3) -> Option<Vec3> {  // ❌ Ray
    let distance = intersect_plane(ray, plane_origin, plane_normal)?;
    Some(ray.origin + ray.direction * distance)  // ❌ Direct access
}
```

**After:**
```rust
use bevy::prelude::*;

fn intersect_plane(ray: &Ray3d, plane_origin: Vec3, plane_normal: Vec3) -> Option<f32> {  // ✅ Ray3d reference
    let denominator = plane_normal.dot(*ray.direction);  // ✅ Dereference
    if denominator.abs() > f32::EPSILON {
        let distance = (plane_origin - ray.origin).dot(plane_normal) / denominator;
        if distance >= f32::EPSILON {
            return Some(distance);
        }
    }
    None
}

fn get_point_on_plane(ray: &Ray3d, plane_origin: Vec3, plane_normal: Vec3) -> Option<Vec3> {  // ✅ Ray3d reference
    let distance = intersect_plane(ray, plane_origin, plane_normal)?;
    Some(ray.origin + *ray.direction * distance)  // ✅ Dereference
}
```

**Key Points:**
- Type rename: `Ray` → `Ray3d`
- Direction field: Now returns `&Dir3`, requires dereference with `*`
- Take by reference: Functions should take `&Ray3d` not `Ray3d` (avoid copies)

### Why This Change?

The Ray API changed to:
1. **Distinguish 2D/3D** - `Ray2d` and `Ray3d` are distinct types
2. **Memory efficiency** - Direction is normalized, stored efficiently
3. **Type safety** - `Dir3` guarantees normalized direction vectors

---

## 6. KeyCode Enum Changes

### Overview

**Bevy Version:** 0.13+
**Complexity:** 🟢 Low
**Files Affected:** 2

Bevy changed the `KeyCode` enum to use more explicit names for letter/number keys to avoid conflicts with symbols.

### Changes Required

| Old KeyCode | New KeyCode | Notes |
|-------------|-------------|-------|
| `KeyCode::A` through `KeyCode::Z` | `KeyCode::KeyA` through `KeyCode::KeyZ` | All letter keys |
| `KeyCode::Key0` through `KeyCode::Key9` | `KeyCode::Digit0` through `KeyCode::Digit9` | Number row keys |
| `KeyCode::F` | `KeyCode::KeyF` | Example: F key |
| `KeyCode::V` | `KeyCode::KeyV` | Example: V key |
| Special keys (Escape, Space, etc.) | Unchanged | No prefix added |

### Migration Examples

**File:** `src/systems/move_player.rs`

**Before:**
```rust
if keyboard_input.pressed(KeyCode::A) {  // ❌ Old
    directions.push(Vec3::new(-1.0, 0.0, 0.0));
}
if keyboard_input.pressed(KeyCode::D) {  // ❌ Old
    directions.push(Vec3::new(1.0, 0.0, 0.0));
}
if keyboard_input.pressed(KeyCode::W) {  // ❌ Old
    directions.push(Vec3::new(0.0, 1.0, 0.0));
}
if keyboard_input.pressed(KeyCode::S) {  // ❌ Old
    directions.push(Vec3::new(0.0, -1.0, 0.0));
}
```

**After:**
```rust
if keyboard_input.pressed(KeyCode::KeyA) {  // ✅ New
    directions.push(Vec3::new(-1.0, 0.0, 0.0));
}
if keyboard_input.pressed(KeyCode::KeyD) {  // ✅ New
    directions.push(Vec3::new(1.0, 0.0, 0.0));
}
if keyboard_input.pressed(KeyCode::KeyW) {  // ✅ New
    directions.push(Vec3::new(0.0, 1.0, 0.0));
}
if keyboard_input.pressed(KeyCode::KeyS) {  // ✅ New
    directions.push(Vec3::new(0.0, -1.0, 0.0));
}
```

### Complete Mapping

**Letter Keys:**
```rust
A → KeyA    B → KeyB    C → KeyC    D → KeyD
E → KeyE    F → KeyF    G → KeyG    H → KeyH
I → KeyI    J → KeyJ    K → KeyK    L → KeyL
M → KeyM    N → KeyN    O → KeyO    P → KeyP
Q → KeyQ    R → KeyR    S → KeyS    T → KeyT
U → KeyU    V → KeyV    W → KeyW    X → KeyX
Y → KeyY    Z → KeyZ
```

**Number Keys (Top Row):**
```rust
Key0 → Digit0    Key1 → Digit1    Key2 → Digit2
Key3 → Digit3    Key4 → Digit4    Key5 → Digit5
Key6 → Digit6    Key7 → Digit7    Key8 → Digit8
Key9 → Digit9
```

**Special Keys (Unchanged):**
```rust
Escape, Space, Enter, Tab, Backspace,
Left, Right, Up, Down,
LShift, RShift, LControl, RControl,
LAlt, RAlt, etc.
```

### Why This Change?

The KeyCode enum changed to:
1. **Avoid ambiguity** - `KeyCode::Key1` could mean the number or F1
2. **Match standards** - Aligns with web/OS keyboard APIs
3. **Future-proof** - Enables support for different keyboard layouts

---

## Summary Table

| Migration | Complexity | Files | Time | Status |
|-----------|-----------|-------|------|--------|
| Input System | 🟢 Low | 3 | 10 min | ✅ |
| Window System | 🟡 Medium | 3 | 15 min | ✅ |
| Event System | 🟢 Low | 1 | 5 min | ✅ |
| State System | 🟡 Medium | 2 | 10 min | ✅ |
| Ray API | 🟢 Low | 1 | 5 min | ✅ |
| KeyCode Enum | 🟢 Low | 2 | 5 min | ✅ |
| **TOTAL** | | **10** | **50 min** | ✅ |

---

## Next Steps

With these migrations complete, we can now tackle the more complex remaining changes:

1. **Sprite System** (TextureAtlasSprite → Sprite + TextureAtlas)
2. **System Registration** (SystemSet → add_systems)
3. **Text Rendering** (TextAlignment changes)
4. **Asset Loading** (Handle types)

See `docs/BREAKING_CHANGES_REMAINING.md` for details on these migrations.
