/// Seuils de scoring pour HourlyStats

/// ATR adapté Forex (30 pts)
pub const ATR_EXCELLENT: f64 = 0.00025;
pub const ATR_GOOD: f64 = 0.00015;
pub const ATR_FAIR: f64 = 0.00010;
pub const ATR_POOR: f64 = 0.00005;

/// Body Range réaliste (25 pts)
pub const BODY_RANGE_EXCELLENT: f64 = 45.0;
pub const BODY_RANGE_GOOD: f64 = 35.0;
pub const BODY_RANGE_FAIR: f64 = 25.0;
pub const BODY_RANGE_POOR: f64 = 15.0;

/// Volatilité (20 pts)
pub const VOL_EXCELLENT: f64 = 0.30;
pub const VOL_GOOD: f64 = 0.20;
pub const VOL_FAIR: f64 = 0.10;
pub const VOL_POOR: f64 = 0.05;

/// Noise Ratio (15 pts)
pub const NOISE_EXCELLENT: f64 = 2.0;
pub const NOISE_GOOD: f64 = 3.0;
pub const NOISE_FAIR: f64 = 4.0;

/// Breakout % (10 pts)
pub const BREAKOUT_EXCELLENT: f64 = 15.0;
pub const BREAKOUT_GOOD: f64 = 10.0;
pub const BREAKOUT_FAIR: f64 = 5.0;

/// Straddle: Range (60 pts max)
pub const RANGE_STRADDLE_EXCELLENT: f64 = 0.0025;
pub const RANGE_STRADDLE_GOOD: f64 = 0.0020;
pub const RANGE_STRADDLE_FAIR: f64 = 0.0015;
pub const RANGE_STRADDLE_POOR: f64 = 0.0010;

/// Straddle: ATR (25 pts max)
pub const ATR_STRADDLE_EXCELLENT: f64 = 0.0020;
pub const ATR_STRADDLE_GOOD: f64 = 0.0015;
pub const ATR_STRADDLE_FAIR: f64 = 0.0010;
pub const ATR_STRADDLE_POOR: f64 = 0.0005;

/// Straddle: BodyRange (15 pts max)
pub const BODY_RANGE_STRADDLE_EXCELLENT: f64 = 45.0;
pub const BODY_RANGE_STRADDLE_GOOD: f64 = 35.0;
pub const BODY_RANGE_STRADDLE_FAIR: f64 = 25.0;
pub const BODY_RANGE_STRADDLE_POOR: f64 = 15.0;
