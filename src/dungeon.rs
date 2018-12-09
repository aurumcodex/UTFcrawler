//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#![allow(dead_code)]
#![allow(warnings)]   //sticking fingersin ears about warnings


extern crate rand;
extern crate termion;     //imports for external libraries

use std::{thread, time};
use termion::clear;
use crate::game_state::palettes::*;
use crate::combat::*;                 //iports for the other libraries 
use crate::player::*;
use rand::Rng;
use termion::{color, cursor};

#[derive(Copy, Clone)]
pub struct map {    //struct for the map object
		pub output: [[usize; 32]; 32],	  //holds the map array and its tyep
		pub mapType: usize,
	}
pub struct pPos{  //struct to pass the player's position
	pub X: usize,
	pub Y: usize,
}
pub fn createMap(length: usize, width: usize, select: usize) -> map{   //function to generate a new map. returns a map struct

	//let mut select: usize = rand::thread_rng().gen_range(1, 6);
	let mut input: [[usize; 32]; 32]  = [[0; 32]; 32];    //map array
	let mut lootX: usize = 0;   //coordinates for the loot in rooms
	let mut lootY: usize = 0;
	
// select = 7;
	
	let mut a: usize = 0;   //itterators for nested while loops
    let mut i: usize = 0;
    
    while(i <= length){
		while(a <= width){
			
			if((i < 2 || i >length-2) || (a < 2 || a >width-2)) {
			input[i][a] = 1; //setting outer bounds to 'wall' characers
			}else{       //generateing map with checkered floor
				if(a%2 == 0 && i%2 ==0){input[i][a] = 2;
				}
				if(a%2 != 0 && i%2 ==0){input[i][a] = 3
				}
				if(a%2 == 0 && i%2 !=0){input[i][a] = 3;
				}
				if(a%2 != 0 && i%2 !=0){input[i][a] = 2;
				}
			}
			a+=1; //itteratig loops
		}
		i+=1;
		a=0;
	}
    
    i=0;
    a=0;//resetting itterators
    
    if(select == 4 || select == 5){   //creates 'L' shaped rooms. all 4 loops are structually identical, but act on different corners 
		let mut corner: usize = rand::thread_rng().gen_range(0, 4); //chooses random corner
		//corner=0; 
		if(length > 12 && width > 12){	  //if the room is too small, it is simply made a rectangualar room	
			if(corner == 0){
				while(i <= length/2){
					while(a <= width/2){  //loop that makes a chunk of the selected corner wall 
						input[i][a] = 1;
						a+=1;
					}
					i+=1;
					a=0;
				}
				i=0;
		        a=0;
				while(i <= length/2-2){   //makes a smaller section black to give the impression that the room is 'L' shaped 
					while(a <= width/2-2){
						input[i][a] = 0;
						a+=1;
					}
					i+=1;
					a=0;
				}
				input[1][width/2+width/4] = 4;
				input[1][width/2+width/4+1] = 4;   //setting position for the single door in these type of rooms
				
				input[length/2+length/4][2] = 5;  //setting player starting position
				lootX = rand::thread_rng().gen_range(length/2, length-2);
				lootY = rand::thread_rng().gen_range(width/2, width-2);      //position for the loot pickup item. 
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);    //the number dictates what item it is
				
				let X: usize = width/4;
				let Y: usize = length/2+length/4;
				let pOut = pPos{X,Y};            //putting the player's position in the player struct
			}						
		i = length;
		if(corner == 1){                    //the next three sections are the same structure, but knock out different corners
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
		input[1][width/2] = 4;  //settig doors on the top left and right of rectangluar rooms
		input[1][width/2+1] = 4;
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
		
		input[length-2][width/2] = 5;  //setting player position 
		
		if(width > 6){
			if(length > 6){
				lootX = rand::thread_rng().gen_range(2, length-2);  //setting loot position
				lootY = rand::thread_rng().gen_range(2, width-2);
				input[lootX][lootY] = rand::thread_rng().gen_range(6, 9);
			}
		}
		
		let X: usize = width/2;
		let Y: usize = length-2;   //inserting player position into its struct
		let pOut = pPos{X,Y};
	}
	
	
	 if(select == 7){   //creates the '+' shaped pre boss rooms
		 a = 25;
		 i = 0;    //setting itterators
		 while(i <= 8){
					while(a >= 17){
						input[i][a] = 1;  //makng the first corner wall
						a-=1;
					}
					i+=1;
					a=25;
				}
				a=25;
		        i=0;
				while(i <= 6){
					while(a >= 19){
						input[i][a] = 0;  //making a slightly smaller area of the first corner black
						a-=1;
					}
					i+=1;
					a=25;
				}
				
		 a = 0;
		 i = 0;
		 while(i <= 8){   //these also function the same jsut on different corners. similar to 'L' shaped rooms
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
		input[1][width/2+1] = 4;    //setting doors on the top left and right
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;   
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
		
		input[length-2][width/2] = 5;
		
		let X: usize = width/2;   //setting player position in the room
		let Y: usize = length-2;
		let pOut = pPos{X,Y};
	}
	
	if(select == 3 || select == 2 || select == 1){
		let X: usize = width/2;
		let Y: usize = length-2;  //setting player position in nromal rectangular rooms
		let pOut = pPos{X,Y};
	}
	
	//println!("{}", select);
	let output = input;
	let mapType = select; 		
	let mapOut = map{output, mapType};    //putting map information in the map struct 
	
	mapOut  //returnin the map
}

pub fn printMap(mapIn: map, length: usize, width: usize){  //function that interprets and prints the map to the terminal
	let mut d: u8= 0; //death animation itterator
	let mut output: [[usize; 32]; 32] = [[0; 32]; 32]; //output array
	output = mapIn.output;   //map taken is as argument
	let mapType: usize = mapIn.mapType;   //may type variable
	
	let mut i: usize = 0;
	let mut a: usize = 0;  //loop itterators
	
		if(mapType == 5 || mapType == 1){   //print for grey rooms with round floor tiles
			while(i <= length){
				while(a <= width){
					print!("{}", color::Fg(nes_palette::NES_LGT_GREY));  //wall color
					if(output[i][a] == 1){
						print!("▦ " ); //wall character
					}
					if(output[i][a] == 2){
						print!("{}▢ ", color::Fg(nes_palette::NES_MED_GREY));  //light floor color and its character
					}
					if(output[i][a] == 3){
						print!("{}◯ ", color::Fg(nes_palette::NES_DRK_GREY)); //dark floor color and its character
					} 
					if(output[i][a] == 0){
						print!("{}▦ ", color::Fg(nes_palette::NES_BLACK)); //blank character that are unseen
					}
					if(output[i][a] == 4){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));  //door characters
					}
					if(output[i][a] == 5){
						print!("{}☺ ", color::Fg(nes_palette::NES_YELLOW));   //the player character
					}
					if(output [i][a] > 5){
						print!("{}▲ ", color::Fg(nes_palette::NES_CYAN));   //loot character
					}
					a+=1;
				}
				print!("{}", color::Fg(nes_palette::NES_YELLOW));  //setting color for the next frame
				println!("\r"); //carraige return
				i+=1; //itterating loops
				a=0; //resetting loop
			}
		}
		if(mapType == 4 || mapType == 2){  //loop for blue walled rooms with checkered floors. same structure as above 
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
		if(mapType == 3){   //loop for the green walled goo rooms
			
			while(i <= length){
				while(a <= width){
					let mut choose: usize = rand::thread_rng().gen_range(0, 2);
					print!("{}", color::Fg(nes_palette::NES_GREEN));
					if(output[i][a] == 1){
						print!("▦ " );
					}
					if(output [i][a] != 1 && output [i][a] != 4 && output [i][a] < 5 && choose == 1 ){
						print!("{}▤ ", color::Fg(nes_palette::NES_BROWN));                               //ignores checkered pattern on map array and outputs a random position for the two colors
					}                                                                                   //this creates a different floor every time the player moves wiving the illusion of a liquid floor
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
    
    		if(mapType == 7 || mapType == 8){    //loops for boss rooms
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
    
    if(mapType == 6){   //loop for death splash screen. death stranscends the palette
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
\r"; //big raw string. basically a big sprite
			while(d < 250){  //loop for color
				println!("{}", clear::All); //clear terminal 
				//println!("{}", d);
				println!("{}", color::Fg(color::Rgb(d,0,0)));  //fades into red from black
				println!("{}",DEATH); //prints the big raw string in the color above as it fades to red
				d+=2;
				thread::sleep_ms(10); //waits for 10 milliseconds to time the fade animation
			}
    }	
}
