/// ans = 4613732

struct Fib(usize, usize);
impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        *self = Fib(self.1, self.0 + self.1);
        Some(self.1)
    }
}

fn main() {
    let mut ans = 0;
    let fib = Fib(0, 1);
    for x in fib {
        if x > 4000000 {
            break;
        }
        if x % 2 == 0 {
            ans += x;
        }
    }
    println!("{}", ans);
}