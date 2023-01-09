use std::fmt::Display;

use crate::{percolation::Percolatable, random::Random};

pub struct PercolationStats {
    pub counts: Vec<usize>, // of length width * height
    // counts[i] represents how many perculated after exactly i tiles opened
    mean: f64,            // mean of (open sites)/(width * height) to percolate
    stddev: f64,          // stddev of ratio
    confidence_low: f64,  // 95% threshhold
    confidence_high: f64, // 95% threshhold
}

impl Display for PercolationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "mean: {:.4}, stddev: {:.4}, 95% confidence interval: ({:.4}, {:.4})",
            self.mean, self.stddev, self.confidence_low, self.confidence_high
        )
    }
}

pub fn calculate_stats<P: Percolatable>(
    width: usize,
    height: usize,
    trials: usize,
    random: &mut Random,
) -> PercolationStats {
    let mut counts = Vec::new();
    counts.resize(width * height, 0);
    for _ in 0..trials {
        let mut percolation = P::new(width, height);
        while !percolation.percolates() {
            let row = random.next_below(height);
            let col = random.next_below(width);
            percolation.open(row, col);
        }
        counts[percolation.number_of_open_sites()] += 1;
    }
    let mut mean = 0.0;
    let mut stddev = 0.0;
    for (i, count) in counts.iter().enumerate() {
        let ratio = i as f64 / (width * height) as f64;
        mean += (ratio * *count as f64) / trials as f64;
    }
    for (i, count) in counts.iter().enumerate() {
        let ratio = i as f64 / (width * height) as f64;
        stddev += (ratio - mean).powi(2) * *count as f64;
    }
    stddev /= trials as f64 - 1.0;
    stddev = stddev.sqrt();

    let confidence_low = mean - 1.96 * stddev / (trials as f64).sqrt();
    let confidence_high = mean + 1.96 * stddev / (trials as f64).sqrt();
    PercolationStats {
        counts,
        mean,
        stddev,
        confidence_low,
        confidence_high,
    }
}
