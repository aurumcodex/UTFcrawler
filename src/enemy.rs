//!
//! Authors: Nathan Adams, Reid Marsh, Nicolas Sanderson, Pressy Muraguri
//!

extern crate rand;

use crate::status::{Ailment, Psyche};
use rand::Rng;

#[derive(Debug)]
pub enum EnemyType {
    Common,
    Rare,
    Boss,
}

//pub enum EnemyLevel

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub enemy_id: u8,
    pub enemy_name: String,
    pub level: u8,

}

impl Enemy {
    pub fn new(foe_type: EnemyType, id: u8) {
        match foe_type {
            // TODO: implement a default construction of enemies so that they can be properly created
            // and set up.
            EnemyType::Common => (),
            EnemyType::Rare => (),
            EnemyType::Boss => ()
        }
    }
}