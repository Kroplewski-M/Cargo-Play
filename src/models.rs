#[derive(Clone, Default)]
pub struct Track {
    pub name: String,
    pub duration: f32,
    pub location: String,
    pub bytes: u64,
}
impl Track {
    pub fn formatted_duration(&self) -> String {
        formated_duration(self.duration as u64)
    }
    pub fn formatted_size(&self) -> String {
        match self.bytes {
            b if b < 1_024 => format!("{b} B"),
            b if b < 1_048_576 => format!("{:.1} KB", b as f64 / 1_024.0),
            b => format!("{:.1} MB", b as f64 / 1_048_576.0),
        }
    }
}
pub fn formated_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_omits_hours_when_zero() {
        let t = Track {
            duration: 90.0,
            ..Default::default()
        };
        assert_eq!(t.formatted_duration(), "01:30");
    }

    #[test]
    fn duration_includes_hours() {
        let t = Track {
            duration: 3661.0,
            ..Default::default()
        };
        assert_eq!(t.formatted_duration(), "01:01:01");
    }
}
