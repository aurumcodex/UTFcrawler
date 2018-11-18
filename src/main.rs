//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
//!  Nicholas checking in!

#![allow(dead_code)]

extern crate rand;
extern crate termion;
//extern crate ansi_term;

mod player;
mod enemy;
mod status;
mod combat;
mod dungeon;
mod inventory;
mod game_state;

use rand::Rng;
use termion::{color, cursor, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use self::player::*;
use self::enemy::*;
use self::status::*;
// use self::combat::*;
use self::dungeon::*;
use self::inventory::*;

use self::game_state::TITLE;
use self::game_state::palettes::*;
use std::env;
//use std::io;
use std::io::{Read, Write, stdout, stdin, stderr};
use std::process;

fn main() {
    let stdin = stdin();
//    let mut stdin = stdin.lock();
//    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdout = stdout();
    let mut stdout = stdout.lock();

    let mut player = Player::new(Archetype::Generalist);

//    let rand_stat = Player

    let norm = color::Rgb(104, 68, 252);
    
    writeln!(stdout, "{} {} {}", color::Fg(nes_palette::NES_BRT_GREEN), TITLE, color::Fg(color::Reset));

    let mut args = env::args().skip(1);
    loop {
        let arg = if let Some(n) = args.next() {
            n
        } else { break;};

        match arg.as_str() {
            "a" | "alchemist" => {
                let arch = Archetype::Alchemist;
                player = Player::new(arch);
            },
            "b" | "blackguard" => {
                let arch = Archetype::Blackguard;
                player = Player::new(arch);
            },
            "n" | "generalist" => {
                let arch = Archetype::Generalist;
                player = Player::new(arch);
            },
            "g" | "gunner" => {
                let arch = Archetype::Gunner;
                player = Player::new(arch);
            },
            "m" | "mercenary" => {
                let arch = Archetype::Mercenary;
                player = Player::new(arch);
            },
            _ => {
//                stderr().write(b"{}unknown argument.\n", color::Fg(nes_palette::NES_BRT_RED)).unwrap();
                write!(stderr(), "{}unknown argument of {}\n{}", color::Fg(nes_palette::NES_BRT_RED), arg,
                       color::Fg(color::Reset));
                stderr().flush().unwrap();
                process::exit(1);
            }
        }
    }// loop to read environment arguments

    println!("{}{}greetings, {}{}{}\r", style::Bold, color::Fg(norm), player.player_name,
                        color::Fg(color::Reset), style::Reset);

//    test_player.print_stats();
    player.gain_exp(rand::thread_rng().gen_range(5, 10));
    println!("player EXP is now {}\r", &player.exp);


    println!();

//    let mut test_enemy = Enemy::new(EnemyType::Common, rand::thread_rng().gen_range(3, 15) * player.level);
//
//    combat::combat(&mut player, &mut test_enemy);
//    if player.is_dead == true {return;}

    println!();

    println!("player exp is now: {}\r", player.exp);
//    println!("check enemy status:: is_dead? -> {}", test_enemy.is_dead);

    println!("player is currently dead? :: {}\r", player.check_status());

    println!();
    println!();
    println!();
    println!();

//    let mut in_bytes = stdin.bytes();
//    loop {
//        let input = in_bytes.next().unwrap().unwrap();
//        match input {
//            b'c' => {
//                let mut enemy = Enemy::new(EnemyType::Common, rand::thread_rng().gen_range(3, 15) * player.level);
//                combat::combat(&mut player, &mut enemy);
//            },
//            b'q' => return,
//            _a => {},
//        }
//    }// testing loop for combat shenanigans, byte styled inputs
    write!(stdout, "{}please enter an input... (q to quit, c for another combat sequence{}\n\r", color::Fg(nes_palette::NES_BRT_BLUE), color::Fg(color::Reset));

    for c in stdin.keys() {
        write!(stdout, "{}please enter an input... (q to quit, c for another combat sequence{}\n\r", color::Fg(nes_palette::NES_BRT_BLUE), color::Fg(color::Reset));
        match c.unwrap() {
            Key::Char('q') => return,
            Key::Char('c') => {
                let mut enemy = Enemy::new(EnemyType::Common, rand::thread_rng().gen_range(3, 15) * player.level);
                combat::combat(&mut player, &mut enemy);
            },
            _ => {},
        }
    }// for loop version of above loop
}
