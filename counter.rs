struct Counter {
    val: Box<u64>,
}

impl Counter {
    fn new() -> Counter {
        return Counter { val: Box::new(0) };
    }

    fn read(&self) -> u64 {
        *self.val
    }

    fn count(&mut self) {
        *self.val += 1;
    }
}

mod tests {
    use crate::Counter;

    #[test]
    fn starts_at_zero() {
        assert_eq!(Counter::new().read(), 0)
    }

    #[test]
    fn count_one() {
        let mut counter = Counter::new();
        counter.count();
        assert_eq!(counter.read(), 1);
    }

    #[test]
    fn count_two() {
        let mut counter = Counter::new();
        counter.count();
        assert_eq!(counter.read(), 1);
        counter.count();
        assert_eq!(counter.read(), 2);
    }
}
