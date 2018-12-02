//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

// TODO: potentially scrap this file altogether and move contents into player.rs and enemy.rs

#![allow(dead_code)]

#[derive(Debug)]
pub enum Ailment {
    Normal,
//    Poisoned,
//    Paralyzed,
//    Asleep,
    Blind,
    Unconscious,
}

#[derive(Debug)]
pub enum Psyche {
    Normal,
//    Drunk,
//    Terror,
//    Depressed,
//    Hallucinating,
}

#[derive(Debug)]
pub enum EnemyAilment {
    Normal,
//    Poisoned,
//    Paralyzed,
//    Asleep,
    Blind,
    Dead,
}

#[derive(Debug)]
pub enum EnemyPsyche {
    Normal,
//    Terrified,
//    Enraged,
}
