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
	let mut player = Player::new(String::from("butch"),Archetype::Generalist);
	
	println!("press Enter to start");
	
	//boss = 1;
	
	while(run == 1){
		i = 0;
		j = 0;
		
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
			//println!("OOF");
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
