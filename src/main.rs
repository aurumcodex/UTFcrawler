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

use std::io;
use std::io::{Read, Write, stdout, stdin};
use self::player::*;
use self::enemy::*;
use self::status::*;
// use self::combat::*;
use self::dungeon::*;
use self::inventory::*;
use self::game_state::TITLE;
use self::game_state::palettes::*;
use rand::Rng;
use termion::{color, cursor, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;


 fn main() {

    let mut length: usize = rand::thread_rng().gen_range(5, 26);
    let mut width: usize = rand::thread_rng().gen_range(5, 26);
    let mut run: usize = 1;
    
    let stdin = stdin();
    // let mut stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    
	let mut input: String = String::new();
 
	let mainMap = createMap(length, width);
	printMap(mainMap, length, width);
	println!("");
	printMap(mainMap, length, width);
	
	while(run == 1){
		//let mut input = stdin.read_line(&mut input);
		io::stdin().read_line(&mut input).expect("failed to read line");
		//run = 0;
		if(input.eq(&String::from("close\n"))){
			run = 0;
		}
		printMap(mainMap, length, width);
		println!("{}", input);
		input = "".to_string();
		
	}
	
}


/*
fn main() {
    // TODO: create all the functions and data types that we'll need to use.
    // that comes later, though.

    let stdin = stdin();
    // let mut stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    // stdout = stdout.lock();

    let norm = color::Rgb(104, 68, 252);
    
    writeln!(stdout, "{} {} {}", color::Fg(nes_palette::NES_BRT_GREEN), TITLE, color::Fg(color::Reset));

    println!("enter player's name: \r");
    let mut plyr_name: String = String::new();
    // io::stdin().read_line(&mut plyr_name).expect("failed to read line");
    let plyr_name = stdin.read_line(&mut plyr_name);

//    let plyr_type = stdin.read_line();

    let mut plyr_arch = Archetype::None;

    while plyr_arch == Archetype::None {
         let mut plyr_type: String = String::new();
        println!("enter the archetype you wish to use:: ");
        // io::stdin().read_line(&mut plyr_type).expect("failed to read line");
        //
        let mut plyr_type = stdin.read_line(&mut plyr_type);
        plyr_arch = Archetype::Gunner;
        // TODO: need to figure out how to parse string inputs from Results and match that stuff.
        //
//        if plyr_type.eq(&String::from("Alchemist")) || plyr_type.eq(&String::from("alchemist")) {
//            plyr_arch = Archetype::Alchemist;
//        }
//        if plyr_type.eq(&String::from("Blackguard")) || plyr_type.eq(&String::from("blackguard")) {
//            plyr_arch = Archetype::Blackguard;
//        }
//        if plyr_type.eq(&String::from("Generalist")) || plyr_type.eq(&String::from("generalist")) {
//            plyr_arch = Archetype::Generalist;
//        }
//        if plyr_type.eq(&String::from("Gunner")) || plyr_type.eq(&String::from("gunner")) {
//            plyr_arch = Archetype::Gunner;
//        }
//        if plyr_type.eq(&String::from("Mercenary")) || plyr_type.eq(&String::from("mercenary")) {
//            plyr_arch = Archetype::Mercenary;
//        }
//        else { println!("please reenter your choice:: "); }
    }

    // let plyr_arch = Archetype::Gunner;

    let mut test_player: Player = Player::new(String::from("bacon"), plyr_arch);

    // let mut test_player: Player = Player::new(plyr_name, plyr_arch);

    println!("{}{}greetings, {}{}{}", style::Bold, color::Fg(norm), test_player.player_name,
                        color::Fg(color::Reset), style::Reset);

    test_player.print_stats();
    test_player.gain_exp(rand::thread_rng().gen_range(5, 10));
    println!("player EXP is now {}", &test_player.exp);


    let mut test_enemy = Enemy::new(EnemyType::Common, rand::thread_rng().gen_range(3, 15) * test_player.level);

    println!();

    combat::combat(&mut test_player, &mut test_enemy);

    println!();

    println!("player exp is now: {}", test_player.exp);
    println!("check enemy status:: is_dead? -> {}", test_enemy.is_dead);

    println!("player is currently dead? :: {}", test_player.check_status());

    let test_int_as_float: i8 = 1.1 as i8;
    println!("testing int as a float :: {}", test_int_as_float);

    println!();

    let nes_msg = "this is a string to print out in many a colour using the NES palette!";
    println!("{}{}{}{}{}", color::Fg(nes_palette::NES_BLACK), color::Bg(nes_palette::NES_WHITE),
             nes_msg, color::Bg(color::Reset), color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_BLUE), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_BROWN), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_BRT_BLUE), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_BRT_GREEN), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_BRT_RED), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_CYAN), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_DRK_GREY), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_GREEN), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_LGT_GREY), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_MED_GREY), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_ORANGE), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_PURPLE), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_RED), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_WHITE), nes_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(nes_palette::NES_YELLOW), nes_msg, color::Fg(color::Reset));
//    write!(stdout, "{}{}{}{}{}", color::Fg(nes_palette::NES_BLACK), color::Bg(nes_palette::NES_WHITE),
//            nes_msg, color::Bg(color::Reset), color::Fg(color::Reset));


    println!();

    // let c64_msg = "this is a string to print out in many a colour using the C64 palette!";
    // println!("{}{}{}{}{}", color::Fg(c64_palette::C64_BLACK), color::Bg(c64_palette::C64_WHITE),
    //          c64_msg, color::Bg(color::Reset), color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_WHITE), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_RED), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_CYAN), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_PURPLE), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_GREEN), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_BLUE), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_YELLOW), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_BROWN), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_LGT_RED), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_DRK_GREY), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_ORANGE), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_MED_GREY), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_LGT_GREEN), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_LGT_BLUE), c64_msg, color::Fg(color::Reset));
    // println!("{}{}{}", color::Fg(c64_palette::C64_LGT_GREY), c64_msg, color::Fg(color::Reset));

    println!();

    let default_msg = "this is a string to print out in many a colour using the default terminal color palette!";
    println!("{}{}{}{}{}", color::Fg(default_palette::DFLT_BLACK), color::Bg(default_palette::DFLT_WHITE),
             default_msg, color::Bg(color::Reset), color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_RED), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_GREEN), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_YELLOW), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_BLUE), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_MAGENTA), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_CYAN), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_WHITE), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_BLACK), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_RED), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_GREEN), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_YELLOW), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_BLUE), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_MAGENTA), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_CYAN), default_msg, color::Fg(color::Reset));
    println!("{}{}{}", color::Fg(default_palette::DFLT_LGT_WHITE), default_msg, color::Fg(color::Reset));

    println!();

    test_player.print_stats();

    println!();

    println!();


    // while test_player.level < 13 {
    //     test_player.gain_exp(20);
    //     if test_player.level == 10 { break; }
    // }


    game_state::check_colors();

    let mut n:u8 = 1;
    let mut q = 1;
    while q < 17 {
//        n <<= 1;
//        n /= 2;
        println!("n is : {}", n);
        n = (n+2) / 2 << 1;
        q += 1;
    }

//    println!("{} {} {}", color::Fg(nes_palette::NES_PURPLE), TITLE, color::Fg(color::Reset));
//    println!("{} {} {}", color::Fg(nes_palette::NES_BRT_GREEN), TITLE, color::Fg(color::Reset));
    writeln!(stdout, "{} {} {}", color::Fg(nes_palette::NES_BRT_GREEN), TITLE, color::Fg(color::Reset));
//    println!("{} {} {}", color::Fg(default_palette::DFLT_LGT_GREEN), TITLE, color::Fg(color::Reset));
    println!();

    let mut test_enemy_2 = Enemy::new(EnemyType::Common, rand::thread_rng().gen_range(3, 15) * test_player.level);

    println!();

    combat::combat(&mut test_player, &mut test_enemy_2);
}
*/
