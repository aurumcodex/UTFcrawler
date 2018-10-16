# UTFcrawler
This is a project that is designed to create an RPG game using Unicode characters as ASCII art, instead of using raw ASCII.  
![GitHub repo size in bytes](https://img.shields.io/github/repo-size/badges/shields.svg) 
![Current Version](https://img.shields.io/badge/version-0.0.1-blue.svg)

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
    - [ ] declare enums and values for status ailments and potential psyche ailments
    
- util.rs
    - [ ] implement saving functionality
    - [ ] implement loading functionality


#### For the team members: 
Recommended reading: [The Rust Programming Language](https://doc.rust-lang.org/stable/book/2018-edition/)  
Recommended IDE: [IntelliJ Community Edition](https://www.jetbrains.com/idea/download/#section=windows)  
Recommended Plugin (for Rust support): [IntelliJ Rust](https://intellij-rust.github.io/)  
