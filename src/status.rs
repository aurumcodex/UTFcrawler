//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#[derive(Debug)]
pub enum Ailment {
    Normal,
    Poisoned,
    Paralyzed,
    Asleep,
    Blind,
    Unconscious,
}

#[derive(Debug)]
pub enum Psyche {
    Normal,
    Drunk,
    Terror,
    Depressed,
}