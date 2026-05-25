#[cfg(test)]
mod tests {
    use super::super::cooldown::CooldownTracker;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_process_is_allowed() {
        let tracker = CooldownTracker::new(60);
        assert!(tracker.is_allowed("nginx"));
    }

    #[test]
    fn test_after_record_process_is_blocked() {
        let mut tracker = CooldownTracker::new(60);
        tracker.record_alert("nginx");
        assert!(!tracker.is_allowed("nginx"));
    }

    #[test]
    fn test_different_processes_are_independent() {
        let mut tracker = CooldownTracker::new(60);
        tracker.record_alert("nginx");
        assert!(!tracker.is_allowed("nginx"));
        assert!(tracker.is_allowed("postgres"));
    }

    #[test]
    fn test_cooldown_expires_after_duration() {
        let mut tracker = CooldownTracker::new(1);
        tracker.record_alert("myapp");
        assert!(!tracker.is_allowed("myapp"));
        thread::sleep(Duration::from_millis(1100));
        assert!(tracker.is_allowed("myapp"));
    }

    #[test]
    fn test_remaining_secs_when_not_tracked() {
        let tracker = CooldownTracker::new(30);
        assert_eq!(tracker.remaining_secs("unknown"), 0);
    }

    #[test]
    fn test_remaining_secs_after_record() {
        let mut tracker = CooldownTracker::new(30);
        tracker.record_alert("redis");
        let remaining = tracker.remaining_secs("redis");
        assert!(remaining > 0 && remaining <= 30);
    }

    #[test]
    fn test_remaining_secs_decreases_after_expiry() {
        let mut tracker = CooldownTracker::new(1);
        tracker.record_alert("myapp");
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(tracker.remaining_secs("myapp"), 0);
    }

    #[test]
    fn test_reset_clears_cooldown() {
        let mut tracker = CooldownTracker::new(60);
        tracker.record_alert("worker");
        assert!(!tracker.is_allowed("worker"));
        tracker.reset("worker");
        assert!(tracker.is_allowed("worker"));
    }

    #[test]
    fn test_reset_all_clears_all_processes() {
        let mut tracker = CooldownTracker::new(60);
        tracker.record_alert("svc_a");
        tracker.record_alert("svc_b");
        tracker.reset_all();
        assert!(tracker.is_allowed("svc_a"));
        assert!(tracker.is_allowed("svc_b"));
    }
}
