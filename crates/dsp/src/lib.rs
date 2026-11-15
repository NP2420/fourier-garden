//! Signal processing for Fourier Garden. Pure Rust, no Windows code.

/// Number of log-spaced frequency bands in a [`FeatureFrame`].
pub const NUM_BANDS: usize = 64;

/// One tick of analysed audio, in the shape every visual receives.
///
/// All levels are normalised to `0.0..=1.0`.
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureFrame {
    /// Seconds on the audio clock.
    pub t: f64,
    /// Overall loudness.
    pub rms: f32,
    /// 20-250 Hz, smoothed.
    pub bass: f32,
    /// 250 Hz-4 kHz, smoothed.
    pub mid: f32,
    /// 4-16 kHz, smoothed.
    pub treble: f32,
    /// Log-spaced bands, lowest frequency first.
    pub bands: [f32; NUM_BANDS],
    /// An onset was detected in this frame.
    pub beat: bool,
    /// Nothing has played for a moment.
    pub silent: bool,
}

impl FeatureFrame {
    /// A frame of silence at time `t`.
    pub fn silent_at(t: f64) -> Self {
        Self {
            t,
            rms: 0.0,
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            bands: [0.0; NUM_BANDS],
            beat: false,
            silent: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silent_frame_is_zeroed() {
        let frame = FeatureFrame::silent_at(1.5);
        assert!(frame.silent);
        assert!(!frame.beat);
        assert_eq!(frame.bands.len(), NUM_BANDS);
        assert!(frame.bands.iter().all(|&b| b == 0.0));
    }
}
