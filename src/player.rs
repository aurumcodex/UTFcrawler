//! 
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!
// mod status;

extern crate rand;

// use std::io;
use crate::status::{Ailment, Psyche};
use rand::Rng;

#[derive(Debug)]
pub enum Archetype {
    Mercenary,
    Gunner,
    Occultist,
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
    pub fn new(name: String) -> Player {
        Player {
            player_name: name,
            level: 1,
            exp: 0,
            archetype: Archetype::Gunner,
            hp: rand::thread_rng().gen_range(20, 50),
            mp: rand::thread_rng().gen_range(20, 50),
            phys_attk: rand::thread_rng().gen_range(20, 50),
            phys_def: rand::thread_rng().gen_range(20, 50),
            mag_attk: rand::thread_rng().gen_range(20, 50),
            mag_def: rand::thread_rng().gen_range(20, 50),
            strength: rand::thread_rng().gen_range(20, 50),
            magic: rand::thread_rng().gen_range(20, 50),
            vitality: rand::thread_rng().gen_range(20, 50),
            dexterity: rand::thread_rng().gen_range(20, 50),
            agility: rand::thread_rng().gen_range(20, 50),
            luck: rand::thread_rng().gen_range(20, 50),
            status: Ailment::Normal,
            psyche: Psyche::Normal,
        }
    }

    pub fn incr_exp(&mut self, add_exp: u32) {
        println!("gained {} EXP", add_exp);
        self.exp += add_exp
    }

    pub fn level_up(&mut self) {

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