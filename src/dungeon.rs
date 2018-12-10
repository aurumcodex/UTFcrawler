//! # UTFcrawler
//!
//! ### Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
//! ## About
//! This is a program to render a top-down view of a dungeon crawler game that can be played in a
//! termial. A terminal size of 103 cols x 30 lines is highly recommended for the maps to display
//! correctly. A terminal font with a lot of Unicode character support is also recommended, as is a
//! terminal that displays 256 colors, or has RGB color support.
//!
//! This is the `dungeon.rs` file, which runs this application's dungeon creation and traversal.

#![allow(dead_code)]
#![allow(warnings)]


extern crate rand;
extern crate termion;

use rand::Rng;
use termion::clear;
use termion::{color, cursor};

use combat::*;
use player::*;
use game_state::palettes::*;

use std::{thread, time};

/// Struct to hold the map data.
#[derive(Copy, Clone)]
pub struct map {
		pub output: [[usize; 32]; 32],	
		pub mapType: usize,
	}

/// Struct for the Player's position.
pub struct pPos{
	pub X: usize,
	pub Y: usize,
}
// TODO: do dungeon crafting things

/// Creates a map and stores it into a variaple with the map struct type.
pub fn createMap(length: usize, width: usize, select: usize) -> map{

	//let mut select: usize = rand::thread_rng().gen_range(1, 6);
	let mut input: [[usize; 32]; 32]  = [[0; 32]; 32];
	let mut lootX: usize = 0;
	let mut lootY: usize = 0;
	
// select = 7;
	
	let mut a: usize = 0;
    let mut i: usize = 0;
    
    while(i <= length){
		while(a <= width){
			
			if((i < 2 || i >length-2) || (a < 2 || a >width-2)) {
			input[i][a] = 1;
			}else{
				if(a%2 == 0 && i%2 ==0){input[i][a] = 2;
				}
				if(a%2 != 0 && i%2 ==0){input[i][a] = 3
				}
				if(a%2 == 0 && i%2 !=0){input[i][a] = 3;
				}
				if(a%2 != 0 && i%2 !=0){input[i][a] = 2;
				}
			}
			a+=1;
		}
		i+=1;
		a=0;
	}
    
    i=0;
    a=0;
    
    if(select == 4 || select == 5){
		let mut corner: usize = rand::thread_rng().gen_range(0, 4);
		//corner=0; 
		if(length > 12 && width > 12){		
			if(corner == 0){
				while(i <= length/2){
					while(a <= width/2){
						input[i][a] = 1;
						a+=1;
					}
					i+=1;
					a=0;
				}
				i=0;
		        a=0;
				while(i <= length/2-2){
					while(a <= width/2-2){
						input[i][a] = 0;
						a+=1;
					}
					i+=1;
					a=0;
				}
				input[1][width/2+width/4] = 4;
				input[1][width/2+width/4+1] = 4;
				
				input[length/2+length/4][2] = 5;
				lootX = rand::thread_rng().gen_range(length/2, length-2);
				lootY = rand::thread_rng().gen_range(width/2, width-2);
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);
				
				let X: usize = width/4;
				let Y: usize = length/2+length/4;
				let pOut = pPos{X,Y};
			}						
		i = length;
		if(corner == 1){
				while(i >= length/2){
					while(a <= width/2){
						input[i][a] = 1;
						a+=1;
					}
					i-=1;
					a=0;
				}
				i=length;
		        a=0;
				while(i >= length/2+2){
					while(a <= width/2-2){
						input[i][a] = 0;
						a+=1;
					}
					i-=1;
					a=0;
				}
				input[length/4][1] = 4;
				input[length/4+1][1] = 4;
				
				input[length-2][width/2+width/4] = 5;
				
				lootX = rand::thread_rng().gen_range(2, length/2);
				lootY = rand::thread_rng().gen_range(2, width/2);
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);
				
				let X: usize = width/2+width/4;
				let Y: usize = length-2;
				let pOut = pPos{X,Y};
			}
			
		a = width;
		i = length;
		if(corner == 2){
				while(i >= length/2){
					while(a >= width/2){
						input[i][a] = 1;
						a-=1;
					}
					i-=1;
					a=width;
				}
				a=width;
		        i=length;
				while(i >= length/2+2){
					while(a >= width/2+2){
						input[i][a] = 0;
						a-=1;
					}
					i-=1;
					a=width;
				}
				input[length/4][width-1] = 4;
				input[length/4+1][width-1] = 4;
				
				input[length-2][width/4] = 5;
				
				lootX = rand::thread_rng().gen_range(length/4, length/2);
				lootY = rand::thread_rng().gen_range(width/4, width/2);
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);
				
				let X: usize = width/4;
				let Y: usize = length-2;
				let pOut = pPos{X,Y};
		
			}			
		a = width;
		i=0;
		if(corner == 3){
				while(i <= length/2){
					while(a >= width/2){
						input[i][a] = 1;
						a-=1;
					}
					i+=1;
					a=width;
				}
				a=width;
		        i=0;
				while(i <= length/2-2){
					while(a >= width/2+2){
						input[i][a] = 0;
						a-=1;
					}
					i+=1;
					a=width;
				}
				
				input[1][width/4] = 4;
				input[1][width/4+1] = 4;
				
				input[length/2+length/4][width-2] = 5;
				
				lootX = rand::thread_rng().gen_range(length/2, length-2);
				lootY = rand::thread_rng().gen_range(width/2, width-2);
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);
				
				let X: usize = width-2;
				let Y: usize = length/2+length/4;
				let pOut = pPos{X,Y};
			}
		}else{
			input[1][width/2] = 4;
			input[1][width/2+1] = 4;
				
			input[length/2][1] = 4;
			input[length/2+1][1] = 4;
				
			input[length/2][width-1] = 4;
			input[length/2+1][width-1] = 4;
			
			input[length-2][width/2] = 5;
			
			let X: usize = width/2;
			let Y: usize = length-2;
			let pOut = pPos{X,Y};
		}

	}
	
	 if(select == 3 || select == 2 || select == 1 || select == 8){
		input[1][width/2] = 4;
		input[1][width/2+1] = 4;
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
		
		input[length-2][width/2] = 5;
		
		if(width > 6){
			if(length > 6){
				lootX = rand::thread_rng().gen_range(2, length-2);
				lootY = rand::thread_rng().gen_range(2, width-2);
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);
			}
		}
		
		let X: usize = width/2;
		let Y: usize = length-2;
		let pOut = pPos{X,Y};
	}
	
	
	 if(select == 7){
		 a = 25;
		 i = 0;
		 while(i <= 8){
					while(a >= 17){
						input[i][a] = 1;
						a-=1;
					}
					i+=1;
					a=25;
				}
				a=25;
		        i=0;
				while(i <= 6){
					while(a >= 19){
						input[i][a] = 0;
						a-=1;
					}
					i+=1;
					a=25;
				}
				
		 a = 0;
		 i = 0;
		 while(i <= 8){
					while(a <= 8){
						input[i][a] = 1;
						a+=1;
					}
					i+=1;
					a=0;
				}
				a=0;
		        i=0;
				while(i <= 6){
					while(a <= 6){
						input[i][a] = 0;
						a+=1;
					}
					i+=1;
					a=0;
				}
				
		 a = 0;
		 i = 25;
		 while(i >= 17){
					while(a <= 8){
						input[i][a] = 1;
						a+=1;
					}
					i-=1;
					a=0;
				}
				a=0;
		        i=25;
				while(i >= 19){
					while(a <= 6){
						input[i][a] = 0;
						a+=1;
					}
					i-=1;
					a=0;
				}
		
		 a = 25;
		 i = 25;
		 while(i >= 17){
					while(a >= 17){
						input[i][a] = 1;
						a-=1;
					}
					i-=1;
					a=25;
				}
				a=25;
		        i=25;
				while(i >= 19){
					while(a >= 19){
						input[i][a] = 0;
						a-=1;
					}
					i-=1;
					a=25;
				}
		 
		input[1][width/2] = 4;
		input[1][width/2+1] = 4;
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
		
		input[length-2][width/2] = 5;
		
		let X: usize = width/2;
		let Y: usize = length-2;
		let pOut = pPos{X,Y};
	}
	
	if(select == 3 || select == 2 || select == 1){
		let X: usize = width/2;
		let Y: usize = length-2;
		let pOut = pPos{X,Y};
	}
	
	//println!("{}", select);
	let output = input;
	let mapType = select; 		
	let mapOut = map{output, mapType}; 
	
	mapOut
}

/// Prints the map so that the player can see where they are, and where the doors are.
pub fn printMap(mapIn: map, length: usize, width: usize){
	let mut d: u8= 0;
	let mut output: [[usize; 32]; 32] = [[0; 32]; 32];
	output = mapIn.output;
	let mapType: usize = mapIn.mapType;
	
	let mut i: usize = 0;
	let mut a: usize = 0;
	
		if(mapType == 5 || mapType == 1){
			while(i <= length){
				while(a <= width){
					print!("{}", color::Fg(nes_palette::NES_LGT_GREY));
					if(output[i][a] == 1){
						print!("▦ " );
					}
					if(output[i][a] == 2){
						print!("{}▢ ", color::Fg(nes_palette::NES_MED_GREY));
					}
					if(output[i][a] == 3){
						print!("{}◯ ", color::Fg(nes_palette::NES_DRK_GREY));
					}
					if(output[i][a] == 0){
						print!("{}▦ ", color::Fg(nes_palette::NES_BLACK));
					}
					if(output[i][a] == 4){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));
					}
					if(output[i][a] == 5){
						print!("{}☺ ", color::Fg(nes_palette::NES_YELLOW));
					}
					if(output [i][a] > 5){
						print!("{}▲ ", color::Fg(nes_palette::NES_CYAN));
					}
					a+=1;
				}
				print!("{}", color::Fg(nes_palette::NES_YELLOW));
				println!("\r");
				i+=1;
				a=0;
			}
		}
		if(mapType == 4 || mapType == 2){
			while(i <= length){
				while(a <= width){
					print!("{}", color::Fg(nes_palette::NES_BLUE));
					if(output[i][a] == 1){
						print!("▦ " );
					}
					if(output[i][a] == 2){
						print!("{}▧ ", color::Fg(nes_palette::NES_LGT_GREY));
					}
					if(output[i][a] == 3){
						print!("{}▨ ", color::Fg(nes_palette::NES_MED_GREY));
					}
					if(output[i][a] == 0){
						print!("{}▦ ", color::Fg(nes_palette::NES_BLACK));
					}
					if(output[i][a] == 4){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));
					}
					if(output[i][a] == 5){
						print!("{}☺ ", color::Fg(nes_palette::NES_YELLOW));
					}
					if(output [i][a] > 5){
						print!("{}▲ ", color::Fg(nes_palette::NES_CYAN));
					}
					a+=1;
				}
				print!("{}", color::Fg(nes_palette::NES_YELLOW));
				println!("\r");
				i+=1;
				a=0;
			}
		}
		if(mapType == 3){
			
			while(i <= length){
				while(a <= width){
					let mut choose: usize = rand::thread_rng().gen_range(0, 2);
					print!("{}", color::Fg(nes_palette::NES_GREEN));
					if(output[i][a] == 1){
						print!("▦ " );
					}
					if(output [i][a] != 1 && output [i][a] != 4 && output [i][a] < 5 && choose == 1 ){
						print!("{}▤ ", color::Fg(nes_palette::NES_BROWN));
					}
					if(output [i][a] != 1 && output [i][a] != 4 && output [i][a] < 5 && choose == 0){
						print!("{}▥ ", color::Fg(nes_palette::NES_RED));
					}
					if(output[i][a] == 4){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));
					}
					if(output[i][a] == 5){
						print!("{}☺ ", color::Fg(nes_palette::NES_YELLOW));
					}
					if(output [i][a] > 5){
						print!("{}▲ ", color::Fg(nes_palette::NES_CYAN));
					}
					a+=1;
				}
				print!("{}", color::Fg(nes_palette::NES_YELLOW));
				println!("\r");
				i+=1;
				a=0;
			}
    }
    
    		if(mapType == 7 || mapType == 8){
				//println!("{}", mapType);
			while(i <= length){
				while(a <= width){
					print!("{}", color::Fg(nes_palette::NES_PURPLE));
					if(output[i][a] == 1){
						print!("▦ " );
					}
					if(output[i][a] == 2){
						print!("{}◇ ", color::Fg(nes_palette::NES_BLUE));
					}
					if(output[i][a] == 3){
						print!("{}◆ ", color::Fg(nes_palette::NES_RED));
					}
					if(output[i][a] == 0){
						print!("{}▦ ", color::Fg(nes_palette::NES_BLACK));
					}
					if(output[i][a] == 4 && mapType == 8){
						print!("{}▦ ", color::Fg(nes_palette::NES_PURPLE));
					}
					if(output[i][a] == 4 && mapType == 7){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));
					}
					if(output[i][a] == 5){
						print!("{}☺ ", color::Fg(nes_palette::NES_YELLOW));
					}
					if(output [i][a] > 5){
						print!("{}▲ ", color::Fg(nes_palette::NES_CYAN));
					}
					a+=1;
				}
				print!("{}", color::Fg(nes_palette::NES_YELLOW));
				println!("\r");
				i+=1;
				a=0;
			}
		}
    
    if(mapType == 6){
			pub const DEATH: &str = "\r
@@@ @@@   @@@@@@   @@@  @@@     @@@@@@@   @@@  @@@@@@@@  @@@@@@@\r
@@@ @@@  @@@@@@@@  @@@  @@@     @@@@@@@@  @@@  @@@@@@@@  @@@@@@@@\r
@@! !@@  @@!  @@@  @@!  @@@     @@!  @@@  @@!  @@!       @@!  @@@\r
!@! @!!  !@!  @!@  !@!  @!@     !@!  @!@  !@!  !@!       !@!  @!@\r
 !@!@!   @!@  !@!  @!@  !@!     @!@  !@!  !!@  @!!!:!    @!@  !@!\r
  @!!!   !@!  !!!  !@!  !!!     !@!  !!!  !!!  !!!!!:    !@!  !!!\r
  !!:    !!:  !!!  !!:  !!!     !!:  !!!  !!:  !!:       !!:  !!!\r
  :!:    :!:  !:!  :!:  !:!     :!:  !:!  :!:  :!:       :!:  !:!\r
   ::    ::::: ::  ::::: ::      :::: ::   ::   :: ::::   :::: ::\r
   :      : :  :    : :  :      :: :  :   :    : :: ::   :: :  : \r
\r";
			while(d < 250){
				println!("{}", clear::All);
				//println!("{}", d);
				println!("{}", color::Fg(color::Rgb(d,0,0)));
				println!("{}",DEATH);
				d+=2;
				thread::sleep_ms(10);
			}
    }	
}
