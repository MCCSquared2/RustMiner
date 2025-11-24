# RustMiner

Starter Bevy game framework with a lightweight player controller and UI overlays.

## Features

- Configurable Bevy app with a pre-set window, camera, and clear color.
- Basic player entity with WASD/arrow movement while in the `InGame` state.
- Pause/resume state management via `Esc`, with matching HUD status text.
- Simple UI overlay showing controls and the current game state.

## Running

```bash
cargo run
```

Use `Esc` to toggle pause and `W/A/S/D` or the arrow keys to move the placeholder player square.
