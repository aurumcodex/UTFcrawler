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

pub enum Psyche {
    Normal,
    Drunk,
    Terror,
    Depressed,
}