use crate::history::MetricHistory;

/// Direction of a detected trend.
#[derive(Debug, Clone, PartialEq)]
pub enum Trend {
    Rising,
    Falling,
    Stable,
}

/// Analyse the recent samples in a `MetricHistory` and return a trend.
///
/// Uses a simple linear regression slope over all samples in the window.
/// `threshold` is the minimum absolute slope (units/sample) to be considered
/// Rising or Falling rather than Stable.
pub fn detect_trend(history: &MetricHistory, threshold: f64) -> Trend {
    let samples: Vec<f64> = history
        .samples
        .iter()
        .map(|s| s.value)
        .collect();

    let n = samples.len();
    if n < 2 {
        return Trend::Stable;
    }

    let slope = linear_slope(&samples);

    if slope > threshold {
        Trend::Rising
    } else if slope < -threshold {
        Trend::Falling
    } else {
        Trend::Stable
    }
}

/// Compute the slope of a simple linear regression (y = a + b*x) over
/// evenly-spaced indices 0..n.
fn linear_slope(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let x_mean = (n - 1.0) / 2.0;
    let y_mean: f64 = values.iter().sum::<f64>() / n;

    let numerator: f64 = values
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f64 - x_mean) * (y - y_mean))
        .sum();

    let denominator: f64 = values
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let dx = i as f64 - x_mean;
            dx * dx
        })
        .sum();

    if denominator == 0.0 { 0.0 } else { numerator / denominator }
}
