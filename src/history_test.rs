#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread::sleep;

    use crate::history::MetricHistory;
    use crate::history_store::HistoryStore;

    #[test]
    fn test_average_single_sample() {
        let mut h = MetricHistory::new(Duration::from_secs(60));
        h.push(42.0);
        assert_eq!(h.average(), Some(42.0));
    }

    #[test]
    fn test_average_multiple_samples() {
        let mut h = MetricHistory::new(Duration::from_secs(60));
        h.push(10.0);
        h.push(20.0);
        h.push(30.0);
        let avg = h.average().unwrap();
        assert!((avg - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_max() {
        let mut h = MetricHistory::new(Duration::from_secs(60));
        h.push(5.0);
        h.push(99.0);
        h.push(3.0);
        assert_eq!(h.max(), Some(99.0));
    }

    #[test]
    fn test_empty_history() {
        let h = MetricHistory::new(Duration::from_secs(60));
        assert!(h.is_empty());
        assert_eq!(h.average(), None);
        assert_eq!(h.max(), None);
    }

    #[test]
    fn test_eviction_after_window() {
        let mut h = MetricHistory::new(Duration::from_millis(50));
        h.push(100.0);
        sleep(Duration::from_millis(80));
        // Push a new value to trigger eviction
        h.push(1.0);
        assert_eq!(h.len(), 1);
        assert_eq!(h.average(), Some(1.0));
    }

    #[test]
    fn test_store_record_and_retrieve() {
        let mut store = HistoryStore::new(Duration::from_secs(60));
        store.record(1234, "cpu", 55.0);
        store.record(1234, "cpu", 65.0);
        let hist = store.get(1234, "cpu").expect("history should exist");
        assert_eq!(hist.len(), 2);
    }

    #[test]
    fn test_store_remove_pid() {
        let mut store = HistoryStore::new(Duration::from_secs(60));
        store.record(42, "cpu", 10.0);
        store.record(42, "mem", 200.0);
        store.record(99, "cpu", 5.0);
        store.remove_pid(42);
        assert!(store.get(42, "cpu").is_none());
        assert!(store.get(99, "cpu").is_some());
        assert_eq!(store.entry_count(), 1);
    }
}
