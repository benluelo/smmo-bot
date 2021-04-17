use chrono::Duration;

pub fn duration_pretty(d: Duration) -> String {
    let h = d.num_hours() % 24;
    let m = d.num_minutes() % 60;
    let s = d.num_seconds() % 60;
    if d.num_days() > 0 {
        let days = d.num_days();
        format!("{} days, {} hours {} mins {} secs.", days, h, m, s)
    } else {
        format!("{} hours {} mins {} secs.", h, m, s)
    }
}
