#[cfg(test)]
mod tests {
    use super::super::escalation::{EscalationLevel, EscalationTracker};
    use std::time::Duration;

    fn tracker() -> EscalationTracker {
        // warn at 3 fires, critical at 6 fires, within a 60s window
        EscalationTracker::new(3, 6, Duration::from_secs(60))
    }

    #[test]
    fn test_initial_fires_are_normal() {
        let mut t = tracker();
        assert_eq!(t.record("proc:cpu"), EscalationLevel::Normal);
        assert_eq!(t.record("proc:cpu"), EscalationLevel::Normal);
    }

    #[test]
    fn test_warn_threshold_reached() {
        let mut t = tracker();
        t.record("proc:mem");
        t.record("proc:mem");
        let level = t.record("proc:mem");
        assert_eq!(level, EscalationLevel::Warning);
    }

    #[test]
    fn test_critical_threshold_reached() {
        let mut t = tracker();
        for _ in 0..5 {
            t.record("proc:cpu");
        }
        let level = t.record("proc:cpu");
        assert_eq!(level, EscalationLevel::Critical);
    }

    #[test]
    fn test_reset_clears_state() {
        let mut t = tracker();
        for _ in 0..5 {
            t.record("proc:cpu");
        }
        t.reset("proc:cpu");
        assert_eq!(t.count("proc:cpu"), 0);
        let level = t.record("proc:cpu");
        assert_eq!(level, EscalationLevel::Normal);
    }

    #[test]
    fn test_independent_keys() {
        let mut t = tracker();
        for _ in 0..6 {
            t.record("proc:cpu");
        }
        // A different key should start fresh
        let level = t.record("proc:mem");
        assert_eq!(level, EscalationLevel::Normal);
        assert_eq!(t.count("proc:cpu"), 6);
        assert_eq!(t.count("proc:mem"), 1);
    }

    #[test]
    fn test_count_returns_zero_for_unknown_key() {
        let t = tracker();
        assert_eq!(t.count("nonexistent"), 0);
    }
}
