pub struct Track {
    pub id: u32,
    pub name: String,
    pub duration: f32,
    pub location: String,
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
}
