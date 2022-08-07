use super::memory::Ram;

use regex::Regex;
use std::fs;

mod addressing_consts {
    pub const PROGRAM_START: u16 = 0x0200;
    pub const FONT_POS: (u16, u16) = (0x050, 0x09F);
    //pub const FONT_SIZE: u8 = 0x05;
}

pub fn init() -> cpu {
    cpu::new()
}

#[allow(non_camel_case_types)]
pub struct cpu {
    ram: Ram,
    stack: Vec<u16>,
    pc: u16,  // really 12 bits wide
    idx: u16, // really 12 bits wide
}

impl cpu {
    pub fn new() -> cpu {
        cpu {
            ram: cpu::init_ram(),
            stack: vec![],
            pc: 0x0000,
            idx: 0x0000,
        }
    }

    fn init_ram() -> Ram {
        // put fonts where they belong
        let ram: Ram = Ram::new();
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
        let letters: Vec<&str> = {
            let mut letters: Vec<&str> = Regex::new(r"[,]*[  ]* // [0-9a-zA-Z][\n]*")
                .unwrap()
                .split(&font)
                .collect();
            letters.pop();
            letters
        };

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
