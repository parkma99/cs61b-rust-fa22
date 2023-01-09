use crate::deques::Deque;
use std::{fmt::Display, mem};

#[derive(Default)]
pub struct ArrayDeque<T> {
    items: Box<[T]>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T: Default + Display> Display for ArrayDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if self.size == 0 {
            return write!(f, "]");
        }
        for i in 1..self.size {
            let j = (self.head + i) % self.items.len();
            write!(f, "{}, ", self.items[j])?;
        }
        let j = (self.head + self.size) % self.items.len();
        write!(f, "{}", self.items[j])?;
        write!(f, "]")
    }
}
impl<T: Default + Display> ArrayDeque<T> {
    fn resize(&mut self, new_size: usize) {
        let mut new_items = allocate_slice(new_size);
        let mut i = self.plus_one(self.head);
        for j in 0..self.size {
            new_items[j] = mem::take(&mut self.items[i]);
            i = self.plus_one(i);
        }
        self.items = new_items;
        self.head = new_size - 1;
        self.tail = self.size;
    }
    fn plus_one(&self, i: usize) -> usize {
        (i + 1) % self.items.len()
    }
    fn minus_one(&self, i: usize) -> usize {
        (i + self.items.len() - 1) % self.items.len()
    }
}
impl<T: Default + Display> Deque for ArrayDeque<T> {
    type Item = T;

    fn new() -> Self {
        Self {
            items: allocate_slice(8),
            head: 0,
            tail: 1,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn add_first(&mut self, item: Self::Item) {
        if self.size == self.items.len() {
            self.resize(self.size * 2);
        }
        self.items[self.head] = item;
        self.head = self.minus_one(self.head);
        self.size += 1;
    }

    fn add_last(&mut self, item: Self::Item) {
        if self.size == self.items.len() {
            self.resize(self.size * 2);
        }
        self.items[self.tail] = item;
        self.tail = self.plus_one(self.tail);
        self.size += 1;
    }

    fn remove_first(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        self.head = self.plus_one(self.head);
        let item = mem::take(&mut self.items[self.head]);
        self.size -= 1;
        if self.size < self.items.len() / 4 {
            self.resize(self.items.len() / 2);
        }
        Some(item)
    }

    fn remove_last(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }
        self.tail = self.minus_one(self.tail);
        let item = mem::take(&mut self.items[self.tail]);
        self.size -= 1;
        if self.size < self.items.len() / 4 {
            self.resize(self.items.len() / 2);
        }
        Some(item)
    }

    fn get_first(&self) -> Option<&Self::Item> {
        if self.size == 0 {
            return None;
        }
        let cur = self.plus_one(self.head);
        self.items.get(cur)
    }

    fn get_last(&self) -> Option<&Self::Item> {
        if self.size == 0 {
            return None;
        }
        let cur = self.minus_one(self.tail);
        self.items.get(cur)
    }

    fn get_first_mut(&mut self) -> Option<&mut Self::Item> {
        if self.size == 0 {
            return None;
        }
        let cur = self.plus_one(self.head);
        self.items.get_mut(cur)
    }

    fn get_last_mut(&mut self) -> Option<&mut Self::Item> {
        if self.size == 0 {
            return None;
        }
        let cur = self.minus_one(self.tail);
        self.items.get_mut(cur)
    }
}

/* Allocates a fixed-length Box<[T]> */
fn allocate_slice<T: Default>(n: usize) -> Box<[T]> {
    unsafe {
        let layout = std::alloc::Layout::array::<T>(n).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut T;
        for i in 0..n {
            std::ptr::write(ptr.add(i), T::default());
        }
        let slice = core::slice::from_raw_parts_mut(ptr, n);
        Box::from_raw(slice)
    }
}
