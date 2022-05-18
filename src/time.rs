use std::time::Duration;

pub fn duration_without_ns(dur: &std::time::Duration) -> std::time::Duration {
    Duration::from_millis(dur.as_millis() as u64)
}
