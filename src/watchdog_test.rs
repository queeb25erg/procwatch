#[cfg(test)]
mod tests {
    use super::super::watchdog::*;
    use std::time::Duration;

    #[test]
    fn test_register_and_len() {
        let mut wd = Watchdog::new(3);
        assert!(wd.is_empty());
        wd.register(1001, "nginx");
        wd.register(1002, "redis");
        assert_eq!(wd.len(), 2);
    }

    #[test]
    fn test_unregister() {
        let mut wd = Watchdog::new(3);
        wd.register(1001, "nginx");
        wd.unregister(1001);
        assert!(wd.is_empty());
    }

    #[test]
    fn test_touch_resets_missed_ticks() {
        let mut entry = WatchdogEntry::new(42, "myproc", 3);
        entry.missed_ticks = 2;
        entry.touch();
        assert_eq!(entry.missed_ticks, 0);
    }

    #[test]
    fn test_tick_ok_when_recently_seen() {
        let mut entry = WatchdogEntry::new(42, "myproc", 3);
        // last_seen is now, so elapsed < 5s, missed_ticks stays 0
        let status = entry.tick();
        assert_eq!(status, WatchdogStatus::Ok);
        assert_eq!(entry.missed_ticks, 0);
    }

    #[test]
    fn test_unresponsive_after_max_missed_ticks() {
        let mut entry = WatchdogEntry::new(42, "myproc", 2);
        entry.missed_ticks = 2;
        let status = entry.tick();
        // missed_ticks already at max before tick increments
        assert_eq!(status, WatchdogStatus::Unresponsive);
    }

    #[test]
    fn test_is_stale() {
        let entry = WatchdogEntry::new(42, "myproc", 3);
        assert!(!entry.is_stale(Duration::from_secs(60)));
        assert!(entry.is_stale(Duration::from_nanos(1)));
    }

    #[test]
    fn test_tick_all_returns_all_entries() {
        let mut wd = Watchdog::new(5);
        wd.register(100, "proc_a");
        wd.register(200, "proc_b");
        let results = wd.tick_all();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_unresponsive_pids_empty_when_all_ok() {
        let mut wd = Watchdog::new(5);
        wd.register(100, "proc_a");
        wd.touch(100);
        let bad = wd.unresponsive_pids();
        assert!(bad.is_empty());
    }

    #[test]
    fn test_touch_unknown_pid_does_not_panic() {
        let mut wd = Watchdog::new(3);
        // touching an unregistered pid should be a no-op
        wd.touch(9999);
        assert!(wd.is_empty());
    }
}
