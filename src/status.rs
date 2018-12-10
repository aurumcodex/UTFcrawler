//! # UTFcrawler
//!
//! ### Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
//! ## About
//! This is a program to render a top-down view of a dungeon crawler game that can be played in a
//! termial. A terminal size of 103 cols x 30 lines is highly recommended for the maps to display
//! correctly. A terminal font with a lot of Unicode character support is also recommended, as is a
//! terminal that displays 256 colors, or has RGB color support.
//!
//! This is the `status.rs` file, which contains the enums for status ailments.


#![allow(dead_code)]

#[derive(Debug)]
pub enum Ailment {
    Normal,
    Blind,
    Unconscious,
}

#[derive(Debug)]
pub enum Psyche {
    Normal,
}

#[derive(Debug)]
pub enum EnemyAilment {
    Normal,
    Blind,
    Dead,
}

#[derive(Debug)]
pub enum EnemyPsyche {
    Normal,
}
