//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#[allow(dead_code)]
#[derive(Debug)]
pub enum Ailment {
    Normal,
//    Poisoned,
//    Paralyzed,
//    Asleep,
    Blind,
    Unconscious,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Psyche {
    Normal,
//    Drunk,
//    Terror,
//    Depressed,
//    Hallucinating,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EnemyAilment {
    Normal,
//    Poisoned,
//    Paralyzed,
//    Asleep,
    Blind,
    Dead,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EnemyPsyche {
    Normal,
//    Terrified,
//    Enraged,
}