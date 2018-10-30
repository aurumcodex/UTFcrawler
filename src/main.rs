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
mod game_state;

use std::io;
use self::player::*;
use self::enemy::*;
use self::status::*;
use self::combat::*;
use self::game_state::palettes::*;
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
    let black = Color::RGB(0, 0, 0);
    let drk_gray = Color::RGB(78, 83, 92);
    let med_gray = Color::RGB(142, 152, 167);
    let lgt_gray = Color::RGB(193, 198, 207);
    let white = Color::RGB(244, 245, 247);
    let norm = Color::RGB(104, 68, 252);

    println!("yaay testing stuff");


    println!("enter player's name:");
    let mut plyr_name: String = String::new();
    io::stdin().read_line(&mut plyr_name).expect("failed to read line");

    let mut plyr_arch = Archetype::None;

    while plyr_arch == Archetype::None {
        let mut plyr_type: String = String::new();
        println!("enter the archetype you wish to use:: ");
        io::stdin().read_line(&mut plyr_type).expect("failed to read line");
        if plyr_type.trim().eq(&String::from("Alchemist")) || plyr_type.trim().eq(&String::from("alchemist")) { plyr_arch = Archetype::Alchemist; }
        if plyr_type.trim().eq(&String::from("Blackguard")) || plyr_type.trim().eq(&String::from("blackguard")) { plyr_arch = Archetype::Blackguard; }
        if plyr_type.trim().eq(&String::from("Generalist")) || plyr_type.trim().eq(&String::from("generalist")) { plyr_arch = Archetype::Generalist; }
        if plyr_type.trim().eq(&String::from("Gunner")) || plyr_type.trim().eq(&String::from("gunner")) { plyr_arch = Archetype::Gunner; }
        if plyr_type.trim().eq(&String::from("Mercenary")) || plyr_type.trim().eq(&String::from("mercenary")) { plyr_arch = Archetype::Mercenary; }
        else { println!("please reenter your choice:: "); }
    }

    
    let mut test_player: Player = Player::new(plyr_name, plyr_arch);
//    termion
//    println!("{}{}Welcome, {}{}", color::Fg(norm), style::Bold, test_player.player_name, style::Reset);

//    ansi_term
    println!("{}{}", Style::new().bold().fg(norm).paint("Greetings, "), Style::new().bold().fg(norm).paint(&test_player.player_name));

    test_player.print_stats();
    test_player.gain_exp(rand::thread_rng().gen_range(5, 10));
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

    println!();

    combat::combat(&mut test_player, &mut test_enemy);

    println!();

//    test_player.gain_exp(test_enemy.given_exp);
    println!("player exp is now: {}", test_player.exp);
    println!("check enemy status:: is_dead? -> {}", test_enemy.is_dead);

    println!("player is currently dead? :: {}", test_player.check_status());

    let mut test_num = 42;
    // note: loop within an if is still infinite loop
    while test_num < 50 {
        println!("num is :: {}", test_num);
        test_num += 1;
    }

    println!();

    let nes_meg = "this is a string to print out in many a colour using the NES palette!";
    println!("{}", Style::new().fg(nes_palette::NES_BLACK).on(nes_palette::NES_WHITE).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_BLUE).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_BROWN).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_BRT_BLUE).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_BRT_GREEN).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_BRT_RED).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_CYAN).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_DRK_GREY).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_GREEN).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_LGT_GREY).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_MED_GREY).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_ORANGE).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_PURPLE).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_RED).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_WHITE).paint(nes_meg));
    println!("{}", Style::new().fg(nes_palette::NES_YELLOW).paint(nes_meg));

    println!();

    let c64_msg = "this is a string to print out in many a colour using the C64 palette!";
    println!("{}", Style::new().fg(c64_palette::C64_BLACK).on(c64_palette::C64_WHITE).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_BLUE).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_BROWN).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_CYAN).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_DRK_GREY).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_GREEN).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_LGT_BLUE).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_LGT_GREEN).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_LGT_GREY).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_LGT_RED).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_MED_GREY).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_ORANGE).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_PURPLE).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_RED).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_WHITE).paint(c64_msg));
    println!("{}", Style::new().fg(c64_palette::C64_YELLOW).paint(c64_msg));

    println!();

    let atari_msg = "this is a string to print out in many a colour using the Atari 2600 palette!";
    println!("{}", Style::new().fg(atari_palette::ATARI_BLACK).on(atari_palette::ATARI_WHITE).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_BLUE).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_BLUE2).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_BROWN).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_DRK_GREY).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_GREEN).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_GREEN2).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_LGT_GREY).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_OLIVE).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_ORANGE).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_PURPLE).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_RED).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_RED2).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_TEAL).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_WHITE).paint(atari_msg));
    println!("{}", Style::new().fg(atari_palette::ATARI_YELLOW).paint(atari_msg));
    
    println!("this is a test line written in neovim. please ignore it");
}
