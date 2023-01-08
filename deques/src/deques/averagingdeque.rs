use crate::deques::arraydeque::ArrayDeque;
use crate::deques::{Average, Deque};
use std::fmt::Display;

pub struct AveragingDeque<T> {
    base: ArrayDeque<T>,
    sum: f64,
}

impl<T: Default + Display + Clone + Into<f64>> Display for AveragingDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)
    }
}

impl<T: Default + Display + Clone + Into<f64>> Deque for AveragingDeque<T> {
    type Item = T;

    fn new() -> Self {
        Self {
            base: ArrayDeque::new(),
            sum: 0.0,
        }
    }

    fn len(&self) -> usize {
        self.base.len()
    }

    fn add_first(&mut self, item: Self::Item) {
        self.sum += item.clone().into();
        self.base.add_first(item);
    }

    fn add_last(&mut self, item: Self::Item) {
        self.sum += item.clone().into();
        self.base.add_last(item);
    }

    fn remove_first(&mut self) -> Option<Self::Item> {
        match self.base.remove_first() {
            None => None,
            Some(item) => {
                self.sum -= item.clone().into();
                Some(item)
            }
        }
    }

    fn remove_last(&mut self) -> Option<Self::Item> {
        match self.base.remove_last() {
            None => None,
            Some(item) => {
                self.sum -= item.clone().into();
                Some(item)
            }
        }
    }

    fn get_first(&self) -> Option<&Self::Item> {
        self.base.get_first()
    }

    fn get_last(&self) -> Option<&Self::Item> {
        self.base.get_last()
    }

    fn get_first_mut(&mut self) -> Option<&mut Self::Item> {
        None
    }

    fn get_last_mut(&mut self) -> Option<&mut Self::Item> {
        None
    }
}

impl<T: Default + Display + Clone + Into<f64>> Average for AveragingDeque<T> {
    fn average(&self) -> Option<f64> {
        match self.len() {
            0 => None,
            size => Some(self.sum / (size as f64)),
        }
    }
}
