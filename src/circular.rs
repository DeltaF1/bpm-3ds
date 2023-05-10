pub struct CircularBuffer<T, const LEN: usize> {
    items: [T; LEN],
    head: usize,
}

impl<T: Default + Copy, const LEN: usize> CircularBuffer<T, LEN> {
    pub fn new() -> CircularBuffer<T, LEN> {
        CircularBuffer {
            items: [Default::default(); LEN],
            head: 0,
        }
    }

    pub fn add(&mut self, item: T) {
        self.items[self.head] = item;
        self.head = (self.head + 1) % LEN;
    }

    pub const fn len(&self) -> usize {
        LEN
    }
}

impl<T: Copy, const LEN: usize> From<&[T; LEN]> for CircularBuffer<T, LEN> {
    fn from(other: &[T; LEN]) -> Self {
        CircularBuffer {
            items: *other,
            head: 0
        }
    }
}

impl<'a, T, const LEN: usize> IntoIterator for &'a CircularBuffer<T, LEN> {
    type Item = &'a T;
    type IntoIter = CircularBufferIterator<'a, T, LEN>;
    fn into_iter(self) -> Self::IntoIter {
        CircularBufferIterator {
            buffer: &self,
            ptr: self.head,
            count: LEN,
        }
    }
}

pub struct CircularBufferIterator<'a, T, const LEN: usize> {
    buffer: &'a CircularBuffer<T, LEN>,
    ptr: usize,
    count: usize,
}

impl<'a, T, const LEN: usize> Iterator for CircularBufferIterator<'a, T, LEN> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        let ptr = self.ptr;
        self.ptr = (self.ptr + 1) % LEN;
        self.count -= 1;
        return Some(&self.buffer.items[ptr]);
    }
}
