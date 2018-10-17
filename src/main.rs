//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!
extern crate rand;

mod player;
mod status;

use self::player::*;
use self::status::{Ailment, Psyche};
use rand::Rng;

fn main() {
    // TODO: create all the functions and data types that we'll need to use.
    // that comes later, though.
    // additional like in combat branch
    println!("Hello, world!");

    // let mut test_player = Player {
        // player_name: String::from(""),
        // level: 1,
        // exp: 0,
        // archetype: player::Archetype::Gunner,
        // hp: rand::thread_rng().gen_range(20, 50),
        // mp: rand::thread_rng().gen_range(20, 50),
        // phys_attk: rand::thread_rng().gen_range(20, 50),
        // phys_def: rand::thread_rng().gen_range(20, 50),
        // mag_attk: rand::thread_rng().gen_range(20, 50),
        // mag_def: rand::thread_rng().gen_range(20, 50),
        // strength: rand::thread_rng().gen_range(20, 50),
        // magic: rand::thread_rng().gen_range(20, 50),
        // vitality: rand::thread_rng().gen_range(20, 50),
        // dexterity: rand::thread_rng().gen_range(20, 50),
        // agility: rand::thread_rng().gen_range(20, 50),
        // luck: rand::thread_rng().gen_range(20, 50),
        // status: Ailment::Normal,
        // psyche: Psyche::Normal,
    // };
// 
    // println!("enter player name:: ");
    // let mut input_name = String::new();
    // let mut test_player =

    test_player.print_stats();
}
