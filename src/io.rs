// iF someone ever reuses this code , del the io file dont reuse it

pub const cols: usize = 12;
pub const rows: usize = 7;

pub const col_pn: [u8; cols] = [
    1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
];

pub const row_pn: [u8; rows] = [
    14, 15, 16, 17, 18, 21, 48,
];

#[derive(Clone, Copy)]
pub struct Matrix {
    pub keys: [[bool; cols]; rows],
}

impl Matrix {
    pub const fn new() -> Self {
        Self {
            keys: [[false; cols]; rows],
        }
    }
}
