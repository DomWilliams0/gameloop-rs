# gameloop-rs

![Build Status](https://github.com/DomWilliams0/gameloop-rs/workflows/Rust/badge.svg)
[![Documentation](https://docs.rs/gameloop-rs/badge.svg)](https://docs.rs/gameloop)
[![Version](https://img.shields.io/crates/v/gameloop)](https://crates.io/crates/gameloop)
[![License](https://img.shields.io/crates/l/gameloop)](https://github.com/DomWilliams0/gameloop-rs/blob/master/LICENSE)


An implementation of [deWiTTERS](https://dewitters.com/dewitters-gameloop/) game loop.

## Usage

```rust
// run at 20 ticks per second, with max frame skip of 5
let game_loop = GameLoop::new(20, 5).unwrap();

// begin core game loop
loop {
    // ... handle window events ...

    for action in game_loop.actions() {
        match action {
            FrameAction::Tick => /* simulate 1 game tick */
            FrameAction::Render { interpolation } => /* render the game state interpolated
                                                        between previous and next tick */
        }
    }
}
```
