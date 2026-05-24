#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::buffer::EventBuffer;
    use crate::buffer_manager::BufferManager;

    #[test]
    fn test_push_and_drain() {
        let mut buf = EventBuffer::new(10, Duration::from_secs(60));
        assert!(buf.push("event1".to_string()));
        assert!(buf.push("event2".to_string()));
        assert_eq!(buf.len(), 2);
        let drained = buf.drain_ready();
        assert_eq!(drained.len(), 2);
        assert_eq!(drained[0].payload, "event1");
        assert!(buf.is_empty());
    }

    #[test]
    fn test_capacity_limit() {
        let mut buf = EventBuffer::new(2, Duration::from_secs(60));
        assert!(buf.push("a".to_string()));
        assert!(buf.push("b".to_string()));
        assert!(buf.is_full());
        assert!(!buf.push("c".to_string()));
        assert_eq!(buf.len(), 2);
    }

    #[test]
    fn test_evict_expired() {
        let mut buf = EventBuffer::new(10, Duration::from_millis(1));
        buf.push("old".to_string());
        std::thread::sleep(Duration::from_millis(5));
        buf.evict_expired();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_event_age() {
        let mut buf = EventBuffer::new(5, Duration::from_secs(60));
        buf.push("x".to_string());
        let events = buf.drain_ready();
        assert!(events[0].age() < Duration::from_secs(1));
    }

    #[test]
    fn test_manager_push_and_drain() {
        let mut mgr = BufferManager::new(10, Duration::from_secs(60));
        mgr.push("proc_a", "alert1".to_string());
        mgr.push("proc_a", "alert2".to_string());
        mgr.push("proc_b", "alert3".to_string());
        let drained = mgr.drain("proc_a");
        assert_eq!(drained.len(), 2);
        assert_eq!(mgr.total_pending(), 1);
    }

    #[test]
    fn test_manager_drain_all() {
        let mut mgr = BufferManager::new(10, Duration::from_secs(60));
        mgr.push("x", "e1".to_string());
        mgr.push("y", "e2".to_string());
        let all = mgr.drain_all();
        assert_eq!(all.len(), 2);
        assert_eq!(mgr.total_pending(), 0);
    }

    #[test]
    fn test_manager_remove() {
        let mut mgr = BufferManager::new(5, Duration::from_secs(60));
        mgr.push("proc", "data".to_string());
        mgr.remove("proc");
        assert_eq!(mgr.drain("proc").len(), 0);
    }
}
