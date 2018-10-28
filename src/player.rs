//! 
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!
// mod status;
// test comment for checking prompt

extern crate rand;

// use std::io;
use crate::status::{Ailment, Psyche};
use rand::Rng;

#[derive(Debug)]
pub enum Archetype {
    Mercenary, Gunner, Alchemist, Blackguard, Generalist,
}

pub struct Player {
    pub player_name: String,
    pub level: u8,
    pub exp: u32,
    pub to_next_level: u32,
    pub archetype: Archetype,
    pub hp: i16,
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
    pub is_dead: bool,
}

impl Player {
    pub fn new(name: String, arch: Archetype) -> Player {
        Player {
            player_name: name,
            level: 1,
            exp: 0,
            to_next_level: 20,
            archetype: arch,
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
            is_dead: false,
        }
    }

    pub fn gain_exp(&mut self, add_exp: u32) {
        println!("gained {} EXP", add_exp);
        self.exp += add_exp
    }

    pub fn level_up(&mut self) {
        // TODO: implement leveling up functions, i.e. stat increases, resetting to next level value
    }

    pub fn check_level_up(&self) {
        // TODO: implement
    }

    pub fn take_dmg(&mut self, damage: i16) {
        self.hp -= damage;
        if self.hp <= 0 {self.status = Ailment::Unconscious}
    }

    pub fn check_status(&self) -> bool { self.is_dead }

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
        println!("Is Player Dead? :: {}", self.is_dead);
    }
}