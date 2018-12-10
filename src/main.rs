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
//! This is the `main.rs` file, which runs this application in its entirety.

#![allow(dead_code)]
#![allow(warnings)]     //hides the 400+ warnigns about the usage of camel case

extern crate rand;     //import for rng libraries
extern crate termion;   //import for the terminal manager. it allows full rgb support and easy clearing and buffering 

mod player;
mod enemy;
mod status;   //the modules which are used in this file. each is its own separate file 
mod combat;
mod dungeon;
mod inventory;
mod util;

use rand::Rng;
use termion::{color, cursor, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;

use self::player::*;
use self::enemy::*;
use self::status::*;
use self::dungeon::*;           //imports for the parts of the modules similar to java imports for libraries and classes
use self::inventory::*;
use self::util::TITLE;
use self::util::palettes::*;

use std::io;
use std::io::{Read, Write, stdout, stdin};

// ---------------------------------------------------------- //

/// The main game loop. Controls most everything in the program.
fn main() {//main function
    let mut length: usize = rand::thread_rng().gen_range(5, 26);
    let mut width: usize = rand::thread_rng().gen_range(5, 26);     //length width and type for the rooms to be created
    let mut Type: usize = rand::thread_rng().gen_range(1, 6);
    let mut run: usize = 1;  //run variable for the main loop. setting it to 0 stop the game
    
    let mut encounter: usize = 0;  //is later set to random number used to create random encounters
    let mut boss = 0;   //boss toggle. when 1 the + shaped boss rooms are created 
    let mut count = 0;   //room count
    let mut floor = 0;    //floor count. room generation on a new 'floor' is unchanged. floors are representative of difficulty
    let mut win: bool = false;   //stops the game if set to true
    
    let mut playerX: usize = 0;    //coordinates of player character
	let mut playerY: usize = 0;
	
	let mut demo: usize = 49;     //advanges roop count for demo mode
	let mut demoRoom: usize = 1;   //sets starting room for demo
    
    let stdin = stdin();     //initializing the statdard imput c style
    let stdout = stdout().into_raw_mode().unwrap();    //doing the same for stadard output
    //let mut stdout = stdout.lock();
    
    let mut baddy;    //enemy variable. can be common or boss
    
	let mut byteInput = stdin.bytes();        //input from std in. its take bytewise so each button is taken as an umput. necessary for raw mode. this makes it so you dont have to hit enter after pressig a key
	
	let mut mainMap = createMap(length, width, Type);    //creates the main map using its constructer
	
	let mut i: usize = 0;      //itterators for pretty much all of the nested while loops that interact with the 2d map array
	let mut j: usize = 0;
	while(i < 32){
		while(j < 32){
			//print!("{}",mainMap.output[i][j]);  //debug junk
			if(mainMap.output[i][j]	== 5){
				playerX = i;
				playerY = j;			      //nested while loops that check every array index for the cplayer character indicator and set the palyers coordinates to it	
			}                                //this is done because the player can start at different position on each map and the startig position is placed by the map creaton algorithm.
			j += 1;
		}
		i+=1;
		j = 0;
		//println!("");
	}
	let mut player = Player::new(String::from("butch"),Archetype::Generalist);       //basic initialization for the player 
	println!("{}", color::Fg(nes_palette::NES_BRT_GREEN));                           //setting text color
        
        println!("{}", clear::All);
	println!("{}",TITLE); //title logo 
	print!("{}", color::Fg(nes_palette::NES_YELLOW)); //setti color
	println!("press 1 to select Alchemist\r");
	println!("press 2 to select Blackguard\r");  //information telling you what buttons select characters
	println!("press 3 to select Generalist\r");
	println!("press 4 to select Gunner\r");
	println!("press 5 to select Mercinary\r");
	println!("press 6 to start a demo dungeon\r");
	
	//boss = 1;
	let mut goo: usize = 0;   //variable for determinitn gif a player takes damage from floor goo in green rooms
	let mut enemyType: u8 = 0;  //container for enemy type selection
	let mut choose: u8 = 1;   //togle for character selection. when 0, the game starts
	
	let mut inv = init();      //initializing the inventory with its constructor   
	let mut mode: usize = 0;    //toggle for inventory mode. when 1 inventory is active
	let mut invPos: usize = 1;  //highlight position for the selected item in inventory
	
	while(choose == 1){   //cahracter selection loop
	let input = byteInput.next().unwrap().unwrap();	  //input variable
		match input{   //big match statement for selectig character
			b'1' =>{//match for button press
				player = Player::new(String::from("Annwyn"),Archetype::Alchemist); //sets player variable to new characater with chosen architype
				println!("you chose Annwyn, the Alchemist\r");  //print character name
						choose = 0;//end selection loop
						println!("Press any key to continue\r"); //an imput is needed to clear the display and print the map. dont hit q. it will just start an ended game			
			},
			b'2' =>{
				player = Player::new(String::from("Skipp"),Archetype::Blackguard);   //all of these are funtionaly identicall
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
			b'6' =>{
				player = Player::new(String::from("Demo"),Archetype::Generalist);
				println!("you chose Demo, the Generalist\r");
						demo = 5;
						choose = 0;
						println!("Press any key to continue\r");
			},
			_ => {
				println!("Invalid entry\r");  //print for nonmatching character
				},
		}	

	}
	
		if(demo != 49){   //selection for demo game
			Type = demoRoom; //starts on the first room will be itterated to sequentially show each room type
			mainMap = createMap(length, width, Type);  //creates that eoom 
		}
	
	while(run == 1 && win == false){ //M A I N  G A M E  L O O P
		i = 0;
		j = 0;    //resetting loop itterators
		
		if(count >= demo){
			boss = 1;	  //selcts boss room when all of the normal rooms have been entered
		}
		goo = rand::thread_rng().gen_range(0, 3);   //1 in 3 chance of taking one damage from floor goo
		if(Type == 3 && goo == 1){
			player.hp -= 1;  //removes one of player's hp
		}
		
		let input = byteInput.next().unwrap().unwrap();	//new byte input in this scope	
	
		if(demo != 49){
			Type = demoRoom;  //making new demo room in this scope
		}
		
		encounter = rand::thread_rng().gen_range(0, 100);  //genearating percentage for player to encounter an enemy
		println!("{}", clear::All);  //clearing terminal
		if(encounter >= 1 && encounter < floor + 1){  //calcualting likelihood of encounter. the higher the floor number the more likely you are to encounter something
			enemyType = rand::thread_rng().gen_range(1, 255);  //randomly choosing enemy. starting at 1 so you cant get the boss as a normal enemy  
			if(Type == 8){  //roomtype 8 is the boss arena. if that is elected, the boss is the next encounter
			baddy = Enemy::new(enemy::EnemyType::Boss, 0);  
			win = combat::combat(&mut player, &mut baddy, Type);  //combat terurns a boolean for beating the boss. if you beat it, win is set to true and the game ends
			}else{
			baddy = Enemy::new(enemy::EnemyType::Common, enemyType); //common enemy creation for normal rooms
			combat::combat(&mut player, &mut baddy, Type);  //enterig normal combat
			}
		}
		if(player.hp <= 0){  //ends the game if the player's healh goes to 0
			Type = 6;  //room type 6 is an anemated death splash
			mainMap = createMap(length, width, Type); //creating that map
			println!("{}", clear::All); //clearing terminal
				run = 0;//ending game
		}	
		match input{ //big match statement for every bitton that you can press in the game
		
		b'q' =>
			run = 0,    //q stops the game. 'quit'
			
		b'i' =>{ //enters inventory
			println!("{}", clear::All);  //clearing terminal
			list(&mut inv, invPos);  //calling list function from inventory module
			mode = 1;   //entering inventory mode
		}
		b'j' =>{  //move to previous index in inventory
			if(invPos <= 15){invPos+=1;}  //itterating inventory position varaible
			println!("{}", clear::All); //clearing terminal 
			list(&mut inv, invPos); //recalling list to show new highlight position
		}
		b'k' =>{  //move to previous index in inventory
			if(invPos >0){invPos-=1;}
			println!("{}", clear::All);  //same as before
			list(&mut inv, invPos);
		}
		b'u' =>{  //use selected inventory
			if(mode == 1){useItem(invPos, &mut inv, &mut player); //calling use item function in inventory module
				list(&mut inv, invPos);  //calling list function from inventory module
				} 
		}
		b'm' =>{ //going to map mode
			println!("{}", clear::All); //clearing terminal
			mode = 0; //setting mode back to map
		}
			
		
		
		b'a' =>{ //movement key. all four movement key are near identiacal. the only real difference is adding or sutracting from the map positions differently (which makes it difficult to abstract it further)
			if(mainMap.output[playerX][playerY-1] == 2 || mainMap.output[playerX][playerY-1] > 5){
				mainMap.output[playerX][playerY] = 3;   //detects the tile adjacent tto the player and sets it to ne opposite one to keep the floor looking right
			}
			if(mainMap.output[playerX][playerY-1] == 3 || mainMap.output[playerX][playerY-1] > 5){ //same as above
				mainMap.output[playerX][playerY] = 2;
			}
			if(mainMap.output[playerX][playerY-1]!=1){ //checks to see if the position that is selected to move to is a wall. if its not you can move
			playerY -=1; //moves player position
			}
			if(mainMap.output[playerX][playerY] == 4){ //detects is the current position is a door. if it is, thigs happen
				if(boss == 0){      //if youre not in a boss area, it creates new random maps
				Type = rand::thread_rng().gen_range(1, 6); //random map type
				length = rand::thread_rng().gen_range(5, 26);  //dimension of new room
				width = rand::thread_rng().gen_range(5, 26);
				if(demo != 49){  //when demo mode is activated it creates room s big enough to create 'L' shaped rooms
					Type = demoRoom;
					floor = 6;
					player.level = 6;  //setting player level to 6 so you can use most weapons but not all
					length = rand::thread_rng().gen_range(15, 26);  //modified room sizes
					width = rand::thread_rng().gen_range(15, 26);
				}
				if(count % 5 == 0){   //makes a new floor every 5 rooms
					floor += 1;	 //itterates floor count
				}
				}else if(count >=  54 || demoRoom >= 8){ //sets room to boss room after youve gone through a specific number of rooms 
					Type = 8;	
					length = 25;   //type and size of final boss room
					width = 25;
				}else{ //if the room number isnt high enough it creates the + shaped pre boss rooms 
					Type = 7;
					length = 25; //size adn type of pre boss rooms 
					width = 25;
				}
				//mainMap.output[playerX][playerY] = 2; 
				mainMap = createMap(length, width, Type);  //creating the new map
				count += 1;  //addign to room count  
				i=0; //resetting itterator
				while(i < 32){
				while(j < 32){   //relocating player position in egards to where it has jsut been adjsuted on the map
					if(mainMap.output[i][j]	== 5){
						playerX = i;
						playerY = j;				
					}
					j += 1;
				}
				i+=1;
				j = 0;

				}
				demoRoom += 1;  //next room in demo mode
			}
			if(mainMap.output[playerX][playerY] > 5){
				//println!("{}",getDesc(mainMap.output[playerX][playerY],&mut inv));
				storeItem(mainMap.output[playerX][playerY], &mut inv);
			}
			mainMap.output[playerX][playerY] = 5; 
			
		},
		b'w' =>{  //w, s, and d are all structually identical to the above. the difference is added or subtracted values for player position
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
				if(demo != 49){
					Type = demoRoom;
					floor = 6;
					player.level = 6;
					length = rand::thread_rng().gen_range(15, 26);
					width = rand::thread_rng().gen_range(15, 26);
				}
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54 || demoRoom >= 8){
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
				demoRoom += 1;
			}
			if(mainMap.output[playerX][playerY] > 5){
				//println!("{}",mainMap.output[playerX][playerY]);
				storeItem(mainMap.output[playerX][playerY], &mut inv);
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
				if(demo != 49){
					Type = demoRoom;
					floor = 6;
					player.level = 6;
					length = rand::thread_rng().gen_range(15, 26);
					width = rand::thread_rng().gen_range(15, 26);
				}
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54 || demoRoom >= 8){
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
				demoRoom += 1;
			}
			if(mainMap.output[playerX][playerY] > 5){
				//println!("{}",mainMap.output[playerX][playerY]);
				storeItem(mainMap.output[playerX][playerY], &mut inv);
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
				if(demo != 49){
					Type = demoRoom;
					floor = 6;
					player.level = 6;
					length = rand::thread_rng().gen_range(15, 26);
					width = rand::thread_rng().gen_range(15, 26);
				}
				if(count % 5 == 0){
					floor += 1;	
				}
				}else if(count >=  54 || demoRoom >= 8){
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
				demoRoom += 1;
			}
			if(mainMap.output[playerX][playerY] > 5){
				//println!("{}",mainMap.output[playerX][playerY]);
				storeItem(mainMap.output[playerX][playerY], &mut inv);
			}
			mainMap.output[playerX][playerY] = 5; 
		},
		_=> {},
	}
		if(win == false){ //if the game is still going, the things are printed
		print!("room: "); 
		print!("{}", count+1); //prints what room you're in
		print!("					Level: "); 
		println!("{}\r", player.level);   //prints your level 
		print!("Floor: ");
		print!("{}", floor);  //prints the current floor
		print!("				{}'s will to live: ", player.player_name);  //prints the health of the chosen character
		println!("{}\r", player.hp);
		//print!("{}", demoRoom);
		if(mode == 0) {printMap(mainMap, length, width);}
		//println!("{}", input);
		//input = "".to_string();
		}
		
	}
}

