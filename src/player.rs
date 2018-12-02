//! 
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#![allow(dead_code)]

extern crate rand;

use rand::Rng;
use termion::{color, style};

use status::{Ailment, Psyche};
use game_state::palettes::nes_palette::*;

/// The Archetypes (classes, if you will) of what the Player can be.
///
/// Alchemists are weak in strength, but make up for it with good alchemy skills.
/// Blackguards are decent in strength, but utilize their dexterity to harm enemies.
/// Generalists are average all-rounders. They're the default choice.
/// Gunners lack in the strength department, but their dexterity is the best of them all.
/// Mercenaries are the brutes of the Archetypes. Strong in strength, but lacking in alchemic
/// skills.
#[derive(Debug, Eq, PartialEq)]
pub enum Archetype {
    Mercenary, Gunner, Alchemist, Blackguard, Generalist, None,
}

/// Struct to hold Player stats.
/// 
/// level = current power level,
/// exp = experience points,
/// archetype = the "class",
/// strength = physical attacks,
/// alchemy = proficiency in alchemy,
/// vitality = how resilient you are,
/// dexterity = deftness with hands,
/// agility = mobility,
/// luck = fairly self explanitory
pub struct Player {
    pub player_name: String,
    pub level: u8,
    pub exp: usize,
    pub to_next_level: u8,
    pub prev_next_level: u8,
    pub archetype: Archetype,
    pub max_hp: i16,
    pub hp: i16,
//    pub max_ap: i16,
//    pub ap: i16,
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
    // TODO: potentially refactor the new() function to be a bit more abstracted out; add
    // docmentation
    /// Creates a player using the Archetype as an argument, which sets the Player's stats
    /// accordingly. 
    pub fn new(arch: Archetype) -> Player {
        let gen_normal_max_hp = rand::thread_rng().gen_range(40, 56);
//        let gen_normal_max_ap = rand::thread_rng().gen_range(40, 56);
        let gen_heavy_max_hp = rand::thread_rng().gen_range(50, 66);
//        let gen_heavy_max_ap = rand::thread_rng().gen_range(50, 66);
        let gen_weak_max_hp = rand::thread_rng().gen_range(35, 51);
//        let gen_weak_max_ap = rand::thread_rng().gen_range(35, 51);

        match arch {
            Archetype::Alchemist => Player {
                player_name: String::from("Player"),
                level: 1,
                exp: 0,
                to_next_level: 1,
                prev_next_level: 1,
                archetype: arch,
                max_hp: gen_weak_max_hp,
                hp: gen_weak_max_hp,
//                max_ap: gen_heavy_max_ap,
//                ap: gen_heavy_max_ap,
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
                player_name: String::from("Player"),
                level: 1,
                exp: 0,
                to_next_level: 1,
                prev_next_level: 1,
                archetype: arch,
                max_hp: gen_normal_max_hp,
                hp: gen_normal_max_hp,
//                max_ap: gen_normal_max_ap,
//                ap: gen_normal_max_ap,
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
                player_name: String::from("Player"),
                level: 1,
                exp: 0,
                to_next_level: 1,
                prev_next_level: 1,
                archetype: arch,
                max_hp: gen_normal_max_hp,
                hp: gen_normal_max_hp,
//                max_ap: gen_normal_max_ap,
//                ap: gen_normal_max_ap,
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
                player_name: String::from("Player"),
                level: 1,
                exp: 0,
                to_next_level: 1,
                prev_next_level: 1,
                archetype: arch,
                max_hp: gen_normal_max_hp,
                hp: gen_normal_max_hp,
//                max_ap: gen_normal_max_ap,
//                ap: gen_normal_max_ap,
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
                player_name: String::from("Player"),
                level: 1,
                exp: 0,
                to_next_level: 1,
                prev_next_level: 1,
                archetype: arch,
                max_hp: gen_heavy_max_hp,
                hp: gen_heavy_max_hp,
//                max_ap: gen_weak_max_ap,
//                ap: gen_weak_max_ap,
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
            _ => { Player::default() }
        }// match statement
    }
    
    /// Creates a default player. Should only crop up when an error has occured of some kind that
    /// wasn't already dealt with.
    pub fn default() -> Player {
        Player {
            player_name: String::from("unknown player entity"),
            level: 0,
            exp: 0,
            to_next_level: 200,
            prev_next_level: 1,
            archetype: Archetype::None,
            max_hp: 1,
            hp: 1,
//            max_ap: 0,
//            ap: 0,
            strength: 0,
            alchemy: 0,
            vitality: 0,
            dexterity: 0,
            agility: 0,
            luck: 0,
            status: Ailment::Blind,
            psyche: Psyche::Normal,
            is_dead: false,
        }// player struct to return
    }//default player creation (used for pleyer gen errors)

    /// Adds experience points (EXP) to the player, and checks if the player is able to level up.
    pub fn gain_exp(&mut self, add_exp: usize) {
        self.exp += add_exp;
        self.to_next_level -= 1;
        self.check_level_up();
    }

    // TODO: RE: new() -- potential refactor to make more abstract.
    /// Function to level up.
    /// Increases the stats of the player depending on what Archetype they chose at the start of
    /// the game.
    pub fn level_up(&mut self) {
        // FIXME: exp gain formula
        if self.level < 13_u8 {
            println!("\rplayer has gained a level!\r");
            self.prev_next_level = (self.prev_next_level + 2) / 2 << 1;
            self.to_next_level = self.prev_next_level;

            println!("player needs {} more enemies to level up\r", self.to_next_level);

            if self.level == 3 || self.level == 6 || self.level == 9 {
                println!("player has gained a new skill!\r");
            }

            self.level += 1;

            match self.archetype {
                Archetype::Alchemist => {
                    self.max_hp += rand::thread_rng().gen_range(3, 7);
//                    self.max_ap += rand::thread_rng().gen_range(6, 10);
                    self.hp = self.max_hp % 17;
//                    self.ap = self.max_ap;
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
//                    self.max_ap += rand::thread_rng().gen_range(3, 7);
                    self.hp = self.max_hp % 17;
//                    self.ap = self.max_ap;
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
//                    self.max_ap += rand::thread_rng().gen_range(4, 7);
                    self.hp = self.max_hp % 17;
//                    self.ap = self.max_ap;
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
//                    self.max_ap += rand::thread_rng().gen_range(4, 8);
                    self.hp = self.max_hp % 17;
//                    self.ap = self.max_ap;
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
//                    self.max_ap += rand::thread_rng().gen_range(3, 7);
                    self.hp = self.max_hp % 17;
//                    self.ap = self.max_ap;
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
//            self.print_stats();
        }
    }//level_up

    /// Checks to see if conditions have been met to have a level up occur.
    pub fn check_level_up(&mut self) {
        if self.level < 17_u8 {
            if self.to_next_level == 0 { self.level_up() }
        }
    }//check_level_up

    /// Makes the player take damage.
    /// Used in `combat.rs` to deal with the back-and-forth of damage dealing, 
    pub fn take_dmg(&mut self, damage: i16) {
        self.hp -= damage;
        if self.hp <= 0 {
            self.status = Ailment::Unconscious;
            self.is_dead = true;
        }
        print!("{}Player has taken {}{}{}{} points of damage!{}\n\r", color::Fg(NES_ORANGE), style::Bold, 
                                   damage, style::Reset, color::Fg(NES_ORANGE), color::Fg(color::Reset));
    }//take_dmg

    /// A function to check if the player has run out of available HP and is dead.
    pub fn check_status(&self) -> bool { self.is_dead }

    /// Prints the current stats of the current player.
    pub fn print_stats(&self) {
        println!("Player Name: {}\r", self.player_name);
        println!("Player Level: {}\r", self.level);
        println!("Player EXP: {}\r", self.exp);
        println!("Player To Next Level: {}\r", self.to_next_level);
        println!("Player Archetype: {:?}\r", self.archetype);
        println!("Player Max HP: {}\r", self.max_hp);
        println!("Player HP: {}\r", self.hp);
//        println!("Player Max AP: {}\r", self.max_ap);
//        println!("Player AP: {}\r", self.ap);
        println!("Player Strength: {}\r", self.strength);
        println!("Player Magic: {}\r", self.alchemy);
        println!("Player Vitality: {}\r", self.vitality);
        println!("Player Dexterity: {}\r", self.dexterity);
        println!("Player Agility: {}\r", self.agility);
        println!("Player Luck: {}\r", self.luck);
        println!("Player Status: {:?}\r", self.status);
        println!("Player Psyche: {:?}\r", self.psyche);
        println!("Is Player Dead? :: {}\r", self.is_dead);
    }//print_stats
}
