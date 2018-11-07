//! 
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#![allow(dead_code)]

extern crate rand;

// use std::io;
use crate::status::{Ailment, Psyche};
use rand::Rng;

#[derive(Debug, Eq, PartialEq)]
pub enum Archetype {
    Mercenary, Gunner, Alchemist, Blackguard, Generalist, None,
}

pub struct Player {
    pub player_name: String,
    pub level: u8,
    pub exp: u32,
    pub to_next_level: i64,
    pub prev_next_level: i64,
    pub archetype: Archetype,
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
    pub status: Ailment,
    pub psyche: Psyche,
    pub is_dead: bool,
}

impl Player {
    pub fn new(name: String, arch: Archetype) -> Player {
        let gen_normal_max_hp = rand::thread_rng().gen_range(40, 56);
        let gen_normal_max_ap = rand::thread_rng().gen_range(40, 56);
        let gen_heavy_max_hp = rand::thread_rng().gen_range(50, 66);
        let gen_heavy_max_ap = rand::thread_rng().gen_range(50, 66);
        let gen_weak_max_hp = rand::thread_rng().gen_range(35, 51);
        let gen_weak_max_ap = rand::thread_rng().gen_range(35, 51);

        match arch {
            Archetype::Alchemist => Player {
                player_name: name,
                level: 1,
                exp: 0,
                to_next_level: 20,
                prev_next_level: 20,
                archetype: arch,
                max_hp: gen_weak_max_hp,
                max_ap: gen_heavy_max_ap,
                hp: gen_weak_max_hp,
                ap: gen_heavy_max_ap,
                strength: rand::thread_rng().gen_range(3, 11)-3,
                alchemy: rand::thread_rng().gen_range(3, 11)+3,
                vitality: rand::thread_rng().gen_range(3, 11)-2,
                dexterity: rand::thread_rng().gen_range(3, 11),
                agility: rand::thread_rng().gen_range(3, 11)-1,
                luck: rand::thread_rng().gen_range(3, 11),
                status: Ailment::Normal,
                psyche: Psyche::Normal,
                is_dead: false,
            },
            Archetype::Blackguard => Player {
                player_name: name,
                level: 1,
                exp: 0,
                to_next_level: 20,
                prev_next_level: 20,
                archetype: arch,
                max_hp: gen_normal_max_hp,
                max_ap: gen_normal_max_ap,
                hp: gen_normal_max_hp,
                ap: gen_normal_max_ap,
                strength: rand::thread_rng().gen_range(3, 11),
                alchemy: rand::thread_rng().gen_range(3, 11)-2,
                vitality: rand::thread_rng().gen_range(3, 11)-3,
                dexterity: rand::thread_rng().gen_range(3, 11),
                agility: rand::thread_rng().gen_range(3, 11)+3,
                luck: rand::thread_rng().gen_range(3, 11)-1,
                status: Ailment::Normal,
                psyche: Psyche::Normal,
                is_dead: false,
            },
            Archetype::Generalist => Player {
                player_name: name,
                level: 1,
                exp: 0,
                to_next_level: 20,
                prev_next_level: 20,
                archetype: arch,
                max_hp: gen_normal_max_hp,
                max_ap: gen_normal_max_ap,
                hp: gen_normal_max_hp,
                ap: gen_normal_max_ap,
                strength: rand::thread_rng().gen_range(3, 11)+1,
                alchemy: rand::thread_rng().gen_range(3, 11)+1,
                vitality: rand::thread_rng().gen_range(3, 11)+1,
                dexterity: rand::thread_rng().gen_range(3, 11)+1,
                agility: rand::thread_rng().gen_range(3, 11)+1,
                luck: rand::thread_rng().gen_range(3, 11)+1,
                status: Ailment::Normal,
                psyche: Psyche::Normal,
                is_dead: false,
            },
            Archetype::Gunner => Player {
                player_name: name,
                level: 1,
                exp: 0,
                to_next_level: 20,
                prev_next_level: 20,
                archetype: arch,
                max_hp: gen_normal_max_hp,
                max_ap: gen_normal_max_ap,
                hp: gen_normal_max_hp,
                ap: gen_normal_max_ap,
                strength: rand::thread_rng().gen_range(3, 11)-2,
                alchemy: rand::thread_rng().gen_range(3, 11)-3,
                vitality: rand::thread_rng().gen_range(3, 11)-1,
                dexterity: rand::thread_rng().gen_range(3, 11)+3,
                agility: rand::thread_rng().gen_range(3, 11),
                luck: rand::thread_rng().gen_range(3, 11),
                status: Ailment::Normal,
                psyche: Psyche::Normal,
                is_dead: false,
            },
            Archetype::Mercenary => Player {
                player_name: name,
                level: 1,
                exp: 0,
                to_next_level: 20,
                prev_next_level: 20,
                archetype: arch,
                max_hp: gen_heavy_max_hp,
                max_ap: gen_weak_max_ap,
                hp: gen_heavy_max_hp,
                ap: gen_weak_max_ap,
                strength: rand::thread_rng().gen_range(3, 11)+3,
                alchemy: rand::thread_rng().gen_range(3, 11)-3,
                vitality: rand::thread_rng().gen_range(3, 11),
                dexterity: rand::thread_rng().gen_range(3, 11),
                agility: rand::thread_rng().gen_range(3, 11)-2,
                luck: rand::thread_rng().gen_range(3, 11)-1,
                status: Ailment::Normal,
                psyche: Psyche::Normal,
                is_dead: false,
            },
            _ => { Player::default(name) }
        }// match statement
    }
    
    pub fn default(name: String) -> Player {
        Player {
            player_name: name,
            level: 0,
            exp: 0,
            to_next_level: 20,
            prev_next_level: 20,
            archetype: Archetype::None,
            max_hp: 1,
            max_ap: 0,
            hp: 1,
            ap: 0,
            strength: 0,
            alchemy: 0,
            vitality: 0,
            dexterity: 0,
            agility: 0,
            luck: 0,
            status: Ailment::Normal,
            psyche: Psyche::Normal,
            is_dead: false,
        }
    }

    pub fn gain_exp(&mut self, add_exp: u32) {
//        println!("gained {} EXP", add_exp);
        self.exp += add_exp;
        if self.level < 11_u8 { self.to_next_level -= add_exp as i64; }
        self.check_level_up();
    }

    pub fn level_up(&mut self) {
        // FIXME: exp gain formula
        let temp_next_lvl_exp: f64 =
            ((self.prev_next_level as f64 * ((self.prev_next_level as f64)
                .powi(self.level as i32 + 1_i32).log10())) * 1.7654_f64 + (((self.prev_next_level as f64)
                .powi(self.level as i32 + 1_i32)).ln())) / ((self.level as f64 + 1_f64).powf(1.4_f64));

        if self.level < 10_u8 {
            println!("player has gained a level!");
            self.level += 1;

            self.to_next_level += temp_next_lvl_exp as i64;
            self.prev_next_level = temp_next_lvl_exp as i64;

            match self.archetype {
                Archetype::Alchemist => {
                    self.max_hp += rand::thread_rng().gen_range(3, 7);
                    self.max_ap += rand::thread_rng().gen_range(6, 10);
                    self.hp = self.max_hp;
                    self.ap = self.max_ap;
                    self.strength += rand::thread_rng().gen_range(1, 4);
                    self.alchemy += rand::thread_rng().gen_range(2, 6);
                    self.vitality += rand::thread_rng().gen_range(1, 4);
                    self.dexterity += rand::thread_rng().gen_range(2, 5);
                    self.agility += rand::thread_rng().gen_range(1, 3);
                    self.luck += rand::thread_rng().gen_range(2, 4);
                    self.status = Ailment::Normal;
                },
                Archetype::Blackguard => {
                    self.max_hp += rand::thread_rng().gen_range(4, 8);
                    self.max_ap += rand::thread_rng().gen_range(3, 7);
                    self.hp = self.max_hp;
                    self.ap = self.max_ap;
                    self.strength += rand::thread_rng().gen_range(2, 5);
                    self.alchemy += rand::thread_rng().gen_range(2, 4);
                    self.vitality += rand::thread_rng().gen_range(0, 4);
                    self.dexterity += rand::thread_rng().gen_range(2, 5);
                    self.agility += rand::thread_rng().gen_range(3, 6);
                    self.luck += rand::thread_rng().gen_range(2, 4);
                    self.status = Ailment::Normal;
                },
                Archetype::Generalist => {
                    self.max_hp += rand::thread_rng().gen_range(4, 8);
                    self.max_ap += rand::thread_rng().gen_range(4, 7);
                    self.hp = self.max_hp;
                    self.ap = self.max_ap;
                    self.strength += rand::thread_rng().gen_range(2, 5);
                    self.alchemy += rand::thread_rng().gen_range(2, 5);
                    self.vitality += rand::thread_rng().gen_range(2, 5);
                    self.dexterity += rand::thread_rng().gen_range(2, 5);
                    self.agility += rand::thread_rng().gen_range(2, 5);
                    self.luck += rand::thread_rng().gen_range(2, 5);
                    self.status = Ailment::Normal;
                },
                Archetype::Gunner => {
                    self.max_hp += rand::thread_rng().gen_range(5, 9);
                    self.max_ap += rand::thread_rng().gen_range(4, 8);
                    self.hp = self.max_hp;
                    self.ap = self.max_ap;
                    self.strength += rand::thread_rng().gen_range(3, 6);
                    self.alchemy += rand::thread_rng().gen_range(2, 6);
                    self.vitality += rand::thread_rng().gen_range(3, 6);
                    self.dexterity += rand::thread_rng().gen_range(4, 8);
                    self.agility += rand::thread_rng().gen_range(3, 7);
                    self.luck += rand::thread_rng().gen_range(3, 5);
                    self.status = Ailment::Normal;
                },
                Archetype::Mercenary => {
                    self.max_hp += rand::thread_rng().gen_range(6, 10);
                    self.max_ap += rand::thread_rng().gen_range(3, 7);
                    self.hp = self.max_hp;
                    self.ap = self.max_ap;
                    self.strength += rand::thread_rng().gen_range(3, 6);
                    self.alchemy += rand::thread_rng().gen_range(1, 4);
                    self.vitality += rand::thread_rng().gen_range(3, 6);
                    self.dexterity += rand::thread_rng().gen_range(3, 5);
                    self.agility += rand::thread_rng().gen_range(3, 5);
                    self.luck += rand::thread_rng().gen_range(1, 4);
                    self.status = Ailment::Normal;
                },
                _ => {},
            }
            self.print_stats();
        }
    }

    pub fn check_level_up(&mut self) {
        if self.level <= 10_u8 {
            if self.to_next_level <= 0 { self.level_up() }
        }
    }

    pub fn take_dmg(&mut self, damage: i16) {
        self.hp -= damage;
        if self.hp <= 0 {
            self.status = Ailment::Unconscious;
            self.is_dead = true;
        }
    }

    pub fn check_status(&self) -> bool { self.is_dead }

    pub fn print_stats(&self) {
        println!("Player Name: {}", self.player_name);
        println!("Player Level: {}", self.level);
        println!("Player EXP: {}", self.exp);
        println!("Player To Next Level: {}", self.to_next_level);
        println!("Player Archetype: {:?}", self.archetype);
        println!("Player Max HP: {}", self.max_hp);
        println!("Player HP: {}", self.hp);
        println!("Player Max AP: {}", self.max_ap);
        println!("Player AP: {}", self.ap);
        println!("Player Strength: {}", self.strength);
        println!("Player Magic: {}", self.alchemy);
        println!("Player Vitality: {}", self.vitality);
        println!("Player Dexterity: {}", self.dexterity);
        println!("Player Agility: {}", self.agility);
        println!("Player Luck: {}", self.luck);
        println!("Player Status: {:?}", self.status);
        println!("Player Psyche: {:?}", self.psyche);
        println!("Is Player Dead? :: {}", self.is_dead);
    }

}
