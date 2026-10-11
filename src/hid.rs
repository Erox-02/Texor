pub const report_size: usize = 8;
pub const max_keys: usize = 6;

#[derive(Clone, Copy)]
pub struct KeyboardReport {
    pub modifiers: u8,
    pub reserved: u8,
    pub keys: [u8; max_keys],
}

impl KeyboardReport {
    pub const fn new() -> Self {
        Self {
            modifiers: 0,
            reserved: 0,
            keys: [0; max_keys],
        }
    }

    pub fn press(&mut self, key: u8) {
        if (0xE0..=0xE7).contains(&key) {
            self.modifiers |= 1 << (key - 0xE0);
            return;
        }

        if self.keys.contains(&key) {
            return;
        }

        if let Some(slot) = self.keys.iter_mut().find(|slot| **slot == 0) {
            *slot = key;
        }
    }

    pub fn release(&mut self, key: u8) {
        if (0xE0..=0xE7).contains(&key) {
            self.modifiers &= !(1 << (key - 0xE0));
            return;
        }

        for slot in &mut self.keys {
            if *slot == key {
                *slot = 0;
            }
        }
    }

    pub fn release_all(&mut self) {
        self.modifiers = 0;
        self.keys = [0; max_keys];
    }

    pub const fn as_bytes(&self) -> [u8; report_size] {
        [
            self.modifiers,
            self.reserved,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
        ]
    }
}
