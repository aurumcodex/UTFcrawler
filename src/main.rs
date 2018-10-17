//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
extern crate rand;

mod player;
mod status;

use std::io;
use self::player::*;
use self::status::{Ailment, Psyche};
use rand::Rng;

fn main() {
    // TODO: create all the functions and data types that we'll need to use.
    // that comes later, though.
    // additional like in combat branch
    println!("Hello, world!");


    println!("enter player's name:");
    let mut plyr_name: String = String::new();
    io::stdin().read_line(&mut plyr_name).expect("failed to read line");
    let mut test_player: Player = Player::new(plyr_name);
    test_player.print_stats();
    test_player.incr_exp(rand::thread_rng().gen_range(5, 10));
    println!("player EXP is now {}", &test_player.exp);

    
}
