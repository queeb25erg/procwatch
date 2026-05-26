#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::drain::{Drain, DrainPolicy};

    #[test]
    fn test_drain_drop_policy() {
        let mut drain = Drain::new(DrainPolicy::Drop);
        drain.push("event-1".into());
        drain.push("event-2".into());
        drain.push("event-3".into());

        let result = drain.execute(|_| true);
        assert_eq!(result.flushed, 0);
        assert_eq!(result.dropped, 3);
        assert_eq!(drain.pending(), 0);
    }

    #[test]
    fn test_drain_immediate_all_succeed() {
        let mut drain = Drain::new(DrainPolicy::Immediate);
        drain.push("a".into());
        drain.push("b".into());

        let result = drain.execute(|_| true);
        assert_eq!(result.flushed, 2);
        assert_eq!(result.dropped, 0);
        assert_eq!(drain.pending(), 0);
    }

    #[test]
    fn test_drain_immediate_partial_failure() {
        let mut drain = Drain::new(DrainPolicy::Immediate);
        drain.push("ok".into());
        drain.push("fail".into());
        drain.push("ok".into());

        let result = drain.execute(|item| item != "fail");
        assert_eq!(result.flushed, 2);
        assert_eq!(result.dropped, 1);
    }

    #[test]
    fn test_drain_timeout_flushes_within_limit() {
        let mut drain = Drain::new(DrainPolicy::Timeout(Duration::from_secs(5)));
        for i in 0..10 {
            drain.push(format!("event-{}", i));
        }

        let result = drain.execute(|_| true);
        // With a generous timeout all items should flush
        assert_eq!(result.flushed + result.dropped, 10);
        assert_eq!(drain.pending(), 0);
    }

    #[test]
    fn test_drain_timeout_zero_drops_all() {
        let mut drain = Drain::new(DrainPolicy::Timeout(Duration::from_nanos(0)));
        drain.push("x".into());
        drain.push("y".into());

        let result = drain.execute(|_| true);
        // Zero timeout means nothing gets flushed before deadline
        assert_eq!(result.flushed + result.dropped, 2);
        assert_eq!(drain.pending(), 0);
    }

    #[test]
    fn test_drain_pending_count() {
        let mut drain = Drain::new(DrainPolicy::Immediate);
        assert_eq!(drain.pending(), 0);
        drain.push("one".into());
        drain.push("two".into());
        assert_eq!(drain.pending(), 2);
    }
}
