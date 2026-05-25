#[cfg(test)]
mod tests {
    use super::super::heartbeat::{Heartbeat, HeartbeatStatus};
    use std::thread;
    use std::time::Duration;

    fn make_heartbeat(interval_ms: u64, max_missed: u32) -> Heartbeat {
        Heartbeat::new("test-component", Duration::from_millis(interval_ms), max_missed)
    }

    #[test]
    fn test_initial_status_is_stale() {
        let mut hb = make_heartbeat(100, 3);
        assert_eq!(hb.status(), HeartbeatStatus::Stale { missed: 0 });
    }

    #[test]
    fn test_alive_after_beat() {
        let mut hb = make_heartbeat(500, 3);
        hb.beat();
        assert_eq!(hb.status(), HeartbeatStatus::Alive);
    }

    #[test]
    fn test_is_alive_returns_true_after_beat() {
        let mut hb = make_heartbeat(500, 3);
        hb.beat();
        assert!(hb.is_alive());
    }

    #[test]
    fn test_stale_after_interval_elapsed() {
        let mut hb = make_heartbeat(50, 5);
        hb.beat();
        thread::sleep(Duration::from_millis(110));
        match hb.status() {
            HeartbeatStatus::Stale { missed } => assert!(missed >= 1),
            other => panic!("Expected Stale, got {:?}", other),
        }
    }

    #[test]
    fn test_dead_when_max_missed_exceeded() {
        let mut hb = make_heartbeat(30, 2);
        hb.beat();
        thread::sleep(Duration::from_millis(100));
        assert_eq!(hb.status(), HeartbeatStatus::Dead);
    }

    #[test]
    fn test_missed_count_resets_on_beat() {
        let mut hb = make_heartbeat(30, 5);
        hb.beat();
        thread::sleep(Duration::from_millis(70));
        let _ = hb.status(); // trigger missed count update
        hb.beat();
        assert_eq!(hb.missed_count(), 0);
    }

    #[test]
    fn test_last_beat_is_none_initially() {
        let hb = make_heartbeat(100, 3);
        assert!(hb.last_beat().is_none());
    }

    #[test]
    fn test_last_beat_set_after_beat() {
        let mut hb = make_heartbeat(100, 3);
        hb.beat();
        assert!(hb.last_beat().is_some());
    }
}
