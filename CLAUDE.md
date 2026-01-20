# Development Guide

This file provides architectural context and development guidelines for AI assistants and developers working in this repository.

## Project Overview

Talgonite is a Rust-based game client for [Darkages](https://www.darkages.com). The architecture separates concerns into three distinct layers: Bevy for game logic (ECS only), a standalone wgpu rendering library, and Slint for UI.

## Build Commands

```bash
cargo build                    # Debug build (opt-level=1, deps at opt-level=3)
cargo build --release          # Release build (LTO, single codegen unit)
cargo test                     # Run all tests including rendering snapshots
cargo insta review             # Review snapshot test changes
```

Requires mold linker on Linux. WGPU validation disabled via `.cargo/config.toml`.

## Architecture

### The Three Layers

**Bevy (ECS only, no rendering)**
- Game state machine (Loading, MainMenu, InGame)
- Entity management (players, creatures, items, effects)
- Game logic systems (pathfinding, movement, animation state)
- Event routing and input processing
- System scheduling via `GameSet` phases

**rendering crate (standalone wgpu, no Bevy dependency)**
- Pure graphics library that could work without Bevy
- Manages GPU resources, textures, shaders, batching
- Scene rendering: map tiles, walls, players, creatures, items, effects
- Isometric coordinate system conversion
- Instance batching for efficient GPU rendering

**Slint (owns the window and wgpu context)**
- Creates and owns the wgpu Device/Queue
- Provides GPU context to the game renderer (not vice versa)
- Renders all UI (login, inventory, chat, dialogs)
- Composites game texture with UI overlay
- Handles window events, forwards input to Bevy via channels

### Frame Flow

```
Slint rendering_notifier(BeforeRendering)
    │
    ├─► Bevy app.update()
    │      ├─ ECS systems process game logic
    │      └─ RenderSync phase: sync_*_to_renderer systems
    │         extract entity state into GPU instance buffers
    │
    ├─► draw_frame() (Bevy Last schedule)
    │      ├─ Render map, items, creatures, players, effects
    │      └─ Send rendered texture to Slint via FrameChannels
    │
    └─► Slint composites game texture + UI, displays to screen
```

### Workspace Crates

| Crate | Purpose |
|-------|---------|
| **talgonite** | Main app: ECS systems, Slint bridge, session management |
| **rendering** | Standalone wgpu renderer (no Bevy dependency) |
| **packets** | Protocol definitions (100+ packet types in server/ and client/) |
| **network** | Async networking, packet encryption/decryption |
| **formats** | Game file parsers (EFA, EPF, MPF, HPF for Darkages archives) |
| **game-ui** | Slint UI definitions (.slint files) and Rust bindings |
| **game-input** | Unified keyboard/gamepad input abstraction |
| **installer** | Downloads and installs .arx game archives |
| **game-types** | Shared types (hotbar, settings) |

### Key Source Files

| File | Purpose |
|------|---------|
| `src/slint_plugin.rs` | Main bridge (~46KB): rendering_notifier callback, frame exchange, UI state sync |
| `src/render_plugin/game.rs` | Bevy plugin: draw_frame(), GPU init, resize handling |
| `src/ecs/systems/rendering.rs` | Syncs ECS state to GPU batches each frame |
| `src/ecs/plugin.rs` | System ordering via GameSet phases |
| `src/session/runtime.rs` | Network session, login flow, main game loop |
| `rendering/src/scene/mod.rs` | Scene pipeline, depth texture, bind groups |
| `rendering/src/shaders/shader.wgsl` | Main vertex/fragment shaders |

### ECS System Ordering

Systems run in `GameSet` phases (defined in `src/ecs/plugin.rs`):
```
EventProcessing → Spawning → Despawning → Movement → Physics → Animation → Camera → RenderSync → Effects
```

The `RenderSync` phase is critical - it extracts ECS entity state into GPU instance buffers before `draw_frame()` runs in Bevy's Last schedule.

### Input Flow

- **Keyboard**: Slint captures → queued via channels → Bevy ButtonInput
- **Gamepad**: Directly through Bevy's gilrs integration (bypasses Slint)
- See `src/input/README.md` for the unified input system documentation

### Settings

Stored in platform data directory (`~/.local/share/Talgonite/` on Linux) as TOML files.

## Dependencies Note

Slint is patched from a specific git commit (`[patch.crates-io]` in Cargo.toml) for unstable wgpu 27 integration.
