pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
    where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger,
                       value: 0,
                       max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage of max = self.value as f64 / self.max as f64
    }
}

fn main() {
    let x = 5;
    // let y = &mut x;
}
