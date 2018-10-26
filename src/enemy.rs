//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

extern crate rand;

use crate::status::{EnemyAilment, EnemyPsyche};
use rand::Rng;

#[derive(Debug)]
pub enum EnemyType {
    Common,
    Rare,
    Boss,
}

//pub enum EnemyLevel

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub enemy_id: i8,
    pub enemy_name: String,
    pub level: u8,
    pub given_exp: u32,
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
    pub status: EnemyAilment,
    pub psyche: EnemyPsyche,
    pub is_dead: bool,
}

impl Enemy {
    pub fn new(foe_type: EnemyType, id: u8) {
        match foe_type {
            // TODO: implement a default construction of enemies so that they can be properly created
            // and set up.
            EnemyType::Common => (),
            EnemyType::Rare => (),
            EnemyType::Boss => ()
        }
    }

    pub fn decr_hp(&mut self, damage: i16) {
        self.hp -= damage;
        if self.hp <= 0 {
            self.status = EnemyAilment::Dead;
            self.is_dead = true;
        }
    }// decr_hp

    pub fn print_stats(&self) {
        println!("Enemy ID: {} \n\
                  Enemy Name: {} \n\
                  Enemy Type: {:?} \n\
                  Enemy Level: {} \n\
                  Enemy HP: {} \n\
                  Enemy MP: {} \n\
                  Enemy Phys Attk: {} \n\
                  Enemy Phys Def: {} \n\
                  Enemy Mag Attk: {} \n\
                  Enemy Mag Def: {} \n\
                  Enemy Strength: {} \n\
                  Enemy Magic: {} \n\
                  Enemy Vitality: {} \n\
                  Enemy Dexterity: {} \n\
                  Enemy Agility: {} \n\
                  Enemy Luck: {} \n\
                  Enemy Status: {:?} \n\
                  Enemy Psyche: {:?} \n\
                  Enemy Is Dead? :: {} \n",
                  self.enemy_id,
                  self.enemy_name,
                  self.enemy_type,
                  self.level,
                  self.hp,
                  self.mp,
                  self.phys_attk,
                  self.phys_def,
                  self.mag_attk,
                  self.mag_def,
                  self.strength,
                  self.magic,
                  self.vitality,
                  self.dexterity,
                  self.agility,
                  self.luck,
                  self.status,
                  self.psyche,
                  self.is_dead);
    }// print_stats
}