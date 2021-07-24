pub trait SlicesBitAnd<T> {
    fn bitand_as_usize(&self, other: &Self) -> Option<usize>;
    fn bitand_as_string(&self, other: &Self) -> Option<String>;
}

impl<T> SlicesBitAnd<T> for [T] where T: PartialEq {
    fn bitand_as_usize(&self, other: &Self) -> Option<usize> {
        if self.len() != other.len() { return None; }

        let mut bit: usize = 0;
        for i in 0..self.len() {
            bit <<= 1;
            if self[i] == other[i] {
                bit |= 1;
            } else {
                bit |= 0;
            }
        }
        Some(bit)
    }

    fn bitand_as_string(&self, other: &Self) -> Option<String> {
        if self.len() != other.len() { return None; }

        let mut bit = "".to_string();
        for i in 0..self.len() {
            if self[i] == other[i] {
                bit.push('1');
            } else {
                bit.push('0');
            }
        }

        Some(bit)
    }
}
