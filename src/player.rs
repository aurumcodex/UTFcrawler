//! 
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!
// mod status;

use crate::status::{Ailment, Psyche};

#[derive(Debug)]
pub enum Archetype {
    Mercenary,
    Gunner,
    Thaumaturgist,
    Blackguard,
    Freelancer,
}

pub struct Player {
    pub player_name: String,
    pub level: u8,
    pub exp: u32,
    pub archetype: Archetype,
    pub hp: u16,
    pub mp: u16,
    pub phys_attk: u16,
    pub phys_def: u16,
    pub mag_attk: u16,
    pub mag_def: u16,
    pub strength: u8,
    pub magic: u8,
    pub vitality: u8,
    pub dexterity: u8,
    pub agility: u8,
    pub luck: u8,
    pub status: Ailment,
    pub psyche: Psyche,
}

impl Player {
    pub fn new() -> Player {
        
    }

    pub fn incr_exp(&mut self, add_exp: u32) {
        self.exp += add_exp
    }

    pub fn print_stats(&self) {
        println!("Player Name: {}", self.player_name);
        println!("Player Level: {}", self.level);
        println!("Player EXP: {}", self.exp);
        println!("Player Archetype: {:?}", self.archetype);
        println!("Player HP: {}", self.hp);
        println!("Player MP: {}", self.mp);
        println!("Player Phys Attk: {}", self.phys_attk);
        println!("Player Phys Def: {}", self.phys_def);
        println!("Player Magic Attk: {}", self.mag_attk);
        println!("Player Magic Def: {}", self.mag_def);
        println!("Player Strength: {}", self.strength);
        println!("Player Magic: {}", self.magic);
        println!("Player Vitality: {}", self.vitality);
        println!("Player Dexterity: {}", self.dexterity);
        println!("Player Agility: {}", self.agility);
        println!("Player Luck: {}", self.luck);
        println!("Player Status: {:?}", self.status);
        println!("Player Psyche: {:?}", self.psyche);
    }
}