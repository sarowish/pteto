pub fn length_as_hhmmss(length: u32) -> String {
    let seconds = length % 60;
    let minutes = (length / 60) % 60;
    let hours = length / 3600;

    match (hours, minutes, seconds) {
        (0, 0, _) => format!("{:02}", seconds),
        (0, _, _) => format!("{}:{:02}", minutes, seconds),
        _ => format!("{}:{:02}:{:02}", hours, minutes, seconds),
    }
}
