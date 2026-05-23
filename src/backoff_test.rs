#[cfg(test)]
mod tests {
    use super::super::backoff::Backoff;
    use std::time::Duration;

    fn make_backoff() -> Backoff {
        Backoff::new(Duration::from_millis(100), Duration::from_secs(5), 2.0)
    }

    #[test]
    fn test_initial_delay_is_base() {
        let mut b = make_backoff();
        assert_eq!(b.next_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_delay_doubles_each_attempt() {
        let mut b = make_backoff();
        let d1 = b.next_delay();
        let d2 = b.next_delay();
        let d3 = b.next_delay();
        assert_eq!(d1, Duration::from_millis(100));
        assert_eq!(d2, Duration::from_millis(200));
        assert_eq!(d3, Duration::from_millis(400));
    }

    #[test]
    fn test_delay_capped_at_max() {
        let mut b = make_backoff();
        for _ in 0..10 {
            b.next_delay();
        }
        assert!(b.peek_delay() <= Duration::from_secs(5));
    }

    #[test]
    fn test_reset_restores_initial_delay() {
        let mut b = make_backoff();
        b.next_delay();
        b.next_delay();
        b.reset();
        assert_eq!(b.attempt(), 0);
        assert_eq!(b.peek_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_is_exhausted() {
        let mut b = make_backoff();
        assert!(!b.is_exhausted(3));
        b.next_delay();
        b.next_delay();
        b.next_delay();
        assert!(b.is_exhausted(3));
    }

    #[test]
    fn test_peek_does_not_advance() {
        let b = make_backoff();
        let p1 = b.peek_delay();
        let p2 = b.peek_delay();
        assert_eq!(p1, p2);
        assert_eq!(b.attempt(), 0);
    }

    #[test]
    fn test_default_backoff() {
        let mut b = Backoff::default();
        assert_eq!(b.next_delay(), Duration::from_millis(500));
    }
}
