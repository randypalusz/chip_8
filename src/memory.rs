use core::fmt;
use std::cmp;
pub const RAM_SIZE: usize = 4096;

// type RamType = [u8; 4096];
#[derive(Copy, Clone)]
pub struct Ram(pub [u8; RAM_SIZE] /* public for testing purposes, otherwise use Ram::new()*/);

impl Ram {
    pub fn new() -> Ram {
        Ram([0x00; RAM_SIZE])
    }
    pub fn get(&self, pos: usize) -> u8 {
        self.0[pos]
    }
    pub fn set(&mut self, pos: usize, val: u8) {
        self.0[pos] = val;
    }
    pub fn set_range(&mut self, pos: (usize, usize), vals: Vec<u8>) {
        if (pos.1 - pos.0) + 1 != vals.len() {
            return;
        }
        let mut vals_iter = vals.iter();
        for ram_idx in pos.0..=pos.1 {
            let current_val = match vals_iter.next() {
                Some(x) => x,
                None => {
                    println!("set_range ran out of ram to write to. range specified = [{} - {}], len = {}. Length of vals to store = {}", 
                        pos.0, 
                        pos.1, 
                        pos.1-pos.0, 
                        vals.len());
                    return;
                }
            };
            self.set(ram_idx, *current_val)
        }
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
#[cfg(test)]
mod tests {
    use super::Ram;
    use rand::Rng;
    use std::collections::HashMap;
    

    #[test]
    fn set_first_ram_pos() {
        let mut ram = Ram::new();
        ram.set(0, 5);
        assert_eq!(ram.0[0], 5);

        // ensures rest of ram is set to 0
        for (idx, word) in ram.0.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            assert_eq!(*word, 0);
        }
    }

    #[test]
    fn set_random_ram_pos_x100() {
        let mut ram = Ram::new();
        let mut rng = rand::thread_rng();
        let mut asserts: HashMap<usize, u8> = HashMap::new();
        for _ in 0..100 {
            let pos: usize = rng.gen_range(0..crate::memory::RAM_SIZE - 1);
            let val: u8 = rng.gen();
            // if the entry doesn't exist, push into both the assert set and the ram
            if let std::collections::hash_map::Entry::Vacant(e) = asserts.entry(pos) {
                e.insert(val);
                ram.set(pos, val);
            }
        }

        for (pos, val) in asserts.iter() {
            assert_eq!(ram.get(*pos), *val);
        }

        // ensures rest of ram is set to 0
        for (idx, word) in ram.0.iter().enumerate() {
            if asserts.contains_key(&idx) {
                continue;
            }
            assert_eq!(*word, 0);
        }

    }

}
