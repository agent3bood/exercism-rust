use chrono::{DateTime, Utc, Duration};
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let d: Duration = Duration::seconds(1_000_000_000);
    let end = start.checked_add_signed(d);
    end.unwrap()
}
