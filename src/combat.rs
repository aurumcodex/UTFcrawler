//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!

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
//use game_state::{Score};

use std::io::{Write, Read, stdout, stdin};
use std::process;

pub fn combat(player: &mut Player, enemy: &mut Enemy) {
    let stdin = stdin();
//    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdout = stdout();
    let mut stdout = stdout.lock();

    write!(stdout, r"
        {}{}{}=== combat mode initiated ==={}{}
        player has encountered a {}{}",
         clear::All, cursor::Goto(8, 7), style::Bold,
         style::Reset, cursor::Goto(0, 8), enemy.enemy_name, cursor::Goto(0, 10)).unwrap();
    stdout.flush().unwrap();

    let mut in_bytes = stdin.bytes();
    while player.is_dead != true && enemy.is_dead != true {
        let input = in_bytes.next().unwrap().unwrap();
        match input {
            b'a' => {
                let attack_dmg: i16 = rand::thread_rng().gen_range((player.strength - 3) as i16, player.strength as i16);
                enemy.decr_hp(attack_dmg);
//                write!(stdout, "Enemy has taken {} points of damage!\n\r", attack_dmg);
            },
            b's' => {
                if player.level < 3 {
                    write!(stdout, "player is unable to use this skill; not high enough level\n\r");
                    stdout.flush().unwrap();
                    continue;
                } else {
                    let damage_calc = |stat, range| {
                        println!("{} damage", range + stat);
                        range + stat
                    };
//                    let attack_dmg: i16 = rand::thread_rng().gen_range(5, 10);
                    enemy.decr_hp(damage_calc(player.strength as i16, rand::thread_rng().gen_range(5, 10)));
//                    write!(stdout, "Enemy has taken {} points of hefty damage!\n\r", attack_dmg);
                }
            },
            b'd' => {
                if player.level < 6 {
                    write!(stdout, "player is unable to use this skill; not high enough level\n\r");
                    stdout.flush().unwrap();
                    continue;
                } else {
                    let attack_dmg: i16 = rand::thread_rng().gen_range(10, 15);
                    enemy.decr_hp(attack_dmg);
                    write!(stdout, "Enemy has taken {} points of massive damage!\n\r", attack_dmg);
                }
            },
            b'f' => {
                if player.level < 9 {
                    write!(stdout, "player is unable to use this skill; not high enough level\n\r");
                    stdout.flush().unwrap();
                    continue;
                } else {
                    let attack_dmg: i16 = rand::thread_rng().gen_range(20, 25);
                    enemy.decr_hp(attack_dmg);
                    write!(stdout, "Enemy has taken {} points of ridiculous damage!\n\r", attack_dmg);
                }
            },
            b'g' => {
                write!(stdout, "Player is defending this turn\n\r");
                let attack_dmg: i16 = rand::thread_rng().gen_range(1, 6);
                player.take_dmg(attack_dmg);
                write!(stdout, "Player has taken {} points of reduced damage.\n\r", attack_dmg);
            }
            _a => {
                write!(stdout, "{} please input a proper input.\n\r {}{}",
                       color::Fg(nes_palette::NES_BRT_RED), cursor::Goto(2, 1), color::Fg(color::Reset));
            }
        }
    }//main while loop
    if player.is_dead == true {
        println!("{}you have died{}{}", color::Fg(nes_palette::NES_RED), color::Fg(color::Reset), clear::All);
        stdout.flush().unwrap();
        return;
//        process::exit(45);
    }//if player becomes dead
    if enemy.is_dead == true {
        println!("enemy status is now: {:?}", enemy.status);
        write!(stdout, "{}{}player has gained {} exp. {}{}", cursor::Goto(8, 7), color::Fg(nes_palette::NES_BRT_GREEN),
               enemy.given_exp, color::Fg(color::Reset), clear::All);
        player.gain_exp(enemy.given_exp);
        player.check_level_up();
    }// on victory
}
