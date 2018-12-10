//! # UTFcrawler
//!
//! ### Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
//! ## About
//! This is a program to render a top-down view of a dungeon crawler game that can be played in a
//! termial. A terminal size of 103 cols x 30 lines is highly recommended for the maps to display
//! correctly. A terminal font with a lot of Unicode character support is also recommended, as is a
//! terminal that displays 256 colors, or has RGB color support.
//!
//! This is the `combat.rs` file, which runs this application's combat systems.

#![allow(dead_code)]

extern crate rand;
extern crate termion;

use rand::Rng;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, color, style};
use termion::event::Key;

use enemy::*;
use player::*;
use game_state::palettes::nes_palette;
// use game_state::{Score};

use std::{thread, time};
use std::io::{Write, Read, stdout, stdin};
// use std::process;

/// Struct to determine if the game is in its end state.
pub struct endGame { pub end: bool, }

pub const bossSprite: &str =  "
					\r
                       ／ﾌﾌ              ム｀ヽ     \r
                      / ノ)    Λ＿Λ       ）  ヽ    \r
                     / ｜    ( • ω •）ノ⌒（ゝ ,ノ　\r
                    /    ﾉ⌒7⌒     ヽーく　_＼_／　\r
                    丶＿ ノ    ｡  ノ､  ｡ |/\r
                        `ヽ `ー-'_人`ーﾉ\r
                          丶 ￣ _人'彡ﾉ\r";

pub const win: &str =  "\r
oooooo   oooo   .oooooo.   ooooo     ooo    oooooo   oooooo     oooo ooooo ooooo      ooo    .o. \r
 `888.   .8'   d8P'  `Y8b  `888'     `8'     `888.    `888.     .8'  `888' `888b.     `8'    888 \r
  `888. .8'   888      888  888       8       `888.   .8888.   .8'    888   8 `88b.    8     888 \r
   `888.8'    888      888  888       8        `888  .8'`888. .8'     888   8   `88b.  8     Y8P \r
    `888'     888      888  888       8         `888.8'  `888.8'      888   8     `88b.8     `8' \r
     888      `88b    d88'  `88.    .8'          `888'    `888'       888   8       `888     .o. \r
    o888o      `Y8bood8P'     `YbodP'             `8'      `8'       o888o o8o        `8     Y8P \r";


/// Combat function. Implements a turn-based combat system for the game to use. 
/// Takes a mutable Player and a mutable Enemy.
pub fn combat(player: &mut Player, enemy: &mut Enemy, boss: usize) -> bool {
    let mut bossKill = false;
    let stdin = stdin();
    let stdout = stdout();
    let mut stdout = stdout.lock();

    write!(stdout, r"
        {}{}{}=== combat mode initiated ==={}{}
        player has encountered a {}{}",
        clear::All, cursor::Goto(8, 7), style::Bold,
        style::Reset, cursor::Goto(0, 8), enemy.enemy_name, cursor::Goto(0, 10)).unwrap();
    stdout.flush().unwrap();

    // prints boss character image when you fight him
    if boss == 8 { println!("{}",bossSprite); }

    let mut in_bytes = stdin.bytes();
    while player.is_dead != true && enemy.is_dead != true {
        let input = in_bytes.next().unwrap().unwrap();
            match input {
            b'a' => {
                let plyr_atk_dmg: i16 = rand::thread_rng().gen_range((player.strength - 3) as i16,
                                                                      player.strength as i16);
                let enemy_atk_dmg: i16 = rand::thread_rng().gen_range((enemy.strength - 3) as i16,
                                                                       enemy.strength as i16);
                enemy.decr_hp(plyr_atk_dmg);
                player.take_dmg(enemy_atk_dmg);
                if(boss == 8){
				println!("{}\r", clear::All);
				println!("HAha! That tickles!\r");
				println!("{}",bossSprite);
				}
                write!(stdout, "You slap the enemy for {} points of damage!\n\r", plyr_atk_dmg);
                println!("\r");
                thread::sleep_ms(200);
                write!(stdout, "The enemy hits you for {} points of damage!\n\r", enemy_atk_dmg);
                println!("\r");
                write!(stdout, "Current Health: {}\n\r", player.hp);
            },
            b's' => {
                if player.level < 3 {
                    write!(stdout, "You are unable to use this skill; not high enough level\n\r");
                    stdout.flush().unwrap();
                    continue;
                } else {
					if(boss == 8){
					println!("{}\r", clear::All);
					println!("ANOTHER!\r");
					println!("{}",bossSprite);
					}
                    let damage_range = rand::thread_rng().gen_range(5, 16);
                    let player_damage_calc = |stat, range, defense| {
						print!("You did ");
                        println!("{} damage!\r", range + stat - defense);
                        range + stat - defense
                    };
                    let enemy_damage_calc = |stat, range, defense| {
						print!("You just got thumped for ");
                        println!("{}  damage\r", range + stat);
                        range + stat - defense
                    };
                    match player.archetype {
                        Archetype::Alchemist => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a fiery formula on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc(player.alchemy as i16, damage_range,
                                                             enemy_defense));
                            thread::sleep_ms(200);
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Blackguard => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a crossbow bolt on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.dexterity - 2) as i16, damage_range,
                                                              enemy_defense));
                            thread::sleep_ms(200);                                  
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Generalist => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a slightly stronger general attack on the {}!\n\r",
                                   enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc(player.strength as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Gunner => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a shotgun on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc(player.dexterity as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Mercenary => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a metal pipe on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.strength + 2) as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        _ => {},
                    }
                }
            },
            b'd' => {
                if player.level < 6 {
                    write!(stdout, "You are unable to use this skill; not high enough level\n\r");
                    stdout.flush().unwrap();
                    continue;
                } else {
					if(boss == 8){
					println!("{}\r", clear::All);
					println!("A minor wound.\r");
					println!("{}",bossSprite);
					}
                    let damage_range = rand::thread_rng().gen_range(5, 16);
                    let player_damage_calc = |stat, range, defense| {
						print!("You did ");
                        println!("{} damage!\r", range + stat - defense);
                        range + stat - defense
                    };
                    let enemy_damage_calc = |stat, range, defense| {
						print!("You just got thumped for ");
                        println!("{}  damage\r", range + stat);
                        range + stat - defense
                    };
                    match player.archetype {
                        Archetype::Alchemist => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used an icy formula on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.alchemy + 5) as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Blackguard => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a hidden shoe dagger on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.dexterity + 2) as i16, damage_range,
                                                             enemy_defense));
                            thread::sleep_ms(200);                                 
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Generalist => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a hefty generic attack on the {}!\n\r",
                                   enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.strength + 7) as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Gunner => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a gatling gun on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.dexterity + 6) as i16, damage_range,
                                                             enemy_defense));
                           thread::sleep_ms(200);                                  
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Mercenary => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a fierce punch on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.strength + 4) as i16, damage_range,
                                                             enemy_defense));
                            thread::sleep_ms(200);                                 
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        _ => {},
                    }
                }
            },
            b'f' => {
                if player.level < 9 {
                    write!(stdout, "You are  unable to use this skill; not high enough level\n\r");
                    stdout.flush().unwrap();
                    continue;
                }else {
					if(boss == 8){
					println!("{}\r", clear::All);
					println!("HNNRH!\r");
					println!("{}",bossSprite);
					}
                    let damage_range = rand::thread_rng().gen_range(5, 16);
                    let player_damage_calc = |stat, range, defense| {
						print!("You did ");
                        println!("{} damage!\r", range + stat - defense);
                        range + stat - defense
                    };
                    let enemy_damage_calc = |stat, range, defense| {
						print!("You just got thumped for ");
                        println!("{}  damage\r", range + stat);
                        range + stat - defense
                    };
                    match player.archetype {
                        Archetype::Alchemist => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used an electrifying formula on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.alchemy + 11) as i16, damage_range,
                                                             enemy_defense));
                           thread::sleep_ms(200);                                  
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Blackguard => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You threw a shard of glass at the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.dexterity + 5) as i16, damage_range,
                                                             enemy_defense));
                            thread::sleep_ms(200);                                 
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Generalist => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a powerful generic attack on the {}!\n\r",
                                   enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.strength + 11) as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Gunner => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You used a bazooka on the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.dexterity + 9) as i16, damage_range,
                                                             enemy_defense));
                            thread::sleep_ms(200);                                 
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        Archetype::Mercenary => {
                            let player_defense = player.vitality as i16;
                            let enemy_defense = enemy.vitality as i16;
                            write!(stdout, "You slammed a car door into the {}!\n\r", enemy.enemy_name);
                            enemy.decr_hp(player_damage_calc((player.strength + 9) as i16, damage_range,
                                                             enemy_defense));
                             thread::sleep_ms(200);                                
                            player.take_dmg(enemy_damage_calc(enemy.strength as i16, damage_range - 5,
                                                              player_defense));
                        },
                        _ => {},
                    }
                }
            },
            b'g' => {
				if(boss == 8){
					println!("{}\r", clear::All);    //taunt from boss
					println!("Haha, foolish fool!\r");
					println!("{}",bossSprite);
					}
                write!(stdout, "You defending this turn\n\r");
                let attack_dmg: i16 = rand::thread_rng().gen_range(1, 6);
                player.take_dmg(attack_dmg);
                write!(stdout, "You have taken {} points of reduced damage.\n\r", attack_dmg);
            }
            _a => {
                write!(stdout, "{} please input a proper input.\n\r {}",
                       color::Fg(nes_palette::NES_BRT_RED), color::Fg(color::Reset));
            }
        }
    }//main while loop
    if player.is_dead == true {
        println!("{}{}\n\n\nyou have died{}\r\n", clear::All, color::Fg(nes_palette::NES_RED), 
                 color::Fg(color::Reset));
        stdout.flush().unwrap();
        return bossKill;
    }//if player becomes dead
    if enemy.is_dead == true {
		if(boss == 8){
				print!("{}", color::Fg(nes_palette::NES_YELLOW));
				println!("{}\r", clear::All);	
				println!("{}",win);
				bossKill = true;
		}else{
        println!("enemy status is now: {:?}", enemy.status);
        write!(stdout, "{}{}You gained {} exp. {}{}", cursor::Goto(8, 7), 
               color::Fg(nes_palette::NES_BRT_GREEN),
        enemy.given_exp, color::Fg(color::Reset), clear::All);
        player.gain_exp(enemy.given_exp);
        player.check_level_up();
	}
       // player.print_stats();
    }// on victory
    bossKill
}// combat function

