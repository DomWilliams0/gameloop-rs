use std::cell::Cell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Instant;

use log::debug;

/// Represents the core loop for the duration of the game.
///
/// # Example
///
/// ```
/// # use ::gameloop::*;;
/// // run at 20 ticks per second
/// let game_loop = GameLoop::new(20, 5).unwrap();
///
/// // begin core game loop
/// loop {
///     // handle window events
///
///     for action in game_loop.actions() {
///         match action {
///             FrameAction::Tick => /* simulate 1 game tick */
/// # {},
///             FrameAction::Render { interpolation } => /* render the game state interpolated
///                                                         between previous and next tick */
/// # {},
///         }
///     }
///
/// # break; // not infinite pls
/// }
/// ```
pub struct GameLoop {
    /// The game start time
    start_time: Instant,

    /// Milliseconds between each game tick
    skip_ticks: usize,

    /// Maximum number of consecutive ticks before a render is mandatory.
    max_frameskip: usize,

    /// Time in ms for the next scheduled game tick
    next_game_tick: Cell<usize>,
}

/// Errors possible when initializing `GameLoop`.
#[derive(Debug)]
pub enum GameLoopError {
    BadTps,
    BadFrameSkip,
}

impl GameLoop {
    /// Create a new game loop.
    /// # Arguments
    /// * `tps`: game ticks per second
    /// * `max_frameskip`: maximum number of consecutive ticks before a render is mandatory. As
    /// [deWiTTERS](https://dewitters.com/dewitters-gameloop/) explains:
    /// > When running on slow hardware, the framerate can drop until the game update loop will
    /// > reach MAX_FRAMESKIP. In practice this means that when our render FPS drops below 5
    /// > (= FRAMES_PER_SECOND / MAX_FRAMESKIP), the actual game will slow down.
    ///
    /// # Example
    /// ```
    /// # use ::gameloop::*;;
    /// // 20 ticks per second, 5 max frame skip
    /// let game_loop = GameLoop::new(20, 5);
    /// assert!(game_loop.is_ok());
    ///
    /// // tps and max_frameskip must be >= 1
    /// assert!(GameLoop::new(0, 1).is_err());
    /// assert!(GameLoop::new(1, 0).is_err());
    /// ```
    ///
    pub fn new(tps: usize, max_frameskip: usize) -> Result<Self, GameLoopError> {
        if tps < 1 {
            return Err(GameLoopError::BadTps);
        }

        if max_frameskip < 1 {
            return Err(GameLoopError::BadFrameSkip);
        }

        let start_time = Instant::now();
        let skip_ticks = 1000 / tps;

        debug!(
            "initialized with {} ticks/second ({}ms/tick), with a max frame skip of {}",
            tps, skip_ticks, max_frameskip
        );

        Ok(Self {
            start_time,
            max_frameskip,
            skip_ticks,
            next_game_tick: Cell::new(0),
        })
    }

    /// The heart of the game loop, this returns an iterator of `FrameAction`s. These indicate
    /// when your game should tick and render to maintain the fixed tick rate while rendering
    /// as fast as possible.
    ///
    /// Call this once per game loop iteration.
    ///
    /// # Example
    /// ```
    /// # use ::gameloop::*;;
    /// let game_loop = /* initialize game loop */
    /// # GameLoop::new(10,5).unwrap();
    /// loop {
    ///     // handle window events
    ///
    ///     for action in game_loop.actions() {
    ///         match action {
    ///             FrameAction::Tick => /* simulate 1 game tick */
    /// # {},
    ///             FrameAction::Render { interpolation } => /* render the game state interpolated
    ///                                                         between previous and next tick */
    /// # {},
    ///         }
    ///     }
    /// # break;
    /// }
    ///```
    pub fn actions(&self) -> impl Iterator<Item = FrameAction> + '_ {
        FrameActions {
            game_loop: self,
            loops: 0,
            rendered: false,
        }
    }

    /// Milliseconds since the game started.
    fn tick_count(&self) -> usize {
        self.start_time.elapsed().as_millis() as usize
    }

    fn increment_next_game_tick(&self) {
        let current = self.next_game_tick.get();
        self.next_game_tick.set(current + self.skip_ticks);
    }
}

/// Iterator of `FrameAction`s, returned by `GameLoop::actions`.
pub struct FrameActions<'a> {
    game_loop: &'a GameLoop,

    loops: usize,
    rendered: bool,
}

/// Represents a tick or render instruction, to be interpreted by your game.
/// # Example
/// ```ignore
/// # use gameloop::*;
/// let mut my_game = MyGame::default();
/// let game_loop = GameLoop::new(20, 5).unwrap();
///
/// while !my_game.should_quit() {
///     my_game.handle_window_events();
///
///     for action in game_loop.actions() {
///         match action {
///             FrameAction::Tick => my_game.tick(),
///             FrameAction::Render { interpolation } => {
///                 let prev_state = my_game.previous_state();
///                 let curr_state = my_game.current_state();
///
///                 let interpolated_state = prev_state.interpolate(curr_state, interpolation);
///                 my_game.render(interpolated_state);
///             }
///         }
///     }
/// }
///
/// ```
#[derive(Debug)]
pub enum FrameAction {
    /// The game should simulate one tick.
    Tick,

    /// The game should render the game state interpolated by the given amount between the previous
    /// tick and the current.
    Render { interpolation: f64 },
}

impl<'a> Iterator for FrameActions<'a> {
    type Item = FrameAction;

    fn next(&mut self) -> Option<Self::Item> {
        let next_tick = self.game_loop.next_game_tick.get();

        if self.game_loop.tick_count() > next_tick && self.loops < self.game_loop.max_frameskip {
            self.game_loop.increment_next_game_tick();
            self.loops += 1;
            return Some(FrameAction::Tick);
        }

        if !self.rendered {
            self.rendered = true;

            let render_time = self.game_loop.tick_count();
            let skip_ticks = self.game_loop.skip_ticks;
            let interpolation: f64 =
                ((render_time + skip_ticks - next_tick) as f64) / (skip_ticks as f64);

            return Some(FrameAction::Render { interpolation });
        }

        None
    }
}

impl Display for GameLoopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameLoopError::BadTps => write!(f, "Ticks per second must be >= 1"),
            GameLoopError::BadFrameSkip => write!(f, "Max frame skip must be >= 1"),
        }
    }
}

impl Error for GameLoopError {}
