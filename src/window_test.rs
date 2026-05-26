#[cfg(test)]
mod tests {
    use super::super::window::SlidingWindow;
    use std::time::{Duration, Instant};

    #[test]
    fn test_push_and_values() {
        let mut w: SlidingWindow<u32> = SlidingWindow::new(Duration::from_secs(60));
        w.push(1);
        w.push(2);
        w.push(3);
        let vals = w.values();
        assert_eq!(vals, vec![1, 2, 3]);
    }

    #[test]
    fn test_evicts_old_entries() {
        let mut w: SlidingWindow<u32> = SlidingWindow::new(Duration::from_secs(5));
        let old = Instant::now() - Duration::from_secs(10);
        let recent = Instant::now() - Duration::from_secs(1);
        w.push_at(old, 100);
        w.push_at(recent, 200);
        let vals = w.values();
        assert_eq!(vals, vec![200]);
    }

    #[test]
    fn test_len_reflects_live_entries() {
        let mut w: SlidingWindow<u32> = SlidingWindow::new(Duration::from_secs(5));
        let old = Instant::now() - Duration::from_secs(10);
        w.push_at(old, 1);
        w.push_at(old, 2);
        w.push(3);
        assert_eq!(w.len(), 1);
    }

    #[test]
    fn test_is_empty_when_all_evicted() {
        let mut w: SlidingWindow<u32> = SlidingWindow::new(Duration::from_secs(1));
        let old = Instant::now() - Duration::from_secs(5);
        w.push_at(old, 42);
        assert!(w.is_empty());
    }

    #[test]
    fn test_empty_window() {
        let mut w: SlidingWindow<f64> = SlidingWindow::new(Duration::from_secs(30));
        assert!(w.is_empty());
        assert_eq!(w.values(), Vec::<f64>::new());
    }

    #[test]
    fn test_window_size_accessor() {
        let w: SlidingWindow<u8> = SlidingWindow::new(Duration::from_secs(120));
        assert_eq!(w.window_size(), Duration::from_secs(120));
    }

    #[test]
    fn test_multiple_evictions_preserve_order() {
        let mut w: SlidingWindow<u32> = SlidingWindow::new(Duration::from_secs(10));
        let base = Instant::now();
        w.push_at(base - Duration::from_secs(15), 1);
        w.push_at(base - Duration::from_secs(5), 2);
        w.push_at(base - Duration::from_secs(3), 3);
        w.push_at(base - Duration::from_secs(1), 4);
        let vals = w.values();
        assert_eq!(vals, vec![2, 3, 4]);
    }
}
