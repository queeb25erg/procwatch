#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use crate::debounce::Debouncer;

    #[test]
    fn first_event_always_fires() {
        let mut d = Debouncer::new(Duration::from_millis(200));
        assert!(d.should_fire("proc_cpu"));
    }

    #[test]
    fn second_event_within_window_is_suppressed() {
        let mut d = Debouncer::new(Duration::from_millis(200));
        assert!(d.should_fire("proc_cpu"));
        assert!(!d.should_fire("proc_cpu"));
    }

    #[test]
    fn event_fires_again_after_window_expires() {
        let mut d = Debouncer::new(Duration::from_millis(50));
        assert!(d.should_fire("proc_mem"));
        thread::sleep(Duration::from_millis(60));
        assert!(d.should_fire("proc_mem"));
    }

    #[test]
    fn different_keys_are_independent() {
        let mut d = Debouncer::new(Duration::from_millis(200));
        assert!(d.should_fire("key_a"));
        assert!(d.should_fire("key_b"));
        assert!(!d.should_fire("key_a"));
        assert!(!d.should_fire("key_b"));
    }

    #[test]
    fn reset_allows_immediate_refire() {
        let mut d = Debouncer::new(Duration::from_millis(500));
        assert!(d.should_fire("proc_cpu"));
        d.reset("proc_cpu");
        assert!(d.should_fire("proc_cpu"));
    }

    #[test]
    fn evict_expired_removes_stale_entries() {
        let mut d = Debouncer::new(Duration::from_millis(30));
        d.should_fire("a");
        d.should_fire("b");
        assert_eq!(d.len(), 2);
        thread::sleep(Duration::from_millis(40));
        d.evict_expired();
        assert!(d.is_empty());
    }

    #[test]
    fn evict_expired_keeps_fresh_entries() {
        let mut d = Debouncer::new(Duration::from_millis(200));
        d.should_fire("fresh");
        d.evict_expired();
        assert_eq!(d.len(), 1);
    }

    #[test]
    fn len_and_is_empty_reflect_state() {
        let mut d = Debouncer::new(Duration::from_millis(200));
        assert!(d.is_empty());
        d.should_fire("x");
        assert_eq!(d.len(), 1);
        assert!(!d.is_empty());
    }
}
