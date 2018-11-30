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
    let stdout = stdout().into_raw_mode().unwrap();
    //let mut stdout = stdout.lock();
    
    
    
	let mut byteInput = stdin.bytes();
	
	let mut mainMap = createMap(length, width, Type);
	
	//let mut stdout = stdout().into_raw_mode().unwrap();
	
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
	let mut player = Player::new(String::from("butch"),Archetype::Generalist);
	println!("{}", color::Fg(nes_palette::NES_BRT_GREEN));
	
	println!("{}",TITLE);
	
	println!("press 1 to select Alchemist\r");
	println!("press 2 to select Blackguard\r");
	println!("press 3 to select Generalist\r");
	println!("press 4 to select Gunner\r");
	println!("press 5 to select Mercinary\r");
	
	//boss = 1;
	let mut goo: usize = 0;
	let mut enemyType: u8 = 0; 
	let mut choose: u8 = 1;
	
	
	while(choose == 1){
	let input = byteInput.next().unwrap().unwrap();	
		match input{
			b'1' =>{
				player = Player::new(String::from("Annwyn"),Archetype::Alchemist);
				println!("you chose Annwyn, the Alchemist\r");
						choose = 0;
						println!("Press any key to continue\r");						
			},
			b'2' =>{
				player = Player::new(String::from("Skipp"),Archetype::Blackguard);
				println!("you chose Skipp, the Blackguard\r");
						choose = 0;
						println!("Press any key to continue\r");						
			},
			b'3' =>{
				player = Player::new(String::from("John"),Archetype::Generalist);
				println!("you chose John, the Generalist\r");
						choose = 0;
						println!("Press any key to continue\r");
			},
			b'4' =>{
				player = Player::new(String::from("Leda-26"),Archetype::Gunner);
				println!("you chose Leda-26, the Gunner\r");
						choose = 0;
						println!("Press any key to continue\r");
			},
			b'5' =>{
				player = Player::new(String::from("Dusk"),Archetype::Mercenary);
				println!("you chose Dusk, the Mercenary\r");
						choose = 0;
						println!("Press any key to continue\r");
			},
			_ => {
				println!("Invalid entry\r");
				},
		}	

	}
	
	while(run == 1){
		i = 0;
		j = 0;
		
		if(count >= 49){
			boss = 1;	
		}
		goo = rand::thread_rng().gen_range(0, 3);
		if(Type == 3 && goo == 1){
			player.hp -= 1;
		}
		
		let input = byteInput.next().unwrap().unwrap();		
	
		
		encounter = rand::thread_rng().gen_range(0, 100);
		println!("{}", clear::All);
		if(encounter >= 1 && encounter < floor + 1){
			enemyType = rand::thread_rng().gen_range(0, 255);
			//player.hp -= 10;
			let mut baddy = Enemy::new(enemy::EnemyType::Common, enemyType);
			combat::combat(&mut player, &mut baddy);
			
			//enter combat();
			//println!("OOF");
		}
		
		if(player.hp <= 0){
			Type = 6;
			mainMap = createMap(length, width, Type);
			println!("{}", clear::All);
				println!("dead");
				run = 0;
		}	
		match input{
		
		b'q' =>
			run = 0,
		
		
		b'a' =>{
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
			
		},
		b'w' =>{
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
		},
		b's' =>{
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
		},
		b'd' =>{
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
		},
		_=> {},
	}
		print!("room: ");
		print!("{}", count+1);
		print!("					Level: ");
		println!("{}\r", player.level);
		print!("Floor: ");
		print!("{}", floor);
		print!("				{}'s will to live: ", player.player_name);
		println!("{}\r", player.hp);
		printMap(mainMap, length, width);
		//println!("{}", input);
		//input = "".to_string();
		
		
	}
}
