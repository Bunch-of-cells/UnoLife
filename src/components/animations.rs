pub fn sin_bounce(target: f64, factor: f64, time_passed: f64) -> Option<f64> {
    if time_passed < 0.0 || time_passed > target * factor {
        None
    } else {
        Some((target - time_passed / factor) * (0.25 * (time_passed / factor + 18.0)).sin() + target)
    }
}