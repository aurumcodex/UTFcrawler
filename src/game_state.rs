extern crate ansi_term;
//extern crate termion;

pub mod palettes {
    pub mod nes_palette {
        // ansi_term color variables [will be replaced, or least commented out when swapping to termion]
        pub const NES_BLACK: ansi_term::Color       = ansi_term::Color::RGB(0, 0, 0);
        pub const NES_DRK_GREY: ansi_term::Color    = ansi_term::Color::RGB(97, 97, 97);
        pub const NES_MED_GREY: ansi_term::Color    = ansi_term::Color::RGB(124, 124, 124);
        pub const NES_LGT_GREY: ansi_term::Color    = ansi_term::Color::RGB(188, 188, 188);
        pub const NES_WHITE: ansi_term::Color       = ansi_term::Color::RGB(248, 248, 248);
        pub const NES_RED: ansi_term::Color         = ansi_term::Color::RGB(168, 16, 0);
        pub const NES_BRT_RED: ansi_term::Color     = ansi_term::Color::RGB(248, 120, 88);
        pub const NES_ORANGE: ansi_term::Color      = ansi_term::Color::RGB(228, 92, 16);
        pub const NES_YELLOW: ansi_term::Color      = ansi_term::Color::RGB(248, 184, 0);
        pub const NES_GREEN: ansi_term::Color       = ansi_term::Color::RGB(0, 184, 0);
        pub const NES_BRT_GREEN: ansi_term::Color   = ansi_term::Color::RGB(184, 248, 24);
        pub const NES_CYAN: ansi_term::Color        = ansi_term::Color::RGB(0, 232, 216);
        pub const NES_BLUE: ansi_term::Color        = ansi_term::Color::RGB(0, 120, 248);
        pub const NES_BRT_BLUE: ansi_term::Color    = ansi_term::Color::RGB(60, 188, 252);
        pub const NES_PURPLE: ansi_term::Color      = ansi_term::Color::RGB(104, 68, 252);
        pub const NES_BROWN: ansi_term::Color       = ansi_term::Color::RGB(80, 48, 0);
    }

    pub mod c64_palette {
        // ansi_term color variables [will be replaced, or least commented out when swapping to termion]
        pub const C64_BLACK: ansi_term::Color       = ansi_term::Color::RGB(0, 0, 0);
        pub const C64_WHITE: ansi_term::Color       = ansi_term::Color::RGB(255, 255, 255);
        pub const C64_RED: ansi_term::Color         = ansi_term::Color::RGB(137, 64, 54);
        pub const C64_CYAN: ansi_term::Color        = ansi_term::Color::RGB(122, 191, 199);
        pub const C64_PURPLE: ansi_term::Color      = ansi_term::Color::RGB(138, 70, 174);
        pub const C64_GREEN: ansi_term::Color       = ansi_term::Color::RGB(104, 169, 65);
        pub const C64_BLUE: ansi_term::Color        = ansi_term::Color::RGB(62, 49, 162);
        pub const C64_YELLOW: ansi_term::Color      = ansi_term::Color::RGB(208, 220, 113);
        pub const C64_ORANGE: ansi_term::Color      = ansi_term::Color::RGB(144, 95, 37);
        pub const C64_BROWN: ansi_term::Color       = ansi_term::Color::RGB(92, 71, 0);
        pub const C64_LGT_RED: ansi_term::Color     = ansi_term::Color::RGB(187, 119, 109);
        pub const C64_DRK_GREY: ansi_term::Color    = ansi_term::Color::RGB(85, 85, 85);
        pub const C64_MED_GREY: ansi_term::Color    = ansi_term::Color::RGB(128, 128, 128);
        pub const C64_LGT_GREEN: ansi_term::Color   = ansi_term::Color::RGB(172, 234, 136);
        pub const C64_LGT_BLUE: ansi_term::Color    = ansi_term::Color::RGB(124, 112, 218);
        pub const C64_LGT_GREY: ansi_term::Color    = ansi_term::Color::RGB(171, 171, 171);
    }

    pub mod atari_palette {
        // ansi_term color variables [will be replaced, or least commented out when swapping to termion]
        pub const ATARI_BLACK: ansi_term::Color     = ansi_term::Color::RGB(0, 0, 0);
        pub const ATARI_DRK_GREY: ansi_term::Color  = ansi_term::Color::RGB(108, 108, 108);
        pub const ATARI_LGT_GREY: ansi_term::Color  = ansi_term::Color::RGB(200, 200, 200);
        pub const ATARI_WHITE: ansi_term::Color     = ansi_term::Color::RGB(255, 255, 255);
        pub const ATARI_YELLOW: ansi_term::Color    = ansi_term::Color::RGB(160, 160, 52);
        pub const ATARI_ORANGE: ansi_term::Color    = ansi_term::Color::RGB(176, 104, 72);
        pub const ATARI_RED: ansi_term::Color       = ansi_term::Color::RGB(176, 60, 60);
        pub const ATARI_RED2: ansi_term::Color      = ansi_term::Color::RGB(224, 136, 136);
        pub const ATARI_PURPLE: ansi_term::Color    = ansi_term::Color::RGB(140, 88, 184);
        pub const ATARI_BLUE: ansi_term::Color      = ansi_term::Color::RGB(80, 112, 188);
        pub const ATARI_BLUE2: ansi_term::Color     = ansi_term::Color::RGB(104, 156, 192);
        pub const ATARI_TEAL: ansi_term::Color      = ansi_term::Color::RGB(104, 180, 148);
        pub const ATARI_GREEN: ansi_term::Color     = ansi_term::Color::RGB(116, 180, 116);
        pub const ATARI_GREEN2: ansi_term::Color    = ansi_term::Color::RGB(108, 152, 80);
        pub const ATARI_OLIVE: ansi_term::Color     = ansi_term::Color::RGB(132, 140, 76);
        pub const ATARI_BROWN: ansi_term::Color     = ansi_term::Color::RGB(132, 104, 48);
    }

    pub mod default_palette {
        // ansi_term color variables [will be replaced, or least commented out when swapping to termion]
        pub const DFLT_BLACK: ansi_term::Color      = ansi_term::Color::RGB(39, 42, 46);
        pub const DFLT_WHITE: ansi_term::Color      = ansi_term::Color::RGB(244, 245, 247);
        pub const DFLT_RED: ansi_term::Color        = ansi_term::Color::RGB(152, 0, 23);
        pub const DFLT_CYAN: ansi_term::Color       = ansi_term::Color::RGB(30, 239, 193);
    }
}

#[derive(Debug)]
pub enum Palette {
    NES, C64, ATARI,
}

#[derive(Debug)]
pub enum Floor {
    Floor1, Floor2, Floor3, Floor4, Floor5,
    Floor6, Floor7, Floor8, Floor9, Floor10,
    Floor11, Floor12, Floor13, Floor14, Floor15,
}