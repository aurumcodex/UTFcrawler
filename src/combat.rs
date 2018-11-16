//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!

#![allow(dead_code)]

extern crate rand;
extern crate termion;

use crate::player::*;
use crate::enemy::*;
use crate::game_state::{Score, palettes};

use rand::Rng;

use termion::{clear, cursor, color, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std::io::{Write, Read, stdout, stdin};
use game_state::palettes::nes_palette;
//use std::io::Read;

pub fn combat(player: &mut Player, enemy: &mut Enemy) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}=== combat mode initiated ==={}{}\n\
                    player has encountered a {} \r\n",
                    clear::All, cursor::Goto(8, 7), style::Bold,
                    style::Reset, cursor::Goto(0, 8), enemy.enemy_name).unwrap();
    stdout.flush().unwrap();

    let mut in_bytes = stdin.bytes();
    while player.is_dead != true && enemy.is_dead != true {
        let input = in_bytes.next().unwrap().unwrap();
        match input {
            b'a' => {
                let attack_dmg: i16 = rand::thread_rng().gen_range(1, 5);
                enemy.decr_hp(attack_dmg);
                write!(stdout, "Enemy has taken {} points of  damage!\n\r", attack_dmg);
            },
            b's' => {
                let attack_dmg: i16 = rand::thread_rng().gen_range(5, 10);
                enemy.decr_hp(attack_dmg);
                write!(stdout, "Enemy has taken {} points of hefty damage!\n\r", attack_dmg);
            },
            b'd' => {
                let attack_dmg: i16 = rand::thread_rng().gen_range(10, 15);
                enemy.decr_hp(attack_dmg);
                write!(stdout, "Enemy has taken {} points of massive damage!\n\r", attack_dmg);
            },
            b'f' => {
                let attack_dmg: i16 = rand::thread_rng().gen_range(20, 25);
                enemy.decr_hp(attack_dmg);
                write!(stdout, "Enemy has taken {} points of ridiculous damage!\n\r", attack_dmg);
            },
            b'g' => {
                write!(stdout, "Player is defending this turn\n\r");
                let attack_dmg: i16 = rand::thread_rng().gen_range(1, 6);
                player.take_dmg(attack_dmg);
                write!(stdout, "Player has taken {} points of reduced damage.\n\r", attack_dmg);
            }
            _a => {
                write!(stdout, "please input a proper input.\n\r {}", cursor::Goto(0, 1));
            }
        }
    }//main while loop
    if player.is_dead == true {
        println!("you have died");
        return;
    }//if player becomes dead
    if enemy.is_dead == true {
        println!("enemy status is  now: {:?}", enemy.status);
        write!(stdout, "{}{}player has gained {} exp. {}\n\r", cursor::Goto(8, 7), color::Fg(nes_palette::NES_BRT_GREEN), enemy.given_exp, color::Fg(color::Reset));
        player.gain_exp(enemy.given_exp);
        player.check_level_up();
    }// on victory
}