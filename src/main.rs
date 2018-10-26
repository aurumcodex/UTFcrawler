//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
//!  Nicholas checking in!

extern crate rand;
extern crate ansi_term;
//extern crate termion;

mod player;
mod enemy;
mod status;
mod combat;

use std::io;
use self::player::*;
use self::enemy::*;
use self::status::*;
use self::combat::*;
use rand::Rng;
//use termion::{color, cursor, style};
use ansi_term::{Color, Style};

fn main() {
    // TODO: create all the functions and data types that we'll need to use.
    // that comes later, though.

//    termion variables
//    let hi_hp = color::Rgb(146, 180, 33);
//    let mid_hp = color::Rgb(189, 130, 35);
//    let low_hp = color::Rgb(189, 43, 35);
//    let norm = color::Rgb(155, 50, 135);

//    ansi_term variables
    let hi_hp = Color::RGB(146, 180, 33);
    let mid_hp = Color::RGB(189, 130, 35);
    let low_hp = Color::RGB(189, 42, 35);
    let norm = Color::RGB(155, 50, 135);
    
    println!("yaay testing stuff");


    println!("enter player's name:");
    let mut plyr_name: String = String::new();
    io::stdin().read_line(&mut plyr_name).expect("failed to read line");
    // let mut plyr_arch: Archetype;
    println!("enter player's archetype:");
    let mut plyr_type: String = String::new();
    io::stdin().read_line(&mut plyr_type).expect("failed to read line");
//    match plyr_type.as_str() {
//
//    }
    
    let mut test_player: Player = Player::new(plyr_name, Archetype::Alchemist);
//    termion
//    println!("{}{}Welcome, {}{}", color::Fg(norm), style::Bold, test_player.player_name, style::Reset);

//    ansi_term
    println!("{}{}", Style::new().bold().fg(norm).paint("Greetings, "), Style::new().bold().fg(norm).paint(&test_player.player_name));

    test_player.print_stats();
    test_player.incr_exp(rand::thread_rng().gen_range(5, 10));
    println!("player EXP is now {}", &test_player.exp);

    let mut test_enemy = Enemy {
        enemy_type: EnemyType::Common,
        enemy_id: -1,
        enemy_name: String::from("test enemy"),
        level: rand::thread_rng().gen_range(1, 10),
        given_exp: rand::thread_rng().gen_range(1, 10),
        hp: 10,
        mp: rand::thread_rng().gen_range(1, 10),
        phys_attk: rand::thread_rng().gen_range(1, 10),
        phys_def: rand::thread_rng().gen_range(1, 10),
        mag_attk: rand::thread_rng().gen_range(1, 10),
        mag_def: rand::thread_rng().gen_range(1, 10),
        strength: rand::thread_rng().gen_range(1, 10),
        magic: rand::thread_rng().gen_range(1, 10),
        vitality: rand::thread_rng().gen_range(1, 10),
        dexterity: rand::thread_rng().gen_range(1, 10),
        agility: rand::thread_rng().gen_range(1, 10),
        luck: rand::thread_rng().gen_range(1, 10),
        status: EnemyAilment::Normal,
        psyche: EnemyPsyche::Normal,
        is_dead: false,
    };
    test_enemy.print_stats();
    test_enemy.decr_hp(3);

    println!("enemy hp is now: {}", test_enemy.hp);
    println!("enemy status is  now: {:?}", test_enemy.status);

    test_enemy.print_stats();
    test_enemy.decr_hp(3);

    println!("enemy hp is now: {}☺", test_enemy.hp);
    println!("enemy status is  now: {:?}", test_enemy.status);

    test_enemy.print_stats();
    test_enemy.decr_hp(3);

    println!("enemy hp is now: {}", test_enemy.hp);
    println!("enemy status is  now: {:?}", test_enemy.status);

    test_enemy.print_stats();
    test_enemy.decr_hp(4);

    println!("enemy hp is now: {}", test_enemy.hp);
    println!("enemy status is  now: {:?}", test_enemy.status);
//    combat(&mut test_player);
//    let neg_float = -420.69f32;
//    println!("a negative float: {}", neg_float);
    println!();

    test_player.incr_exp(test_enemy.given_exp);
    println!("player exp is now: {}", test_player.exp);
    println!("check enemy status:: is_dead? -> {}", test_enemy.is_dead);

    println!("player is currently dead? :: {}", test_player.check_status());

    let mut test_num = 42;
    // note: loop within an if is still infinite loop
    while test_num < 50 {
        println!("num is :: {}", test_num);
        test_num += 1;
    }
}
