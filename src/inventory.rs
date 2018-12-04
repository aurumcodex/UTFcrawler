
extern crate termion;

use player::*;
use game_state::palettes::*;
use termion::{color, cursor};

pub fn init() -> [usize; 15] {
	let mut inv: [usize; 15] = [0; 15];
	inv
}
fn testFill(inv: &mut [usize]) {
	inv[2] = 2;
	inv[3] = 1;
	inv[10] = 3;
	inv[14] = 1;
}
pub fn list(inv: &mut [usize; 15], select: usize) {
	println!("Your inventory currently contains:\r");
	for x in 0..15 {
		print!("{}", color::Fg(nes_palette::NES_LGT_GREY));
		if(x == select){print!("{}", color::Fg(nes_palette::NES_YELLOW));}
		println!("{}\r", getDesc(x, inv));
		print!("{}", color::Fg(nes_palette::NES_LGT_GREY));
	}
}
pub fn storeItem(item: usize, inv: &mut [usize; 15]) {
    let mut i: usize = 0;
    while(i < 15 && inv[i] != 0){ i+=1;}
    if(i > 14){ println!("Your inventory is full! Cannot pick up item.");}
    else{
        inv[i] = item;
        print!("Stored the ");
        print!("{}",getDesc(item, inv));
        println!(" in inventory\r");
    }
}
pub fn dropItem(index: usize, inv: &mut [usize; 15]) {
    if(index > 15 || index < 0) {println!("Invalid index!\r");}
    else if(inv[index] == 0){ println!("There is no item there!\r");}
    else{
        inv[index] = 0;
        println!("Item dropped.\r");
    }
}
pub fn useItem(index: usize, inv: &mut [usize; 15], player: &mut Player) {
    if(index > 15 || index < 0){ println!("Invalid index!\r");}
    match inv[index] {
   	6 => { println!("The edges are sharp\r"); 
		player.hp +=5;
		inv[index] = 0;
		compress(inv);
		},
	7 => { println!("It's cherry flavored!\r"); 
		player.hp += 15;
		inv[index] = 0;
		compress(inv);
		},
    8 => { println!("You place the ring on your finger.\r"); 
		player.strength +=1;
		inv[index] = 0;
		compress(inv);
		},
    9 => { println!("You break your tooth.\r");
		player.hp -=1;
		inv[index] = 0;
		compress(inv);
		},
    _ => {println!("There is no item at that index.\r");
		
		}
    }
}
pub fn compress(inv: &mut [usize; 15]) {
	let mut current;
    for x in 0..14 {
        current = inv[x];
        if(current == 0 && inv[x+1] != 0){
            inv[x] = inv[x+1];
            inv[x+1] = 0;
        }
    }
}
pub fn getDesc(index: usize, inv: &mut [usize; 15]) -> String {
    let mut desc: String = "---".to_string();
    match inv[index] {
			6 => { desc = "Cyan triangle".to_string(); }
			7 => { desc = "health potion".to_string(); },
			8 => { desc = "scrap pile".to_string(); },
			9 => { desc = "rock".to_string(); },
			_ => { desc = "_".to_string(); },
		}
		desc
}


