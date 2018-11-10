# UTFcrawler
This is a project that is designed to create an RPG game using Unicode characters as ASCII art, instead of using raw ASCII.  <br/>
 
![Language](https://img.shields.io/badge/rust-1.30.0-654321.svg?logo=rust&logoColor=rgb(225,225,225)&style=flat-square)
![Current Version](https://img.shields.io/badge/version-0.0.1-519331.svg?style=flat-square)
![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg?style=popout-square&colorB=1a5c86)
![Build Status](https://gitlab.com/utf-crawler/utf-crawler/badges/master/pipeline.svg)

### TODO List:
- main.rs:
    - [ ] initialize the game itself.
    
- combat.rs:
    - [ ] implement the battle flow for the game.
    - [ ] implement the random generator for the encounter rate of enemies.
    - [ ] implement damage calculations.
    - [ ] implement damage resistances and weaknesses.
    - [ ] implement combat turn orders
    - [ ] implement hit rate and evasion calculations
    - [ ] implement stat displays (UI)
    - [ ] implement all various types of skills
    - [ ] possible implementation of difficulty
    
- dungeon.rs
    - [ ] implement the layout of the dungeon
    - [ ] implement the traversal of the dungeon
    - [ ] implement the movement methods
    - [ ] set up saving areas
    - [ ] implement possible mini-map display (UI)
    
- enemy.rs
    - [ ] implement low-level enemies
        - [ ] implement the low-level common enemies
        - [ ] implement the low-level rare enemies
        - [ ] implement the low-level boss enemies
    - [ ] implement mid-level enemies
        - [ ] implement the mid-level common enemies
        - [ ] implement the mid-level rare enemies
        - [ ] implement the mid-level boss enemies
    - [ ] implement high-level enemies
        - [ ] implement the high-level common enemies
        - [ ] implement the high-level rare enemies
        - [ ] implement the high-level boss enemies
    - [ ] implement all enemy weaknesses and/or resistances
    
- player.rs
    - [ ] implement fields for a player character to exist and be controlled
    - [ ] implement a function that handles experience points
    - [ ] implement a function that deals with level ups
    - [ ] declare archetypes for the player (think classes in a traditional RPG)
    
- status.rs
    - [x] declare enums and values for status ailments and potential psyche ailments (partially, more might be added later.)
    
- util.rs
    - [ ] implement saving functionality
    - [ ] implement loading functionality


#### For the team members: 
Recommended reading: [The Rust Programming Language](https://doc.rust-lang.org/stable/book/2018-edition/)  
Recommended IDE: [IntelliJ Community Edition](https://www.jetbrains.com/idea/download/#section=windows)  
Recommended Plugin (for Rust support): [IntelliJ Rust](https://intellij-rust.github.io/)  
