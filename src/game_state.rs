#![allow(dead_code)]

//extern crate ansi_term;
extern crate termion;

use termion::color::{DetectColors, AnsiValue, Bg};
use termion::raw::IntoRawMode;

use std::io::stdout;

pub const TITLE: &str = "
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
        pub const NES_ORANGE: termion::color::Rgb      = termion::color::Rgb(228, 92, 16);
        pub const NES_YELLOW: termion::color::Rgb      = termion::color::Rgb(248, 184, 0);
        pub const NES_GREEN: termion::color::Rgb       = termion::color::Rgb(0, 184, 0);
        pub const NES_BRT_GREEN: termion::color::Rgb   = termion::color::Rgb(184, 248, 24);
        pub const NES_CYAN: termion::color::Rgb        = termion::color::Rgb(0, 232, 216);
        pub const NES_BLUE: termion::color::Rgb        = termion::color::Rgb(0, 120, 248);
        pub const NES_BRT_BLUE: termion::color::Rgb    = termion::color::Rgb(60, 188, 252);
        pub const NES_PURPLE: termion::color::Rgb      = termion::color::Rgb(104, 68, 252);
        pub const NES_BROWN: termion::color::Rgb       = termion::color::Rgb(80, 48, 0);
    }

    // pub mod c64_palette {
    //     pub const C64_BLACK: termion::color::Rgb       = termion::color::Rgb(0, 0, 0);
    //     pub const C64_WHITE: termion::color::Rgb       = termion::color::Rgb(255, 255, 255);
    //     pub const C64_RED: termion::color::Rgb         = termion::color::Rgb(137, 64, 54);
    //     pub const C64_CYAN: termion::color::Rgb        = termion::color::Rgb(122, 191, 199);
    //     pub const C64_PURPLE: termion::color::Rgb      = termion::color::Rgb(138, 70, 174);
    //     pub const C64_GREEN: termion::color::Rgb       = termion::color::Rgb(104, 169, 65);
    //     pub const C64_BLUE: termion::color::Rgb        = termion::color::Rgb(62, 49, 162);
    //     pub const C64_YELLOW: termion::color::Rgb      = termion::color::Rgb(208, 220, 113);
    //     pub const C64_ORANGE: termion::color::Rgb      = termion::color::Rgb(144, 95, 37);
    //     pub const C64_BROWN: termion::color::Rgb       = termion::color::Rgb(92, 71, 0);
    //     pub const C64_LGT_RED: termion::color::Rgb     = termion::color::Rgb(187, 119, 109);
    //     pub const C64_DRK_GREY: termion::color::Rgb    = termion::color::Rgb(85, 85, 85);
    //     pub const C64_MED_GREY: termion::color::Rgb    = termion::color::Rgb(128, 128, 128);
    //     pub const C64_LGT_GREEN: termion::color::Rgb   = termion::color::Rgb(172, 234, 136);
    //     pub const C64_LGT_BLUE: termion::color::Rgb    = termion::color::Rgb(124, 112, 218);
    //     pub const C64_LGT_GREY: termion::color::Rgb    = termion::color::Rgb(171, 171, 171);
    // }

//    pub mod default_palette {
//        pub const DFLT_BLACK: termion::color::Black              = termion::color::Black;
//        pub const DFLT_RED: termion::color::Red                  = termion::color::Red;
//        pub const DFLT_GREEN: termion::color::Green              = termion::color::Green;
//        pub const DFLT_YELLOW: termion::color::Yellow            = termion::color::Yellow;
//        pub const DFLT_BLUE: termion::color::Blue                = termion::color::Blue;
//        pub const DFLT_MAGENTA: termion::color::Magenta          = termion::color::Magenta;
//        pub const DFLT_CYAN: termion::color::Cyan                = termion::color::Cyan;
//        pub const DFLT_WHITE: termion::color::White              = termion::color::White;
//        pub const DFLT_LGT_BLACK: termion::color::LightBlack     = termion::color::LightBlack;
//        pub const DFLT_LGT_RED: termion::color::LightRed         = termion::color::LightRed;
//        pub const DFLT_LGT_GREEN: termion::color::LightGreen     = termion::color::LightGreen;
//        pub const DFLT_LGT_YELLOW: termion::color::LightYellow   = termion::color::LightYellow;
//        pub const DFLT_LGT_BLUE: termion::color::LightBlue       = termion::color::LightBlue;
//        pub const DFLT_LGT_MAGENTA: termion::color::LightMagenta = termion::color::LightMagenta;
//        pub const DFLT_LGT_CYAN: termion::color::LightCyan       = termion::color::LightCyan;
//        pub const DFLT_LGT_WHITE: termion::color::LightWhite     = termion::color::LightWhite;
//    }
}

#[derive(Debug)]
pub enum Palette {
    NES, C64, DEFAULT,
}

#[derive(Debug)]
pub enum Floor {
    GF, B1F, B2F, B3F, B4F,
}

#[derive(Debug)]
pub enum GameState {
    Title, InGame, Combat, Menu,
}

pub struct Completion {}

//pub struct Score {
//    pub score: u64,
//    pub hi_score: u64,
//}

pub fn check_colors() {
    let mut window = stdout().into_raw_mode().unwrap();
    let color_count = window.available_colors().unwrap();

    println!("this terminal window supports {} colors.", color_count);
    for i in 0..color_count {
        print!("{} {}", Bg(AnsiValue(i as u8)), Bg(AnsiValue(0)));
    }
    println!("{}\r\n\n", Bg(termion::color::Reset));
}
