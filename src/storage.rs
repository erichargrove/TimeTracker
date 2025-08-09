use crate::TimeTracker;
use std::fs;

pub fn save_to_file(
    tracker: &TimeTracker,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string_pretty(tracker)?;
    fs::write(filename, toml_string)?;
    Ok(())
}

pub fn load_from_file(filename: &str) -> Result<TimeTracker, Box<dyn std::error::Error>> {
    if !std::path::Path::new(filename).exists() {
        return Ok(TimeTracker::new());
    }

    let contents = fs::read_to_string(filename)?;
    let tracker: TimeTracker = toml::from_str(&contents)?;
    Ok(tracker)
}
