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
    pub given_exp: usize,
    pub max_hp: i16,
    pub max_ap: i16,
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
    pub fn new(foe_type: EnemyType, id: u8) -> Enemy {
        match foe_type {
            // TODO: implement a default construction of enemies so that they can be properly created
            // and set up.
            EnemyType::Common => {
                match id as u8 {
                    1..=50 => Enemy {
                        enemy_type: EnemyType::Common,
                        enemy_id: 11,
                        enemy_name: String::from("Scrubbot"),
                        level: rand::thread_rng().gen_range(1, 5),
                        given_exp: rand::thread_rng().gen_range(2, 5) * 10,
                        max_hp: 20,
                        max_ap: 10,
                        hp: 20,
                        ap: 10,
                        strength: 3,
                        alchemy: 3,
                        vitality: 3,
                        dexterity: 3,
                        agility: 4,
                        luck: 2,
                        status: EnemyAilment::Normal,
                        psyche: EnemyPsyche::Normal,
                        is_dead: false,
                    },
                    51..=150 => Enemy {
                        enemy_type: EnemyType::Common,
                        enemy_id: 31,
                        enemy_name: String::from("Roblin"),
                        level: rand::thread_rng().gen_range(3, 8),
                        given_exp: rand::thread_rng().gen_range(3, 6) * 10,
                        max_hp: 30,
                        max_ap: 20,
                        hp: 30,
                        ap: 20,
                        strength: 4,
                        alchemy: 4,
                        vitality: 4,
                        dexterity: 4,
                        agility: 4,
                        luck: 3,
                        status: EnemyAilment::Normal,
                        psyche: EnemyPsyche::Normal,
                        is_dead: false,
                    },
                    151..=200 => Enemy {
                        enemy_type: EnemyType::Common,
                        enemy_id: 59,
                        enemy_name: String::from("Death Roomba"),
                        level: rand::thread_rng().gen_range(4, 9),
                        given_exp: rand::thread_rng().gen_range(4, 7) * 10,
                        max_hp: 40,
                        max_ap: 30,
                        hp: 40,
                        ap: 30,
                        strength: 6,
                        alchemy: 6,
                        vitality: 6,
                        dexterity: 6,
                        agility: 6,
                        luck: 4,
                        status: EnemyAilment::Normal,
                        psyche: EnemyPsyche::Normal,
                        is_dead: false,
                    },
                    201..=250 => Enemy {
                        enemy_type: EnemyType::Common,
                        enemy_id: 89,
                        enemy_name: String::from("Geoff"),
                        level: rand::thread_rng().gen_range(5, 10),
                        given_exp: rand::thread_rng().gen_range(5, 8) * 10,
                        max_hp: 50,
                        max_ap: 40,
                        hp: 0,
                        ap: 0,
                        strength: 7,
                        alchemy: 7,
                        vitality: 7,
                        dexterity: 7,
                        agility: 7,
                        luck: 7,
                        status: EnemyAilment::Normal,
                        psyche: EnemyPsyche::Normal,
                        is_dead: false,
                    },
                    251..=255 => Enemy {
                        enemy_type: EnemyType::Common,
                        enemy_id: 107,
                        enemy_name: String::from("Mechoblin"),
                        level: rand::thread_rng().gen_range(6, 11),
                        given_exp: rand::thread_rng().gen_range(6, 9) * 10,
                        max_hp: 50,
                        max_ap: 0,
                        hp: 50,
                        ap: 40,
                        strength: 11,
                        alchemy: 3,
                        vitality: 10,
                        dexterity: 1,
                        agility: 20,
                        luck: 0,
                        status: EnemyAilment::Normal,
                        psyche: EnemyPsyche::Normal,
                        is_dead: false,
                    },
                    _ => { Enemy::default(foe_type) },
                }// end match
            },
            EnemyType::Boss => Enemy {
                enemy_type: EnemyType::Boss,
                enemy_id: 0,
                enemy_name: String::from("Muscel Boye"),
                level: 15,
                given_exp: rand::thread_rng().gen_range(6, 9) * 20,
                max_hp: 1000,
                max_ap: 0,
                hp: 200,
                ap: 10,
                strength: 10,
                alchemy: 20,
                vitality: 20,
                dexterity: 5,
                agility: 10,
                luck: 13,
                status: EnemyAilment::Normal,
                psyche: EnemyPsyche::Normal,
                is_dead: false,
            }
        }
    }// enemy creator

    pub fn default(foe_type: EnemyType) -> Enemy {
        match foe_type {
            EnemyType::Common => Enemy {
                enemy_type: EnemyType::Common,
                enemy_id: 0,
                enemy_name: String::from("unknown enemy"),
                level: 0,
                given_exp: 0,
                max_hp: 10,
                max_ap: 10,
                hp: 10,
                ap: 10,
                strength: 0,
                alchemy: 0,
                vitality: 0,
                dexterity: 0,
                agility: 0,
                luck: 0,
                status: EnemyAilment::Blind,
                psyche: EnemyPsyche::Normal,
                is_dead: false,
            },
            EnemyType::Boss => Enemy {
                enemy_type: EnemyType::Boss,
                enemy_id: 0,
                enemy_name: String::from("unknown boss enemy"),
                level: 0,
                given_exp: 0,
                max_hp: 10,
                max_ap: 10,
                hp: 10,
                ap: 10,
                strength: 0,
                alchemy: 0,
                vitality: 0,
                dexterity: 0,
                agility: 0,
                luck: 0,
                status: EnemyAilment::Blind,
                psyche: EnemyPsyche::Normal,
                is_dead: false,
            }
        }
    }// default enemy

    pub fn decr_hp(&mut self, damage: i16) {
//        println!("Enemy took {} damage", damage);
        self.hp -= damage;
        if self.hp <= 0 {
            self.status = EnemyAilment::Dead;
            self.is_dead = true;
        }
    }// decr_hp

    pub fn check_status(&self) -> bool { self.is_dead }

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
}// end impl block for Enemies
