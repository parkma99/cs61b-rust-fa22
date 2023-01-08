use crate::deques::{averagingdeque::AveragingDeque, Average, Deque};
use rand::Rng;

#[test]
fn add_remove_test() {
    let mut lld = AveragingDeque::new();
    assert!(lld.is_empty());

    lld.add_first(10);
    assert_eq!(false, lld.is_empty());
    assert_eq!(Some(10.0), lld.average());
    let el = lld.remove_first();
    assert_eq!(el, Some(10));
    assert!(lld.is_empty());
    assert_eq!(None, lld.average());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());

    let el = lld.remove_first();
    assert_eq!(el, None);
    assert!(lld.is_empty());
}

#[test]
fn remove_empty_test() {
    let mut lld = AveragingDeque::new();
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
fn really_empty_test() {
    let mut lld = AveragingDeque::<i32>::new();
    assert_eq!(lld.remove_first(), None);
    assert_eq!(lld.remove_last(), None);
    assert_eq!(lld.get_first(), None);
    assert_eq!(lld.get_last(), None);
    assert_eq!(lld.get_first_mut(), None);
    assert_eq!(lld.get_last_mut(), None);
}

#[test]
fn big_test() {
    const N: i32 = 1000000;
    const MID: i32 = 500000;
    let mut lld = AveragingDeque::new();
    for i in 0..N {
        lld.add_last(i);
    }
    assert_eq!(Some(499999.5), lld.average());
    for i in 0..MID {
        assert_eq!(Some(i), lld.remove_first());
    }
    for i in (MID + 1..N).rev() {
        assert_eq!(Some(i), lld.remove_last());
    }
    assert_eq!(Some(500000.0), lld.average());
}

#[test]
fn get_test() {
    let mut lld = AveragingDeque::new();
    lld.add_first(10);
    lld.add_first(5);

    assert_eq!(None, lld.get_first_mut());
    assert_eq!(None, lld.get_last_mut());
    assert_eq!(2, lld.len());
}

#[test]
fn get_fist_remove_next_back_test() {
    let mut lld = AveragingDeque::new();
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
    let mut lld = AveragingDeque::new();
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
    let mut lld = AveragingDeque::new();
    let mut vector = Vec::new();

    for _ in 0..TIMES {
        match rng.gen_range(0..8) {
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
            6 => {
                if vector.len() > 0 {
                    assert_eq!(Some(vector.remove(0)), lld.remove_first())
                } else {
                    assert_eq!(None, lld.remove_first());
                }
            }
            _ => {
                let mut sum = 0.0;
                for i in &vector {
                    sum += *i as f64;
                }
                if vector.len() > 0 {
                    assert_eq!(Some(sum / vector.len() as f64), lld.average());
                } else {
                    assert_eq!(None, lld.average());
                }
            }
        }
    }
}
