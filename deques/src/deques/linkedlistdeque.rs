use std::fmt::Display;

use crate::deques::Deque;

struct Node<T> {
    item: T,
    prev: usize,
    next: usize,
}

pub struct LinkedListDeque<T> {
    nodes: Vec<Node<T>>,
}

impl<T: Default + Display> Display for LinkedListDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut cur_point = self.nodes.get(0).unwrap().next;
        while cur_point != 0 {
            let cur_item = &self.nodes.get(cur_point).unwrap().item;
            let cur_next = self.nodes.get(cur_point).unwrap().next;
            if cur_next == 0 {
                write!(f, "{}", cur_item)?;
            } else {
                write!(f, "{}, ", cur_item)?;
            }
            cur_point = cur_next;
        }
        write!(f, "]")
    }
}

impl<T: Default + Display> Deque for LinkedListDeque<T> {
    type Item = T;

    fn new() -> Self {
        Self {
            nodes: vec![Node {
                item: T::default(),
                prev: 0,
                next: 0,
            }],
        }
    }

    fn len(&self) -> usize {
        self.nodes.len() - 1
    }

    fn add_first(&mut self, item: Self::Item) {
        let sentinel = self.nodes.get(0).unwrap();
        let cur_first = sentinel.next;
        self.nodes.push(Node {
            item,
            prev: 0,
            next: cur_first,
        });
        self.nodes.get_mut(cur_first).unwrap().prev = self.nodes.len() - 1;
        self.nodes.get_mut(0).unwrap().next = self.nodes.len() - 1;
    }

    fn add_last(&mut self, item: Self::Item) {
        let sentinel = self.nodes.get(0).unwrap();
        let cur_last = sentinel.prev;
        self.nodes.push(Node {
            item,
            prev: cur_last,
            next: 0,
        });
        self.nodes.get_mut(cur_last).unwrap().next = self.nodes.len() - 1;
        self.nodes.get_mut(0).unwrap().prev = self.nodes.len() - 1;
    }

    fn remove_first(&mut self) -> Option<Self::Item> {
        match self.len() {
            0 => None,
            1 => {
                self.nodes.get_mut(0).unwrap().next = 0;
                self.nodes.get_mut(0).unwrap().prev = 0;
                self.nodes.pop().map(|node| node.item)
            }
            _ => {
                let sentinel = self.nodes.get(0).unwrap();
                let cur_first = sentinel.next;
                let cur_back = self.nodes.len() - 1;
                let cur_first_node = self.nodes.get(cur_first).unwrap();
                let cur_back_node = self.nodes.get(cur_back).unwrap();
                let cur_first_next = cur_first_node.next;
                let cur_back_next = cur_back_node.next;
                let cur_back_prev = cur_back_node.prev;
                if cur_first == cur_back {
                    self.nodes.get_mut(0).unwrap().next = cur_first_next;
                    self.nodes.get_mut(cur_first_next).unwrap().prev = 0;
                    self.nodes.pop().map(|node| node.item)
                } else if cur_first_next == cur_back {
                    self.nodes.get_mut(cur_back).unwrap().prev = 0;
                    self.nodes.get_mut(cur_back_next).unwrap().prev = cur_first;
                    self.nodes.swap(cur_first, cur_back);
                    self.nodes.pop().map(|node| node.item)
                } else {
                    self.nodes.swap(cur_first, cur_back);
                    self.nodes.get_mut(0).unwrap().next = cur_first_next;
                    self.nodes.get_mut(cur_first_next).unwrap().prev = 0;
                    self.nodes.get_mut(cur_back_next).unwrap().prev = cur_first;
                    self.nodes.get_mut(cur_back_prev).unwrap().next = cur_first;
                    self.nodes.pop().map(|node| node.item)
                }
            }
        }
    }

    fn remove_last(&mut self) -> Option<Self::Item> {
        match self.len() {
            0 => None,
            1 => {
                self.nodes.get_mut(0).unwrap().next = 0;
                self.nodes.get_mut(0).unwrap().prev = 0;
                self.nodes.pop().map(|node| node.item)
            }
            _ => {
                let sentinel = self.nodes.get(0).unwrap();
                let cur_last = sentinel.prev;
                let cur_back = self.nodes.len() - 1;
                let cur_last_node = self.nodes.get(cur_last).unwrap();
                let cur_back_node = self.nodes.get(cur_back).unwrap();
                let cur_last_prev = cur_last_node.prev;
                let cur_back_next = cur_back_node.next;
                let cur_back_prev = cur_back_node.prev;
                if cur_last == cur_back {
                    self.nodes.get_mut(0).unwrap().prev = cur_last_prev;
                    self.nodes.get_mut(cur_last_prev).unwrap().next = 0;
                    self.nodes.pop().map(|node| node.item)
                } else if cur_last_prev == cur_back {
                    self.nodes.get_mut(cur_back).unwrap().next = 0;
                    self.nodes.get_mut(cur_back_prev).unwrap().next = cur_last;
                    self.nodes.swap(cur_last, cur_back);
                    self.nodes.pop().map(|node| node.item)
                } else {
                    self.nodes.swap(cur_last, cur_back);
                    self.nodes.get_mut(0).unwrap().prev = cur_last_prev;
                    self.nodes.get_mut(cur_last_prev).unwrap().next = 0;
                    self.nodes.get_mut(cur_back_next).unwrap().prev = cur_last;
                    self.nodes.get_mut(cur_back_prev).unwrap().next = cur_last;
                    self.nodes.pop().map(|node| node.item)
                }
            }
        }
    }

    fn get_first(&self) -> Option<&Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let sentinel = self.nodes.get(0).unwrap();
        let cur_first = sentinel.next;
        self.nodes.get(cur_first).map(|node| &node.item)
    }

    fn get_last(&self) -> Option<&Self::Item> {
        if self.len() == 0 {
            return None;
        }

        let sentinel = self.nodes.get(0).unwrap();
        let cur_last = sentinel.prev;
        self.nodes.get(cur_last).map(|node| &node.item)
    }

    fn get_first_mut(&mut self) -> Option<&mut Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let sentinel = self.nodes.get(0).unwrap();
        let cur_first = sentinel.next;
        self.nodes.get_mut(cur_first).map(|node| &mut node.item)
    }

    fn get_last_mut(&mut self) -> Option<&mut Self::Item> {
        if self.len() == 0 {
            return None;
        }
        let sentinel = self.nodes.get(0).unwrap();
        let cur_last = sentinel.prev;
        self.nodes.get_mut(cur_last).map(|node| &mut node.item)
    }
}
