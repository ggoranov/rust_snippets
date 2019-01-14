use std::clone::Clone;
use std::cmp::Ordering;

struct _MyTyple(u8, f64);

/// Tuple that has two elements of types T and V
/// Where T must impl Copy trait and V must impl Ord trait
struct _MyGenericType<T: Copy, V: Ord>(T, V);

fn find_min_max<T: Copy + Ord + Clone>(data: Vec<T>) -> (Option<T>, Option<T>) {
    let mut it = data.into_iter();

    // fn next(&mut self) -> Option<Self::Item>
    let mut min: T = match it.next() {
        Some(elem) => elem,
        None => return (None, None),
    };

    // Cloned not borrowed
    let mut max: T = min.clone();

    for elem in it {
        if elem < min {
            min = elem;
        }

        if elem > max {
            max = elem;
        }
    }

    (Some(min), Some(max))
}

#[derive(Eq, Debug, PartialEq, Copy, Clone)]
struct Person {
    id: u32,
    height: u32,
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_find_max() {
    let max_person = Person {
        id: 5,
        height: 1123,
    };

    let min_person = Person { id: 3, height: 11 };
    let vec2 = vec![
        Person { id: 0, height: 112 },
        Person { id: 2, height: 222 },
        min_person,
        Person { id: 4, height: 111 },
        max_person,
        Person { id: 6, height: 12 },
    ];
    let (min_val, max_val) = find_min_max::<Person>(vec2);
    println!("Min {:?}\nMax {:?}", min_val.unwrap(), max_val.unwrap());

    assert_eq!(Some(max_person), max_val);
    assert_eq!(Some(min_person), min_val);
}

fn main() {
    let val: u16 = 1;
    let (a, _) = find_min_max(vec![val]);

    // the trait `std::cmp::Ord` is not implemented for `f64`
    //let b = find_min_max(vec![10.12, 20.22, 30f64, 40f64]);
    //                          ^^^ the trait bound `f64: std::cmp::Ord` is not satisfied

    let (min, max) = find_min_max(vec![100u64, 20u64, 30u64, 40u64]);
    println!("Max unsigned: {} {}", min.unwrap(), max.unwrap());
}
