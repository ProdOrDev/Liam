//! The implementation of the Game Boy's Sharp SM83 processor.
//!
//! # References
//!
//! - <https://izik1.github.io/gbops/>
//! - <https://gekkio.fi/files/gb-docs/gbctr.pdf>
//! - <https://gbdev.io/pandocs/CPU_Instruction_Set>
//! - <http://iceboy.a-singer.de/doc/mem_patterns.html>
//! - <https://www.copetti.org/writings/consoles/game-boy/#cpu>
//! - <https://gist.github.com/SonoSooS/c0055300670d678b5ae8433e20bea595>
//! - <http://www.bitsavers.org/components/sharp/_dataBooks/1996_Sharp_Microcomputer_Data_Book.pdf>

mod registers;

pub use registers::{Flags, Registers};
