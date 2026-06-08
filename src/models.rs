pub struct Track {
    pub name: String,
    pub duration: f32,
    pub location: String,
    pub bytes: u64,
}
impl Track {
    pub fn formatted_duration(&self) -> String {
        let total = self.duration as u32;
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        let seconds = total % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
    pub fn formatted_size(&self) -> String {
        match self.bytes {
            b if b < 1_024 => format!("{b} B"),
            b if b < 1_048_576 => format!("{:.1} KB", b as f64 / 1_024.0),
            b => format!("{:.1} MB", b as f64 / 1_048_576.0),
        }
    }
}
