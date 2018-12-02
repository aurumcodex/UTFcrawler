#![allow(dead_code)]

//extern crate ansi_term;
extern crate termion;

// use termion::color::{DetectColors, AnsiValue, Bg};
// use std::io::stdout;

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
        // pub const NES_ORANGE: termion::color::Rgb      = termion::color::Rgb(228, 92, 16);
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

// #[derive(Debug)]
// pub enum GameState {
//     Title, InGame, Combat, Menu,
// }

//pub struct Score {
//    pub score: u64,
//    pub hi_score: u64,
//}

// pub fn check_colors() {
//     let mut window = stdout().into_raw_mode().unwrap();
//     let color_count = window.available_colors().unwrap();
//
//     println!("this terminal window supports {} colors.", color_count);
//     for i in 0..color_count {
//         print!("{} {}", Bg(AnsiValue(i as u8)), Bg(AnsiValue(0)));
//     }
//     println!("{}\r\n\n", Bg(termion::color::Reset));
// }
