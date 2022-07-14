//! Glass is a simple game engine for 2D grid based games.
//! Planned features:
//! 1. Graphics engine
//! 2. Sound engine
//! 3. Input manager
//! 4. GUI Editor
//! 5. Serialization
//! 6. Console and logger
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::indexing_slicing)]

macro_rules! unwrap_or_err {
    ( $e:expr, $err:expr ) => {
        match $e {
            Some(x) => x,
            None => return $err,
        }
    }
}

pub mod graphics;
pub mod input;
pub mod sound;
pub mod core;
pub mod data_types;