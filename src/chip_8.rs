use core::fmt;
use regex::Regex;
use std::{cmp, fs};

pub const RAM_SIZE: usize = 4096;

// type RamType = [u8; 4096];
#[derive(Copy, Clone)]
struct Ram([u8; RAM_SIZE]);
impl Ram {
    pub fn get(&self, pos: usize) -> u8 {
        self.0[pos]
    }
    pub fn set(&mut self, pos: usize, val: u8) {
        self.0[pos] = val;
    }
}

impl fmt::Display for Ram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const GROUPING: usize = 8;
        let mut pos: usize = 0;
        let mut current_iter = 0;
        writeln!(f, "Ram:")?;
        while pos < RAM_SIZE {
            // covers start case + every GROUPINGth
            if current_iter == 0 || current_iter == GROUPING {
                write!(
                    f,
                    "\n[{:#05X} - {:#05X}]: ",
                    pos,
                    cmp::min(pos + GROUPING - 1, RAM_SIZE - 1)
                )?;
                current_iter = 0;
            }
            write!(f, "{:#04X} ", self.get(pos))?;
            pos += 1;
            current_iter += 1
        }
        Ok(())
    }
}

mod addressing_consts {
    pub const PROGRAM_START: u16 = 0x0200;
    pub const FONT_POS: (u16, u16) = (0x050, 0x09F);
    pub const FONT_SIZE: u8 = 0x05;
}

pub fn init() -> cpu {
    cpu::new()
}

#[allow(non_camel_case_types)]
pub struct cpu {
    ram: Ram,
    pc: u16,  // really 12 bits wide
    idx: u16, // really 12 bits wide
}

impl cpu {
    pub fn new() -> cpu {
        cpu {
            ram: cpu::init_ram(),
            pc: 0x0000,
            idx: 0x0000,
        }
    }

    fn init_ram() -> Ram {
        // put fonts where they belong
        let ram: Ram = Ram([0x00; RAM_SIZE]);
        let ram = match cpu::load_font(ram) {
            Ok(x) => x,
            Err(err) => panic!("{err}"),
        };
        println!("{ram}");
        ram
    }

    fn load_font(ram: Ram) -> Result<Ram, String> {
        let mut ram = ram;
        let font = fs::read_to_string("./resources/font.txt").expect("font could not be loaded");
        let mut ram_pos = addressing_consts::FONT_POS.0;
        let mut letters: Vec<&str> = Regex::new(r"[,]*[  ]* // [0-9a-zA-Z][\n]*")
            .unwrap()
            .split(&font)
            .collect();

        letters.pop();

        for letter in &letters {
            let bytes: Vec<&str> = letter.trim().split(", ").collect();
            for byte in &bytes {
                let without_prefix = byte.trim_start_matches("0x");
                let val = match u8::from_str_radix(without_prefix, 16) {
                    Ok(x) => x,
                    Err(err) => {
                        panic!("Error parsing font: {err}")
                    }
                };

                ram.set(usize::from(ram_pos), val);
                ram_pos += 1;
            }
        }
        if (ram_pos - 1) != addressing_consts::FONT_POS.1 {
            return Err(format!(
                "Incorrect number of bytes read. Desired range: [{} - {}], Read range: [{} - {}]",
                addressing_consts::FONT_POS.0,
                addressing_consts::FONT_POS.1,
                addressing_consts::FONT_POS.0,
                ram_pos - 1
            ));
        }
        Ok(ram)
    }
}
