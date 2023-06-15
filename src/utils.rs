pub fn pretty_size(bytes: u64) -> String {
    let kilo = 1024;
    let mega = kilo * kilo;
    let giga = mega * kilo;
    if bytes >= giga {
        format!("{:.1}GB", bytes as f64 / giga as f64)
    } else if bytes >= mega {
        format!("{:.1}MB", bytes as f64 / mega as f64)
    } else if bytes >= kilo {
        format!("{:.1}KB", bytes as f64 / kilo as f64)
    } else {
        format!("{}B", bytes)
    }
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}
