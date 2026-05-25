#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::history::MetricHistory;
    use crate::trend::{detect_trend, Trend};

    fn make_history(values: &[f64]) -> MetricHistory {
        let mut h = MetricHistory::new(Duration::from_secs(300));
        for &v in values {
            h.push(v);
        }
        h
    }

    #[test]
    fn test_rising_trend() {
        let h = make_history(&[10.0, 20.0, 30.0, 40.0, 50.0]);
        assert_eq!(detect_trend(&h, 1.0), Trend::Rising);
    }

    #[test]
    fn test_falling_trend() {
        let h = make_history(&[50.0, 40.0, 30.0, 20.0, 10.0]);
        assert_eq!(detect_trend(&h, 1.0), Trend::Falling);
    }

    #[test]
    fn test_stable_trend() {
        let h = make_history(&[25.0, 25.1, 24.9, 25.0, 25.1]);
        assert_eq!(detect_trend(&h, 1.0), Trend::Stable);
    }

    #[test]
    fn test_single_sample_is_stable() {
        let h = make_history(&[99.0]);
        assert_eq!(detect_trend(&h, 0.5), Trend::Stable);
    }

    #[test]
    fn test_empty_is_stable() {
        let h = make_history(&[]);
        assert_eq!(detect_trend(&h, 0.5), Trend::Stable);
    }

    #[test]
    fn test_threshold_boundary() {
        // slope ≈ 2.0 per step
        let h = make_history(&[0.0, 2.0, 4.0, 6.0, 8.0]);
        assert_eq!(detect_trend(&h, 2.5), Trend::Stable);
        assert_eq!(detect_trend(&h, 1.5), Trend::Rising);
    }

    #[test]
    fn test_two_samples_rising() {
        // Ensure the minimum viable case (two samples) is handled correctly
        let h = make_history(&[1.0, 10.0]);
        assert_eq!(detect_trend(&h, 1.0), Trend::Rising);
    }

    #[test]
    fn test_two_samples_falling() {
        let h = make_history(&[10.0, 1.0]);
        assert_eq!(detect_trend(&h, 1.0), Trend::Falling);
    }
}
