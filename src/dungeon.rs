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


// TODO: do dungeon crafting things

fn drawMap(mut input: [[usize; 32]; 32], length: usize, width: usize) {
	
	//pub const NES_BLACK: termion::color::Rgb = termion::color::Rgb(0, 0, 0); 
    //pub const NES_DRK_GREY: termion::color::Rgb = termion::color::Rgb(97, 97, 97); 
    //pub const NES_MED_GREY: termion::color::Rgb = termion::color::Rgb(124, 124, 124); 
    //pub const NES_LGT_GREY: termion::color::Rgb = termion::color::Rgb(188, 188, 188);
    //pub const NES_WHITE: termion::color::Rgb = termion::color::Rgb(248, 248, 248);
    //pub const NES_RED: termion::color::Rgb = termion::color::Rgb(168, 16, 0); 
    //pub const NES_BRT_RED: termion::color::Rgb = termion::color::Rgb(248, 120, 88);
    //pub const NES_ORANGE: termion::color::Rgb = termion::color::Rgb(228, 92, 16);
    //pub const NES_YELLOW: termion::color::Rgb = termion::color::Rgb(248, 184, 0); 
    //pub const NES_GREEN: termion::color::Rgb = termion::color::Rgb(0, 184, 0); 
    //pub const NES_BRT_GREEN: termion::color::Rgb = termion::color::Rgb(184, 248, 24); 
    //pub const NES_CYAN: termion::color::Rgb = termion::color::Rgb(0, 232, 216); 
    //pub const NES_BLUE: termion::color::Rgb = termion::color::Rgb(0, 120, 248); 
    //pub const NES_BRT_BLUE: termion::color::Rgb = termion::color::Rgb(60, 188, 252); 
    //pub const NES_PURPLE: termion::color::Rgb = termion::color::Rgb(104, 68, 252); 
    //pub const NES_BROWN: termion::color::Rgb = termion::color::Rgb(80, 48, 0);
    
	
	 let mut select: usize = rand::thread_rng().gen_range(1, 6);
	
	//select = 4;
	
	let mut a: usize = 0;
    let mut i: usize = 0;
    
	if(select == 1){
	while(i <= length){
		while(a <= width){
			print!("{}", color::Fg(NES_LGT_GREY));
			if(input[i][a] == 1){
				print!("▦ " );
			}
			if(input[i][a] == 2){
				print!("{}▢ ", color::Fg(NES_MED_GREY));
			}
			if(input[i][a] == 3){
				print!("{}◯ ", color::Fg(NES_DRK_GREY));
			}
			a+=1;
		}
		println!("");
		i+=1;
		a=0;
	}
    }
    
    if(select == 2){
	while(i <= length){
		while(a <= width){
			print!("{}", color::Fg(NES_BLUE));
			if(input[i][a] == 1){
				print!("▦ " );
			}
			if(input[i][a] == 2){
				print!("{}▧ ", color::Fg(NES_WHITE));
			}
			if(input[i][a] == 3){
				print!("{}▨ ", color::Fg(NES_LGT_GREY));
			}
			a+=1;
		}
		println!("");
		i+=1;
		a=0;
	}
    }
    
    if(select == 3){
	while(i <= length){
		while(a <= width){
			let mut choose: usize = rand::thread_rng().gen_range(0, 2);
			print!("{}", color::Fg(nes_palette::NES_GREEN));
			if(input[i][a] == 1){
				print!("▦ " );
			}
			if(input [i][a] != 1 && choose == 1){
				print!("{}▤ ", color::Fg(nes_palette::NES_BROWN));
			}
			if(input [i][a] != 1 && choose == 0){
				print!("{}▥ ", color::Fg(nes_palette::NES_RED));
			}
			a+=1;
		}
		println!("");
		i+=1;
		a=0;
	}
    }
    
     if(select == 4){
		let mut corner: usize = rand::thread_rng().gen_range(0, 4);
		//corner=3; 
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
			}
	}
		 i=0;
		 a=0;
		 
			while(i <= length){
				while(a <= width){
					print!("{}", color::Fg(nes_palette::NES_BLUE));
					if(input[i][a] == 1){
						print!("▦ " );
					}
					if(input[i][a] == 2){
						print!("{}▧ ", color::Fg(nes_palette::NES_WHITE));
					}
					if(input[i][a] == 3){
						print!("{}▨ ", color::Fg(nes_palette::NES_LGT_GREY));
					}
					if(input[i][a] == 0){
						print!("{}▦ ", color::Fg(nes_palette::NES_BLACK));
					}
					a+=1;
				}
				println!("");
				i+=1;
				a=0;
			}
			}
			
		if(select == 5){
		let mut corner: usize = rand::thread_rng().gen_range(0, 4);
		//corner=3; 
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
			}
	}
		 i=0;
		 a=0;
		 
			while(i <= length){
				while(a <= width){
					print!("{}", color::Fg(nes_palette::NES_LGT_GREY));
					if(input[i][a] == 1){
						print!("▦ " );
					}
					if(input[i][a] == 2){
						print!("{}▢ ", color::Fg(nes_palette::NES_MED_GREY));
					}
					if(input[i][a] == 3){
						print!("{}◯ ", color::Fg(nes_palette::NES_DRK_GREY));
					}
					a+=1;
				}
				println!("");
				i+=1;
				a=0;
			}
			}
}

fn mapGen(length: usize, width: usize) ->  [[usize; 32]; 32]{
	let mut a: usize = 0;
    let mut i: usize = 0;
    
    let mut newOut: [[usize; 32]; 32] = [[0; 32]; 32];
	
	 while(i <= length){
		while(a <= width){
			
			if((i < 2 || i >length-2) || (a < 2 || a >width-2)) {
			newOut[i][a] = 1;
			}else{
				if(a%2 == 0 && i%2 ==0){newOut[i][a] = 2;
				}
				if(a%2 != 0 && i%2 ==0){newOut[i][a] = 3
				}
				if(a%2 == 0 && i%2 !=0){newOut[i][a] = 3;
				}
				if(a%2 != 0 && i%2 !=0){newOut[i][a] = 2;
				}
			}
			a+=1;
		}
		i+=1;
		a=0;
	}
	newOut
}
