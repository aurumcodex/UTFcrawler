//! 
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!
mod status.rs;

use status::{Ailment, Psyche};

#[derive(Debug)]
enum Archetype {
    Mercenary,
    Gunner,
    Thaumaturgist,
    Blackguard,
    Freelancer,
}

pub struct Player {
    player_name: String,
    level: u8,
    exp: u32,
    archetype: Archetype,
    hp: u16,
    mp: u16,
    phys_attk: u16,
    phys_def: u16,
    mag_attk: u16,
    mag_def: u16,
    strength: u8,
    magic: u8,
    vitality: u8,
    dexterity: u8,
    agility: u8,
    luck: u8,
    status: Ailment,
    psyche: Psyche,
}

impl Player {
    fn incr_exp(&mut self, add_exp: u32) {
        self.exp += add_exp
    }

    fn print_stats(&self) {
        println!("Player Name: {}", self.player_name);
        println!("Player Level: {}", self.level);
        println!("Player EXP: {}", self.exp);
        println!("Player Archetype: {}", self.archetype);
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
        println!("Player Status: {}", self.status);
        println!("Player Psyche: {}", self.psyche);
    }
}