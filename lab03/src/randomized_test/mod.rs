use self::{alistnoresizing::AListNR, buggyalist::BAList};

mod alistnoresizing;
mod buggyalist;

#[test]
fn test_add_3_remove_3() {
    let mut correct: AListNR<i32> = AListNR::new();
    let mut broken: BAList<i32> = BAList::new();

    correct.add_last(1);
    correct.add_last(2);
    correct.add_last(3);
    broken.add_last(1);
    broken.add_last(2);
    broken.add_last(3);
    assert_eq!(correct.len(), broken.len());
    assert_eq!(correct.remove_last(), broken.remove_last());
    assert_eq!(correct.remove_last(), broken.remove_last());
    assert_eq!(correct.remove_last(), broken.remove_last());
}

#[test]
fn randomized_test() {
    let mut correct: Vec<i32> = Vec::new();
    let mut broken: BAList<i32> = BAList::new();

    for _ in 0..5000 {
        match rand::random::<u32>() % 4 {
            0 => {
                println!("0");
                let num = rand::random::<i32>() % 1000;
                correct.push(num);
                broken.add_last(num);
            }
            1 => {
                println!("1");
                assert_eq!(correct.len(), broken.len());
            }
            2 => {
                println!("2");
                assert_eq!(correct.last(), broken.get_last());
            }
            3 => {
                println!("3");
                assert_eq!(correct.pop(), broken.remove_last());
            }
            _defalt => {
                println!("{}", _defalt);
            }
        }
    }
}
