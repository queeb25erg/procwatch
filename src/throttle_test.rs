#[cfg(test)]
mod tests {
    use super::super::throttle::AlertThrottle;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn first_fire_is_allowed() {
        let mut throttle = AlertThrottle::new(60);
        assert!(throttle.should_fire("proc:nginx:cpu"));
    }

    #[test]
    fn second_immediate_fire_is_blocked() {
        let mut throttle = AlertThrottle::new(60);
        throttle.should_fire("proc:nginx:cpu");
        assert!(!throttle.should_fire("proc:nginx:cpu"));
    }

    #[test]
    fn different_keys_are_independent() {
        let mut throttle = AlertThrottle::new(60);
        assert!(throttle.should_fire("proc:nginx:cpu"));
        assert!(throttle.should_fire("proc:redis:mem"));
    }

    #[test]
    fn fire_allowed_after_cooldown_expires() {
        let mut throttle = AlertThrottle::new(1);
        assert!(throttle.should_fire("proc:app:cpu"));
        thread::sleep(Duration::from_millis(1100));
        assert!(throttle.should_fire("proc:app:cpu"));
    }

    #[test]
    fn reset_clears_cooldown() {
        let mut throttle = AlertThrottle::new(60);
        throttle.should_fire("proc:app:cpu");
        throttle.reset("proc:app:cpu");
        assert!(throttle.should_fire("proc:app:cpu"));
    }

    #[test]
    fn evict_stale_removes_old_entries() {
        let mut throttle = AlertThrottle::new(1);
        throttle.should_fire("proc:old:cpu");
        thread::sleep(Duration::from_millis(2100));
        throttle.evict_stale();
        // After eviction, key is gone so firing is allowed again without
        // updating the internal map beforehand.
        assert!(throttle.should_fire("proc:old:cpu"));
    }
}
