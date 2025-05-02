use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    let result = add_one_v2(5);
    println!("{result}");

    let add_one_v4 = |x| x+1;

    let result2 = add_one_v4(6);
    println!("{result2}");

    let list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list2 = vec![1,2,3];
    println!("Before defining closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After calling closure: {list2:?}");

    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {list3:?}");

    thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();

    let mut list4 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list4.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{list4:#?}, sorted in {num_sort_operations} operations");
}
