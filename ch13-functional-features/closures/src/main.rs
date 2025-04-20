#[derive(Debug, PartialEq, Copy, Clone)]
enum Shirtcolor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<Shirtcolor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<Shirtcolor>) -> Shirtcolor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> Shirtcolor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                Shirtcolor::Red => num_red += 1,
                Shirtcolor::Blue => num_blue += 1,
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
