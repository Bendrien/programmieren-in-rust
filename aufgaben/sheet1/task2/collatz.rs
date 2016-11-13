fn main() {
    for (i, c) in collatz(27).enumerate().skip(1) {
        println!("{} -> {}", i, c);
    }
}

struct Collatz {
    next_value: Option<usize>,
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let saved = self.next_value;
        if let Some(x) = self.next_value {
            self.next_value = if x <= 1 {
                None
            } else if x % 2 == 0 {
                Some(x/2)
            } else {
                Some(3*x + 1)
            }
        }
        saved
    }
}

fn collatz(n: usize) -> Collatz {
    Collatz{ next_value: Some(n) }
}

#[test]
fn collatz_5() {
    let mut collatz_iter = collatz(5);
    assert_eq!(collatz_iter.next(), Some(5));
    assert_eq!(collatz_iter.next(), Some(16));
    assert_eq!(collatz_iter.next(), Some(8));
    assert_eq!(collatz_iter.next(), Some(4));
    assert_eq!(collatz_iter.next(), Some(2));
    assert_eq!(collatz_iter.next(), Some(1));
    assert_eq!(collatz_iter.next(), None);
}