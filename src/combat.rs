//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!

#![allow(dead_code)]

extern crate rand;
extern crate termion;

use crate::player::*;
use crate::enemy::*;
use crate::game_state::Score;

use rand::Rng;

use termion::{clear, cursor, color, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use std::io::{Write, Read, stdout, stdin};
//use std::io::Read;

pub fn combat(player: &mut Player, enemy: &mut Enemy) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}=== combat mode initiated ==={}{}\n\
                    player has encountered a {}",
                    clear::All, cursor::Goto(8, 7), style::Bold,
                    style::Reset, cursor::Goto(0, 8), enemy.enemy_name).unwrap();
    stdout.flush().unwrap();

    let mut in_bytes = stdin.bytes();
    while player.is_dead != true && enemy.is_dead != true {
        let input = in_bytes.next().unwrap().unwrap();
        match input {
            b'a' => {
                let attack_dmg: i16 = rand::thread_rng().gen_range(1, 4);
                enemy.decr_hp(attack_dmg);
                write!(stdout, "Enemy has taken {} damage!\n\r", attack_dmg);
            }
            a => {
//                write!()
            }
        }
    }//main while loop
    if player.is_dead == true {
        println!("you have died");
    }//if player becomes dead
    if enemy.is_dead == true {
        println!("enemy status is  now: {:?}", enemy.status);
        player.gain_exp(enemy.given_exp);
        player.check_level_up();
    }// on victory
}
