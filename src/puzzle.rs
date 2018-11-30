use MAX_SIZE;

#[derive(Default)]
pub struct Puzzle([[char; MAX_SIZE]; MAX_SIZE]);

impl Puzzle {
    pub fn set(&mut self, r: usize, c: usize, val: char) {
        self.0[r][c] = val;
    }

    pub fn coords_for_char(&self, val: char) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(5);
        for (r, row) in self.0.iter().enumerate() {
            for (c, p_val) in row.iter().enumerate() {
                if *p_val == val {
                    result.push((r, c));
                }
            }
        }
        result
    }
}
