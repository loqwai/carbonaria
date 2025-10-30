# Testing Strategy - Bevy 0.14 Migration

This document outlines the comprehensive testing strategy to validate the Bevy 0.14 migration.

---

## Testing Phases

### Phase 1: Compilation Testing ✅
**Goal:** Verify code compiles without errors

```bash
# Clean build
cargo clean
cargo build --release

# Expected: Success (0 errors)
# Warnings are acceptable but should be reviewed
```

**Success Criteria:**
- [ ] No compilation errors
- [ ] All dependencies resolve
- [ ] Build completes in reasonable time (<5 minutes)

---

### Phase 2: Launch Testing
**Goal:** Verify game launches and initializes correctly

```bash
# Run in debug mode (more error info)
cargo run

# Monitor console for:
# - Asset loading errors
# - System initialization errors
# - Panics or crashes
```

**Success Criteria:**
- [ ] Game window opens
- [ ] Window has correct title
- [ ] No panic on startup
- [ ] Initial game state loads (AppState::InGame)
- [ ] Console shows no errors
- [ ] Camera initializes
- [ ] Background renders (if any)

**Common Issues:**
- **Asset not found:** Check `assets/` folder paths
- **Panic in system:** Check system order dependencies
- **Black screen:** Check camera Z-position and sprite Z-positions

---

### Phase 3: Visual Testing
**Goal:** Verify all sprites and UI render correctly

**Test Checklist:**

#### 3.1 Player Rendering
- [ ] Player sprite loads
- [ ] Player sprite is correct size
- [ ] Player sprite is visible (not clipped)
- [ ] Player Z-index is correct (above background)
- [ ] Player sprite animates (if walking)
- [ ] Player texture atlas indices are correct

**Debug Command:**
```bash
# If player not visible, check:
grep -r "spawn_player" src/systems/
# Verify Transform Z-coordinate
# Verify TextureAtlas index
```

#### 3.2 Enemy Rendering
- [ ] Mobs spawn and are visible
- [ ] Mechs spawn and are visible
- [ ] Enemy sprites are correct size
- [ ] Enemy animations work
- [ ] Multiple enemies render without Z-fighting

#### 3.3 Projectile Rendering
- [ ] Bullets spawn when shooting
- [ ] Bullets are visible
- [ ] Bullet sprites are correct size
- [ ] Bullets move smoothly

#### 3.4 UI Rendering
- [ ] Health bar renders
- [ ] Score counter renders
- [ ] Compass renders (if applicable)
- [ ] Crosshair renders
- [ ] UI text is readable
- [ ] UI updates dynamically

**Visual Bug Checklist:**
- [ ] No flickering sprites
- [ ] No sprite tearing
- [ ] No missing textures (purple/magenta placeholders)
- [ ] Correct aspect ratios
- [ ] No Z-fighting
- [ ] Smooth animations

---

### Phase 4: Input Testing
**Goal:** Verify all input handling works

#### 4.1 Keyboard Input
- [ ] **W key** - Player moves up
- [ ] **A key** - Player moves left
- [ ] **S key** - Player moves down
- [ ] **D key** - Player moves right
- [ ] **Diagonal movement** - Combine keys (e.g., W+D)
- [ ] **F key** - Switch ammo type forward
- [ ] **V key** - Switch ammo type backward
- [ ] **ESC key** - Pause or quit (if implemented)

**Test Procedure:**
```
1. Hold W for 2 seconds → Player should move up
2. Release W, hold S for 2 seconds → Player should move down
3. Hold W+D together → Player should move diagonally
4. Press F → Ammo type should change (check UI or console)
```

#### 4.2 Mouse Input
- [ ] **Mouse movement** - Player rotates to face cursor
- [ ] **Left click** - Shoot weapon
- [ ] **Hold left click** - Continuous shooting (if rate of fire allows)
- [ ] **Mouse wheel scroll** - Switch ammo type
- [ ] **Cursor visibility** - System cursor hidden, crosshair visible

**Test Procedure:**
```
1. Move mouse around screen → Player should rotate smoothly
2. Left click → Bullet should spawn and move toward cursor
3. Hold left click for 3 seconds → Multiple bullets should fire
4. Scroll wheel → Ammo type should cycle
```

---

### Phase 5: Gameplay Logic Testing
**Goal:** Verify core game mechanics work correctly

#### 5.1 Movement System
- [ ] Player moves at consistent speed
- [ ] Player can move in all 8 directions
- [ ] Movement is smooth (no stuttering)
- [ ] Movement respects frame rate (consistent at different FPS)
- [ ] Enemies move toward player
- [ ] Mechs move toward player
- [ ] Movement doesn't drift (stops when keys released)

**Performance Test:**
```
# Run for 60 seconds
# Monitor FPS: should stay at 60
# Monitor frame time: should be ~16.6ms
# Player should cover consistent distance each second
```

#### 5.2 Combat System
- [ ] **Shooting**
  - [ ] Bullets spawn at player position
  - [ ] Bullets move in correct direction (toward cursor)
  - [ ] Bullets have correct speed
  - [ ] Bullets despawn after traveling (time-to-live)
  - [ ] Rate of fire is correct (cooldown works)

- [ ] **Damage**
  - [ ] Bullet collision with enemy detected
  - [ ] Enemy health decreases on hit
  - [ ] Enemy despawns at 0 health
  - [ ] Player health decreases when hit by enemy
  - [ ] Health bar updates correctly

- [ ] **Death**
  - [ ] Player despawns at 0 health
  - [ ] Game over screen appears
  - [ ] Can restart by clicking

**Test Procedure:**
```
1. Shoot enemy 10 times
2. Verify enemy health bar decreases
3. Verify enemy despawns when health reaches 0
4. Verify score increases
5. Let enemy hit player until player dies
6. Verify game over screen appears
7. Click to restart
8. Verify new game starts
```

#### 5.3 Spawning System
- [ ] Enemies spawn at intervals
- [ ] Enemies spawn outside player view
- [ ] Spawn rate is reasonable (not overwhelming)
- [ ] Mechs spawn less frequently than mobs
- [ ] Powerups spawn at intervals
- [ ] Maximum enemy count is reasonable

**Test Procedure:**
```
# Play for 5 minutes
# Count enemies on screen every 30 seconds
# Expected: Gradual increase, plateaus at reasonable count
# No sudden mass spawns
```

#### 5.4 Powerup System
- [ ] Powerups spawn and are visible
- [ ] Player can collect powerups (collision)
- [ ] Speed powerup increases movement speed
- [ ] Health powerup restores health
- [ ] Rate of fire powerup increases fire rate
- [ ] Powerup effects stack (multiple pickups)
- [ ] Powerup effects are visible (faster movement, health bar increase)
- [ ] Temporary powerups expire (TTL)

**Test Procedure:**
```
1. Collect speed powerup
2. Observe player moves faster
3. Collect multiple speed powerups
4. Verify speed continues to increase
5. Collect health powerup
6. Verify health bar increases
```

#### 5.5 Ammo System
- [ ] Different ammo types available
- [ ] Can switch ammo types (F/V keys)
- [ ] Different ammo types have different effects:
  - [ ] Normal: Basic damage
  - [ ] Poison: Damage over time
  - [ ] RageQuit: High damage
  - [ ] Reverser: Special effect
- [ ] Ammo type indicator updates

---

### Phase 6: State Management Testing
**Goal:** Verify game states transition correctly

#### 6.1 State Transitions
- [ ] Game starts in InGame state
- [ ] OnEnter(InGame) systems run once:
  - [ ] Player spawns
  - [ ] Camera spawns
  - [ ] UI spawns
  - [ ] Assets load
- [ ] Systems respect run_if(in_state()) conditions
- [ ] State transitions are immediate (no frame delay)

#### 6.2 Restart Functionality
- [ ] Game over triggers correctly
- [ ] Click-to-restart works
- [ ] New game resets:
  - [ ] Player health to max
  - [ ] Score to 0
  - [ ] Enemy count to 0
  - [ ] Powerup effects cleared
- [ ] No entity leaks (old game entities despawned)

**Test Procedure:**
```
1. Play until death
2. Note score and enemy count
3. Click to restart
4. Verify score is 0
5. Verify no old entities remain
6. Play again to ensure no crashes
```

---

### Phase 7: Performance Testing
**Goal:** Verify game runs smoothly without performance issues

#### 7.1 Frame Rate Test
```bash
# Run in release mode
cargo run --release

# Monitor FPS (should be in console or use external tool)
# Target: Locked 60 FPS
```

**Test Cases:**
- [ ] Idle (no action): 60 FPS
- [ ] Moving: 60 FPS
- [ ] Shooting rapidly: 60 FPS
- [ ] 20+ enemies on screen: 60 FPS
- [ ] 50+ entities total: 60 FPS

**If FPS drops below 60:**
1. Profile with `cargo flamegraph`
2. Check for infinite loops
3. Check for excessive entity spawns
4. Verify fixed timestep is working

#### 7.2 Memory Test
```bash
# Run game for 10 minutes
# Monitor memory usage (htop, Task Manager, Activity Monitor)

# Expected behavior:
# - Memory usage should plateau after ~2 minutes
# - No continuous growth (memory leak)
# - Reasonable total memory (<500 MB for simple roguelike)
```

**Test Procedure:**
```
1. Start game
2. Note initial memory usage
3. Play actively for 10 minutes (move, shoot, kill enemies)
4. Note memory usage every 2 minutes
5. Memory should stabilize, not grow continuously
```

**If memory leaks detected:**
1. Check entity despawn logic
2. Check event cleanup
3. Check resource dropping
4. Use Rust memory profiler

#### 7.3 Frame Time Consistency
```bash
# Fixed timestep should ensure consistent game logic
# Measure time for player to cross screen width

# Test at different rendering FPS:
# - 60 FPS
# - 30 FPS (vsync off, frame limiter)
# - 144 FPS (high refresh rate monitor)

# Game logic (movement distance) should be identical
```

---

### Phase 8: Determinism Testing (Critical for Multiplayer/Replay)
**Goal:** Verify game state is deterministic given same inputs

#### 8.1 Manual Determinism Test
```bash
# Test 1: Record inputs
1. Start game
2. Perform specific actions:
   - Move W for 2 seconds
   - Turn right
   - Shoot 5 times
   - Move D for 2 seconds
3. Note final:
   - Player position
   - Enemy count
   - Score
   - Entity count

# Test 2: Replay exact same inputs
1. Restart game
2. Perform EXACT same actions
3. Verify final state is IDENTICAL
```

**Expected:**
- [ ] Same player position
- [ ] Same enemy positions
- [ ] Same score
- [ ] Same entity count
- [ ] Same RNG seed results (if using seeded RNG)

**If non-deterministic:**
- Check system ordering (must be deterministic)
- Check RNG seeding
- Check floating-point precision issues
- Verify no timing-based logic outside fixed timestep

---

### Phase 9: Edge Case Testing
**Goal:** Test boundary conditions and error cases

#### 9.1 Boundary Tests
- [ ] **Player at screen edge** - Can move off-screen? Clipped?
- [ ] **Shoot at screen edge** - Bullets spawn correctly?
- [ ] **0 enemies** - Game continues normally?
- [ ] **100+ enemies** - Performance acceptable?
- [ ] **0 health** - Death triggers?
- [ ] **Full health** - Health powerup works?
- [ ] **Maximum score** - Overflow handling?

#### 9.2 Stress Tests
- [ ] **Rapid firing** - Hold left click for 60 seconds
  - [ ] No crashes
  - [ ] Bullets despawn correctly
  - [ ] Frame rate stays stable
- [ ] **Mass enemy spawn** - Spawn 100 enemies
  - [ ] Rendering works
  - [ ] Collision detection works
  - [ ] No crashes
- [ ] **Prolonged play** - Play for 30 minutes
  - [ ] No memory leaks
  - [ ] No entity leaks
  - [ ] No performance degradation

#### 9.3 Asset Missing Tests
- [ ] Rename sprite file → Game should error gracefully
- [ ] Remove font file → UI should error gracefully
- [ ] Check console for clear error messages

---

### Phase 10: Regression Testing
**Goal:** Verify no features broke during migration

**Test Matrix:**

| Feature | Pre-Migration | Post-Migration | Status |
|---------|---------------|----------------|--------|
| Player movement | Working | ? | ⬜ |
| Player shooting | Working | ? | ⬜ |
| Enemy spawning | Working | ? | ⬜ |
| Enemy AI | Working | ? | ⬜ |
| Collision detection | Working | ? | ⬜ |
| Health system | Working | ? | ⬜ |
| Score system | Working | ? | ⬜ |
| Powerup system | Working | ? | ⬜ |
| UI updates | Working | ? | ⬜ |
| Ammo switching | Working | ? | ⬜ |
| Game over | Working | ? | ⬜ |
| Restart | Working | ? | ⬜ |

**Fill in Status:**
- ✅ - Working correctly
- ⚠️ - Working with issues
- ❌ - Not working

---

## Automated Testing (Future Work)

### Unit Test Examples

```rust
// tests/systems/movement_test.rs
#[test]
fn test_move_event_emitted() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_event::<MoveEvent>();
    app.add_systems(Update, move_player);

    // Spawn player
    let player = app.world.spawn(Player).id();

    // Simulate input
    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::KeyW);
    app.insert_resource(input);

    // Run system
    app.update();

    // Assert event was sent
    let mut reader = app.world.resource_mut::<Events<MoveEvent>>();
    let events: Vec<_> = reader.iter().collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].direction, Vec3::Y);
}
```

### Integration Test Examples

```rust
// tests/integration/gameplay_test.rs
#[test]
fn test_player_kills_enemy() {
    let mut app = setup_test_app();

    let player = spawn_test_player(&mut app);
    let enemy = spawn_test_enemy(&mut app, Vec3::new(50.0, 0.0, 0.0));

    // Shoot at enemy
    for _ in 0..10 {
        app.send_event(ShootEvent { gun: player_gun });
        app.update();
    }

    // Enemy should be dead
    assert!(!app.world.get_entity(enemy).is_ok());
}
```

---

## Test Report Template

After completing all tests, fill out this report:

### Migration Test Report

**Date:** [Date]
**Tester:** [Name]
**Branch:** `refactor/ecs-simplification`
**Commit:** [Hash]

#### Summary
- [ ] All tests passed
- [ ] Some tests passed (specify failures below)
- [ ] Testing incomplete

#### Phase Results

| Phase | Status | Notes |
|-------|--------|-------|
| 1. Compilation | ⬜ | |
| 2. Launch | ⬜ | |
| 3. Visual | ⬜ | |
| 4. Input | ⬜ | |
| 5. Gameplay | ⬜ | |
| 6. State Management | ⬜ | |
| 7. Performance | ⬜ | |
| 8. Determinism | ⬜ | |
| 9. Edge Cases | ⬜ | |
| 10. Regression | ⬜ | |

#### Issues Found

| Issue | Severity | Description | Status |
|-------|----------|-------------|--------|
| 1 | | | |
| 2 | | | |

#### Performance Metrics

- **Average FPS:** ___
- **Memory Usage:** ___
- **Entity Count (stable):** ___
- **Frame Time:** ___

#### Conclusion

- [ ] ✅ Migration successful - ready to proceed
- [ ] ⚠️ Migration partially successful - issues need fixing
- [ ] ❌ Migration failed - revert needed

---

## Next Steps After Testing

1. **If all tests pass:**
   - Commit test results
   - Proceed to Phase 2 of refactoring (System Consolidation)
   - See `PLAN.md` for next steps

2. **If tests fail:**
   - Document failures in GitHub issue
   - Analyze failures (migration bug vs. pre-existing bug?)
   - Fix critical issues
   - Re-test
   - Do NOT proceed to next phase until stable

3. **If performance regressions:**
   - Profile with flamegraph
   - Compare with Bevy 0.9 version (if available)
   - Investigate system ordering
   - Check for accidental debug builds

---

## Testing Checklist Summary

Quick reference for testing progress:

### Critical Tests (Must Pass)
- [ ] Compilation succeeds
- [ ] Game launches without crash
- [ ] Player visible and controllable
- [ ] Shooting works
- [ ] Enemies spawn and move
- [ ] Combat system works
- [ ] 60 FPS maintained

### Important Tests (Should Pass)
- [ ] All input works
- [ ] UI updates correctly
- [ ] Powerups work
- [ ] State management works
- [ ] No memory leaks
- [ ] Determinism verified

### Nice-to-Have Tests (If Time Permits)
- [ ] Edge cases handled
- [ ] Stress tests passed
- [ ] Asset error handling graceful

---

**Testing is critical - do not skip! 🧪**

A working game that passes all tests is infinitely better than a "migrated" game that crashes.
