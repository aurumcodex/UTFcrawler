//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#![allow(dead_code)]
#[warn(unused_imports)]

extern crate rand;

use crate::status::{EnemyAilment, EnemyPsyche};
use rand::Rng;

#[derive(Debug)]
pub enum EnemyType {
    Common, Boss,
}

//pub enum EnemyLevel

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub enemy_id: i8,
    pub enemy_name: String,
    pub level: u8,
    pub given_exp: u32,
    pub max_hp: u16,
    pub max_ap: u16,
    pub hp: i16,
    pub ap: i16,
    pub strength: u8,
    pub alchemy: u8,
    pub vitality: u8,
    pub dexterity: u8,
    pub agility: u8,
    pub luck: u8,
    pub status: EnemyAilment,
    pub psyche: EnemyPsyche,
    pub is_dead: bool,
}

impl Enemy {
    pub fn new(foe_type: EnemyType, _id: u8) {
        match foe_type {
            // TODO: implement a default construction of enemies so that they can be properly created
            // and set up.
            EnemyType::Common => (),
            EnemyType::Boss => ()
        }
    }

    pub fn decr_hp(&mut self, damage: i16) {
        println!("Enemy took {} damage", damage);
        self.hp -= damage;
        if self.hp <= 0 {
            self.status = EnemyAilment::Dead;
            self.is_dead = true;
        }
    }// decr_hp
    
    pub fn print_stats(&self) {
        println!("Enemy ID: {}", self.enemy_id);
        println!("Enemy Type: {:?}", self.enemy_type);
        println!("Enemy Name: {}", self.enemy_name);
        println!("Enemy Level: {}", self.level);
        println!("Enemy Given EXP: {}", self.given_exp);
        println!("Enemy Max HP: {}", self.max_hp);
        println!("Enemy Max AP: {}", self.max_ap);
        println!("Enemy HP: {}", self.hp);
        println!("Enemy AP: {}", self.ap);
        println!("Enemy Strength: {}", self.strength);
        println!("Enemy Alchemy: {}", self.alchemy);
        println!("Enemy Vitality: {}", self.vitality);
        println!("Enemy Dexterity: {}", self.dexterity);
        println!("Enemy Agility: {}", self.agility);
        println!("Enemy Luck: {}", self.luck);
        println!("Enemy Status: {:?}", self.status);
        println!("Enemy Psyche: {:?}", self.psyche);
        println!("Enemy is dead? :: {}", self.is_dead);
    }// print_stats
}
