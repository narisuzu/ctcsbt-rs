pub struct List<N, const M: usize> {
    data: [N; M],
    len: usize,
}

impl<N: Copy, const M: usize> List<N, M>{
    pub fn push(&mut self, value: N) {
        self.data[self.len] = value;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<N> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.data[self.len])
        }
    }
}