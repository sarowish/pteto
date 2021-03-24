pub fn seconds_to_clock(seconds: u32) -> String {
    let t = &[
        seconds / 3600,      // hours
        seconds % 3600 / 60, //minutes
        seconds % 60,        //seconds
    ];

    let mut disp = String::new();
    let mut flag = false;

    for &i in &t[..2] {
        if !flag && i != 0 {
            flag = true;
        }
        if flag {
            disp.push_str(&format!("{:02}:", i));
        }
    }
    disp.push_str(&format!("{:02}", t[2]));
    disp
}
