//!
//!  Authors: Nathan Adams, Reid Marsh, Nicholas Sandserson, Pressy Muraguri
//!

use crate::player::*;
use crate::enemy::*;

pub fn combat(player: &mut Player, enemy: &mut Enemy) {
    if player.is_dead != true && enemy.is_dead != true {
        if player.agility >= enemy.agility {
            loop {
//                player.get_combat_input();
//                player.use_skill();
//                //enemy.take_dmg(skill_dmg)
//                enemy.check_status();
//                //enemy.gen_attk();
//                enemy.use_skill();
//                //player.take_dmg(skill_dmg)
//                player.check_status();
            }
        }// if player.agility >+ enemy.agility
        if enemy.agility > player.agility {
            loop {
//                enemy.gen_attk();
//                enemy.use_skill();
//                //player.take_dmg(0);
//                player.check_status();
//                player.get_combat_input();
//                player.use_skill();
//                //enemy.take_dmg();
//                enemy.check_status();
            }
        }// if enemy.agility > player.agility
    }
}