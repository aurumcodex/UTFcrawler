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
//! This is the `util.rs` file, which contains various constant variables for the application.

#![allow(dead_code)]

extern crate termion;

// TODO: consider renaming file to util.rs, due to the lack of needing game state enums now.


pub const TITLE: &str =  "\r
  :::    :::  ::::::::  :::::::::      :::     :::       ::: :::        :::::::::: :::::::::\r
  :+:    :+: :+:    :+: :+:    :+:   :+: :+:   :+:       :+: :+:        :+:        :+:    :+:\r
  +:+    +:+ +:+        +:+    +:+  +:+   +:+  +:+       +:+ +:+        +:+        +:+    +:+\r
  +#+    +:+ +#+        +#++:++#:  +#++:++#++: +#+  +:+  +#+ +#+        +#++:++#   +#++:++#:\r
  +#+    +#+ +#+        +#+    +#+ +#+     +#+ +#+ +#+#+ +#+ +#+        +#+        +#+    +#+\r
  #+#    #+# #+#    #+# #+#    #+# #+#     #+#  #+#+# #+#+#  #+#        #+#        #+#    #+#\r
   ########   ########  ###    ### ###     ###   ###   ###   ########## ########## ###    ###\r ";

pub mod palettes {
    pub mod nes_palette {
        pub const NES_BLACK: termion::color::Rgb       = termion::color::Rgb(0, 0, 0);
        pub const NES_DRK_GREY: termion::color::Rgb    = termion::color::Rgb(97, 97, 97);
        pub const NES_MED_GREY: termion::color::Rgb    = termion::color::Rgb(124, 124, 124);
        pub const NES_LGT_GREY: termion::color::Rgb    = termion::color::Rgb(188, 188, 188);
        pub const NES_WHITE: termion::color::Rgb       = termion::color::Rgb(248, 248, 248);
        pub const NES_RED: termion::color::Rgb         = termion::color::Rgb(168, 16, 0);
        pub const NES_BRT_RED: termion::color::Rgb     = termion::color::Rgb(248, 120, 88);
        pub const NES_ORANGE: termion::color::Rgb      = termion::color::Rgb(252, 160, 68);
        pub const NES_YELLOW: termion::color::Rgb      = termion::color::Rgb(248, 184, 0);
        pub const NES_GREEN: termion::color::Rgb       = termion::color::Rgb(0, 184, 0);
        pub const NES_BRT_GREEN: termion::color::Rgb   = termion::color::Rgb(184, 248, 24);
        pub const NES_CYAN: termion::color::Rgb        = termion::color::Rgb(0, 232, 216);
        pub const NES_BLUE: termion::color::Rgb        = termion::color::Rgb(0, 120, 248);
        pub const NES_BRT_BLUE: termion::color::Rgb    = termion::color::Rgb(60, 188, 252);
        pub const NES_PURPLE: termion::color::Rgb      = termion::color::Rgb(104, 68, 252);
        pub const NES_BROWN: termion::color::Rgb       = termion::color::Rgb(80, 48, 0);
    }
}

