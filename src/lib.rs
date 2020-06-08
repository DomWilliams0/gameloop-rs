//!
//! An implementation of [deWiTTERS](https://dewitters.com/dewitters-gameloop/) game loop.
//!
//! # Usage
//!
//! ```
//! # use ::gameloop::*;;
//! // run at 20 ticks per second, with max frame skip of 5
//! let game_loop = GameLoop::new(20, 5).unwrap();
//!
//! // begin core game loop
//! loop {
//!     // ... handle window events ...
//!
//!     for action in game_loop.actions() {
//!         match action {
//!             FrameAction::Tick => /* simulate 1 game tick */
//! # {},
//!             FrameAction::Render { interpolation } => /* render the game state interpolated
//!                                                         between previous and next tick */
//! # {},
//!         }
//!     }
//! # break; // not infinite pls
//! }
//! ```

mod gameloop;

pub use self::gameloop::{FrameAction, FrameActions, GameLoop, GameLoopError};
