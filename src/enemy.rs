//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

//#![allow(dead_code)]

extern crate rand;

use rand::Rng;

use status::{EnemyAilment, EnemyPsyche};

/// There are two common enemy types: `Common` and `Boss`.
///
/// Within the `Common` enemies, there are a handful of different types.
/// These are explained in the `new()` function.
///
/// So far, there is only one `Boss` type implemented.
#[derive(Debug)]
pub enum EnemyType {
    Common, Boss,
}

/// Struct to hold Enemy Data.
///
/// `enemy_type` = Common or Boss,
/// `enemy_id` = used to determine which enemy is encountered,
/// `enemy_name` = name of the enemy being fought,
/// `level` = enemy level,
/// `given_exp` = number of experience points given to Player when defeated,
/// `max_hp` = max hp of enemy,
/// `hp` = current hp of enemy,
/// `strength` = proficiency in physical attacks,
/// `alchemy` = proficiency in alchemy,
/// `vitality` = how resilient the enemy is,
/// `dexterity` = deftness with hands,
/// `agility` = mobility,
/// `luck` = fairly self explanitory
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub enemy_id: i8,
    pub enemy_name: String,
    pub level: u8,
    pub given_exp: usize,
    pub max_hp: i16,
    pub hp: i16,
    // pub max_ap: i16,
    // pub ap: i16,
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
    // TODO: potentially refactor the new() function to be more abstracted out; add documentation
    /// Creates a new enemy using a passthrough argument of what EnemyType is used, along with a
    /// range of enemy IDs. Winds up creating either a Scrubbot, Roblin, Death Roomba, Geoff, or
    /// Mechoblin, as common enemies. A Lord Gundam serves as a boss.
    pub fn new(foe_type: EnemyType, id: u8) -> Enemy {
        match foe_type {
            EnemyType::Common => {
                match id as u8 {
                    1..=50 => Enemy {
                        enemy_type: EnemyType::Common,
                        enemy_id: 11,
                        enemy_name: String::from("Scrubbot"),
                        level: rand::thread_rng().gen_range(1, 5),
                        given_exp: rand::thread_rng().gen_range(2, 5) * 10,
                        max_hp: 20,
                        hp: 20,
                        // max_ap: 10,
                        // ap: 10,
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
                        hp: 30,
                        // max_ap: 20,
                        // ap: 20,
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
                        hp: 40,
                        // max_ap: 30,
                        // ap: 30,
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
                        hp: 50,
                        // max_ap: 40,
                        // ap: 40,
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
                        max_hp: 0,
                        hp: 0,
                        // max_ap: 0,
                        // ap: 0,
                        strength: 8,
                        alchemy: 8,
                        vitality: 10,
                        dexterity: 12,
                        agility: 4,
                        luck: 5,
                        status: EnemyAilment::Normal,
                        psyche: EnemyPsyche::Normal,
                        is_dead: false,
                    },
                    _ => { Enemy::default(foe_type) },
                }// end match for common enemy ids
            },
            EnemyType::Boss => Enemy {
                enemy_type: EnemyType::Boss,
                enemy_id: 101,
                enemy_name: String::from("Lord Gundam"),
                level: 20,
                given_exp: 0,
                max_hp: 137,
                hp: 137,
                // max_ap: 137,
                // ap: 137,
                strength: 13,
                alchemy: 13,
                vitality: 13,
                dexterity: 13,
                agility: 13,
                luck: 13,
                status: EnemyAilment::Normal,
                psyche: EnemyPsyche::Normal,
                is_dead: false,
            }
        }// end enemy type match
    }// enemy creator

    /// Creates a default enemy. Can either be a Common or a Boss. Should only crop up when an
    /// error has occured of some kind that wasn't already dealt with.
    pub fn default(foe_type: EnemyType) -> Enemy {
        match foe_type {
            EnemyType::Common => Enemy {
                enemy_type: EnemyType::Common,
                enemy_id: 0,
                enemy_name: String::from("unknown enemy"),
                level: 0,
                given_exp: 0,
                max_hp: 10,
                hp: 10,
                // max_ap: 10,
                // ap: 10,
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
                hp: 10,
                // max_ap: 10,
                // ap: 10,
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

    /// Function to let the player deal damage to the enemies that are generated.
    /// Used in `combat.rs` to deal with the back-and-forth of damage dealing.
    pub fn decr_hp(&mut self, damage: i16) {
        self.hp -= damage;
        if self.hp <= 0 {
            self.status = EnemyAilment::Dead;
            self.is_dead = true;
        }
        print!("Enemy has taken {} points of damage!\n\r", damage);
    }// decr_hp

    /// A function to check if the enemy is dead and has run out of available HP.
    pub fn check_status(&self) -> bool { self.is_dead }

    /// Prints the stats of the enemy. Used for debugging purposes.
    pub fn print_stats(&self) {
        println!("Enemy ID: {}", self.enemy_id);
        println!("Enemy Type: {:?}", self.enemy_type);
        println!("Enemy Name: {}", self.enemy_name);
        println!("Enemy Level: {}", self.level);
        println!("Enemy Given EXP: {}", self.given_exp);
        println!("Enemy Max HP: {}", self.max_hp);
        println!("Enemy HP: {}", self.hp);
        // println!("Enemy Max AP: {}", self.max_ap);
        // println!("Enemy AP: {}", self.ap);
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
}// end impl block for Enemy
