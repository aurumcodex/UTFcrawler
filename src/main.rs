//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
//!  Nicholas checking in!

#![allow(dead_code)]
#![allow(warnings)]

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

use termion::clear;
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
    let mut Type: usize = rand::thread_rng().gen_range(1, 6);
    let mut run: usize = 1;
    
    let mut encounter: usize = 0;
    let mut boss = 0;
    let mut count = 0;
    let mut floor = 0;
    
    let mut playerX: usize = 0;
	let mut playerY: usize = 0;
    
    let stdin = stdin();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    
	let mut input: String = String::new();
	
	let mut mainMap = createMap(length, width, Type);
	
	
	
	let mut i: usize = 0;
	let mut j: usize = 0;
	while(i < 32){
		while(j < 32){
			//print!("{}",mainMap.output[i][j]);
			if(mainMap.output[i][j]	== 5){
				playerX = i;
				playerY = j;				
			}
			j += 1;
		}
		i+=1;
		j = 0;
		//println!("");
	}
	//println!("{}",playerX);
	//println!("{}",playerY);
	let mut player = Player::new(String::from("butch"),Archetype::Generalist);
	
	println!("press Enter to start");
	
	//boss = 1;
	
	while(run == 1){
		
		if(count >= 49){
			boss = 1;	
		}
				
		io::stdin().read_line(&mut input).expect("failed to read line");
		//run = 0;
		
		encounter = rand::thread_rng().gen_range(0, 50);
		println!("{}", clear::All);
		if(encounter >= 1 && encounter < floor + 1){
			player.hp -= 10;
			//enter combat();
			println!("OOF");
		}
		
		if(player.hp <= 0){
			Type = 6;
			mainMap = createMap(length, width, Type);
			println!("{}", clear::All);
				println!("dead");
				run = 0;
		}
		
		if(input.eq(&String::from("q\n"))){
			run = 0;
		}
		
		if(input.eq(&String::from("a\n"))){
			if(mainMap.output[playerX][playerY-1] == 2 || mainMap.output[playerX][playerY-1] > 5){
				mainMap.output[playerX][playerY] = 3;
			}
			if(mainMap.output[playerX][playerY-1] == 3 || mainMap.output[playerX][playerY-1] > 5){
				mainMap.output[playerX][playerY] = 2;
			}
			if(mainMap.output[playerX][playerY-1]!=1){
			playerY -=1;
			}
			if(mainMap.output[playerX][playerY] == 4){
				if(boss == 0){
				Type = rand::thread_rng().gen_range(1, 6);
				length = rand::thread_rng().gen_range(5, 26);
				width = rand::thread_rng().gen_range(5, 26);
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54){
					Type = 8;	
					length = 25;
					width = 25;
				}else{
					Type = 7;
					length = 25;
					width = 25;
				}
				mainMap.output[playerX][playerY] = 2;
				mainMap = createMap(length, width, Type);
				count += 1;
				i=0;
				while(i < 32){
				while(j < 32){
					if(mainMap.output[i][j]	== 5){
						playerX = i;
						playerY = j;				
					}
					j += 1;
				}
				i+=1;
				j = 0;

				}
			}
			if(mainMap.output[playerX][playerY] > 5){
				//inventory.add(mainMap[playerX][playerY]);
			}
			mainMap.output[playerX][playerY] = 5; 
			
		}
		if(input.eq(&String::from("w\n"))){
			if(mainMap.output[playerX-1][playerY] == 2 || mainMap.output[playerX-1][playerY] > 5){
				mainMap.output[playerX][playerY] = 3;
			}
			if(mainMap.output[playerX-1][playerY] == 3 || mainMap.output[playerX-1][playerY] > 5){
				mainMap.output[playerX][playerY] = 2;
			}
			if(mainMap.output[playerX-1][playerY]!=1){
			playerX -=1;
			}
			if(mainMap.output[playerX][playerY] == 4){
				if(boss == 0){
				Type = rand::thread_rng().gen_range(1, 6);
				length = rand::thread_rng().gen_range(5, 26);
				width = rand::thread_rng().gen_range(5, 26);
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54){
					Type = 8;	
					length = 25;
					width = 25;
				}else{
					Type = 7;
					length = 25;
					width = 25;
				}
				mainMap.output[playerX][playerY] = 2;
				mainMap = createMap(length, width, Type);
				count += 1;
				i=0;
				while(i < 32){
				while(j < 32){
					if(mainMap.output[i][j]	== 5){
						playerX = i;
						playerY = j;				
					}
					j += 1;
				}
				i+=1;
				j = 0;
				}
			}
			mainMap.output[playerX][playerY] = 5; 
		}
		if(input.eq(&String::from("s\n"))){
			if(mainMap.output[playerX+1][playerY] == 2 || mainMap.output[playerX+1][playerY] > 5){
				mainMap.output[playerX][playerY] = 3;
			}
			if(mainMap.output[playerX+1][playerY] == 3 || mainMap.output[playerX+1][playerY] > 5){
				mainMap.output[playerX][playerY] = 2;
			}
			if(mainMap.output[playerX+1][playerY]!=1){
			playerX +=1;
			}
			if(mainMap.output[playerX][playerY] == 4){
				if(boss == 0){
				Type = rand::thread_rng().gen_range(1, 6);
				length = rand::thread_rng().gen_range(5, 26);
				width = rand::thread_rng().gen_range(5, 26);
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54){
					Type = 8;	
					length = 25;
					width = 25;
				}else{
					Type = 7;
					length = 25;
					width = 25;
				}
				mainMap.output[playerX][playerY] = 2;
				mainMap = createMap(length, width, Type);
				count += 1;
				i=0;
				while(i < 32){
				while(j < 32){
					if(mainMap.output[i][j]	== 54){
						playerX = i;
						playerY = j;				
					}
					j += 1;
				}
				i+=1;
				j = 0;
				}
			}
			mainMap.output[playerX][playerY] = 5; 
		}
		if(input.eq(&String::from("d\n"))){
			if(mainMap.output[playerX][playerY+1] == 2 || mainMap.output[playerX][playerY+1] > 5){
				mainMap.output[playerX][playerY] = 3;
			}
			if(mainMap.output[playerX][playerY+1] == 3 || mainMap.output[playerX][playerY+1] > 5){
				mainMap.output[playerX][playerY] = 2;
			}
			if(mainMap.output[playerX][playerY+1]!=1){
			playerY +=1;
			}
			if(mainMap.output[playerX][playerY] == 4){
				if(boss == 0){
				Type = rand::thread_rng().gen_range(1, 6);
				length = rand::thread_rng().gen_range(5, 26);
				width = rand::thread_rng().gen_range(5, 26);
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54){
					Type = 8;	
					length = 25;
					width = 25;
				}else{
					Type = 7;
					length = 25;
					width = 25;
				}
				mainMap.output[playerX][playerY] = 2;
				mainMap = createMap(length, width, Type);
				count += 1;
				i=0;
				while(i < 32){
				while(j < 32){
					if(mainMap.output[i][j]	== 5){
						playerX = i;
						playerY = j;				
					}
					j += 1;
				}
				i+=1;
				j = 0;
				}
			};
			mainMap.output[playerX][playerY] = 5; 
		}
		//println!("{}", Type);
		//println!("{}", boss);
		print!("room: ");
		println!("{}", count+1);
		print!("Floor: ");
		println!("{}", floor);
		print!("Will to live: ");
		println!("{}", player.hp);
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
