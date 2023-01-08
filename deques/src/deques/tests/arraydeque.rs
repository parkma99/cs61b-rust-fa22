use crate::deques::{arraydeque::ArrayDeque, Deque};
use rand::Rng;

#[test]
fn add_is_empty_size_test() {
    let mut lld = ArrayDeque::new();
    assert!(lld.is_empty());

    lld.add_first("front");
    assert_eq!(1, lld.len());

    lld.add_last("middle");
    assert_eq!(2, lld.len());

    lld.add_last("back");
    assert_eq!(3, lld.len());

    // assert_eq!(format!("{:?}", lld), "")
    assert_eq!(lld.to_string(), "[front, middle, back]");
}

#[test]
fn add_remove_test() {
    let mut lld = ArrayDeque::new();
    assert!(lld.is_empty());

    lld.add_first(10);
    assert_eq!(false, lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, Some(10));
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());
}

#[test]
fn remove_empty_test() {
    let mut lld = ArrayDeque::new();
    lld.add_first(10);

    let el = lld.remove_last();
    assert_eq!(el, Some(10));
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_last();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    assert_eq!(lld.len(), 0);
}

#[test]
fn multiple_param_test() {
    let mut lld = ArrayDeque::new();
    lld.add_first(10 as i32);
    assert_eq!(lld.remove_last(), Some(10));

    let mut lld = ArrayDeque::new();
    lld.add_first(10. as f64);
    assert_eq!(lld.remove_last(), Some(10.));

    let mut lld = ArrayDeque::new();
    lld.add_first(10 as usize);
    assert_eq!(lld.remove_last(), Some(10));

    let mut lld = ArrayDeque::new();
    lld.add_first(true);
    assert_eq!(lld.remove_last(), Some(true));

    let mut lld = ArrayDeque::new();
    lld.add_first("cheese");
    assert_eq!(lld.remove_last(), Some("cheese"));

    let mut lld = ArrayDeque::new();
    lld.add_first("cheese".to_string());
    assert_eq!(lld.remove_last(), Some("cheese".to_string()));
}

#[test]
fn really_empty_test() {
    let mut lld = ArrayDeque::<i32>::new();
    assert_eq!(lld.remove_first(), None);
    assert_eq!(lld.remove_last(), None);
    assert_eq!(lld.get_first(), None);
    assert_eq!(lld.get_last(), None);
    assert_eq!(lld.get_first_mut(), None);
    assert_eq!(lld.get_last_mut(), None);
}

#[test]
fn big_test() {
    const N: usize = 1000000;
    const MID: usize = 500000;
    let mut lld = ArrayDeque::new();
    for i in 0..N {
        lld.add_last(i);
    }
    for i in 0..MID {
        assert_eq!(Some(i), lld.remove_first());
    }
    for i in (MID + 1..N).rev() {
        assert_eq!(Some(i), lld.remove_last());
    }
}

#[test]
fn get_test() {
    let mut lld = ArrayDeque::new();
    lld.add_first(10);
    lld.add_first(5);

    let b = lld.get_first_mut().unwrap();
    *b = 64;

    let b = lld.get_last_mut().unwrap();
    *b = 65;

    lld.add_last(66);
    assert_eq!(Some(&64), lld.get_first());
    assert_eq!(Some(&66), lld.get_last());
    assert_eq!(lld.to_string(), "[64, 65, 66]");
}

#[test]
fn get_fist_remove_next_back_test() {
    let mut lld = ArrayDeque::new();
    lld.add_first(10);
    lld.add_last(5);
    lld.add_first(20);

    assert_eq!(Some(20), lld.remove_first());
    assert_eq!(Some(10), lld.remove_first());
    assert_eq!(Some(5), lld.remove_last());
    assert_eq!(None, lld.remove_first());
}

#[test]
fn get_last_remove_next_back_test() {
    let mut lld = ArrayDeque::new();
    lld.add_last(10);
    lld.add_first(5);
    lld.add_last(20);

    assert_eq!(Some(20), lld.remove_last());
    assert_eq!(Some(10), lld.remove_last());
    assert_eq!(Some(5), lld.remove_first());
    assert_eq!(None, lld.remove_first());
}

#[test]
fn random_test() {
    const TIMES: i32 = 100000;
    let mut rng = rand::thread_rng();
    let mut lld = ArrayDeque::new();
    let mut vector = Vec::new();

    for _ in 0..TIMES {
        match rng.gen_range(0..7) {
            0 => {
                assert_eq!(vector.len(), lld.len());
            }
            1 => {
                let value = rng.gen_range(0..100);
                lld.add_first(value);
                vector.insert(0, value);
            }
            2 => {
                let value = rng.gen_range(0..100);
                lld.add_last(value);
                vector.push(value);
            }
            3 => {
                assert_eq!(vector.last(), lld.get_last());
            }
            4 => {
                assert_eq!(vector.get(0), lld.get_first());
            }
            5 => {
                assert_eq!(vector.pop(), lld.remove_last());
            }
            _ => {
                if vector.len() > 0 {
                    assert_eq!(Some(vector.remove(0)), lld.remove_first())
                } else {
                    assert_eq!(None, lld.remove_first());
                }
            }
        }
    }
}
