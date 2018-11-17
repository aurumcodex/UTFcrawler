//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

#![allow(dead_code)]

extern crate rand;
extern crate termion;

use crate::game_state::palettes::*;
use crate::combat::*;
use rand::Rng;
use termion::{color, cursor};

pub struct map {
		output: [[usize; 32]; 32],	
		mapType: usize,
	}

// TODO: do dungeon crafting things

pub fn createMap(length: usize, width: usize) -> map{

	let mut select: usize = rand::thread_rng().gen_range(1, 6);
	let mut input: [[usize; 32]; 32]  = [[0; 32]; 32];
	
	
 //select = 1;
	
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
		//corner=1; 
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
			}
		}else{
			input[1][width/2] = 4;
			input[1][width/2+1] = 4;
				
			input[length/2][1] = 4;
			input[length/2+1][1] = 4;
				
			input[length/2][width-1] = 4;
			input[length/2+1][width-1] = 4;
		}

	}
			
	/*	if(select == 5 || select == 4){
		let mut corner: usize = rand::thread_rng().gen_range(0, 4);
		//corner=1; 
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
			}
		}
	}*/
	
	if(select == 1){
		input[1][width/2] = 4;
		input[1][width/2+1] = 4;
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
	}
	 if(select == 2){
		input[1][width/2] = 4;
		input[1][width/2+1] = 4;
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
	}
	 if(select == 3){
		input[1][width/2] = 4;
		input[1][width/2+1] = 4;
			
		input[length/2][1] = 4;
		input[length/2+1][1] = 4;
			
		input[length/2][width-1] = 4;
		input[length/2+1][width-1] = 4;
	}
	println!("{}", select);
	let output = input;
	let mapType = select; 		
	let mapOut = map{output, mapType}; 
	
	mapOut
}

pub fn printMap(mapIn: map, length: usize, width: usize){
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
					a+=1;
				}
				println!("");
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
						print!("{}▧ ", color::Fg(nes_palette::NES_WHITE));
					}
					if(output[i][a] == 3){
						print!("{}▨ ", color::Fg(nes_palette::NES_LGT_GREY));
					}
					if(output[i][a] == 0){
						print!("{}▦ ", color::Fg(nes_palette::NES_BLACK));
					}
					if(output[i][a] == 4){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));
					}
					a+=1;
				}
				println!("");
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
					if(output [i][a] != 1 && output [i][a] != 4 && choose == 1){
						print!("{}▤ ", color::Fg(nes_palette::NES_BROWN));
					}
					if(output [i][a] != 1 && output [i][a] != 4 && choose == 0){
						print!("{}▥ ", color::Fg(nes_palette::NES_RED));
					}
					if(output[i][a] == 4){
						print!("{}▦ ", color::Fg(nes_palette::NES_BRT_RED));
					}
					a+=1;
				}
				println!("");
				i+=1;
				a=0;
			}
    }
		
		
}
