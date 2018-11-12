//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!

#![allow(dead_code)]

use crate::player::*;
use crate::enemy::*;
use crate::game_state::Score;



pub fn combat(player: &mut Player, enemy: &mut Enemy) {
    while player.is_dead != true && enemy.is_dead != true {
        enemy.decr_hp(3);
    }
    if player.is_dead == true {
        println!("you have died");
    }
    if enemy.is_dead == true {
        println!("enemy status is  now: {:?}", enemy.status);
        player.gain_exp(enemy.given_exp);
        player.check_level_up();
    }
}