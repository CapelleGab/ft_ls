use std::os::unix::fs::MetadataExt;

use crate::entry::FileEntry;

pub fn display_short(entries: &[FileEntry]) {
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    println!("{}", names.join("  "));
}

pub fn display_long(entries: &[FileEntry]) {
    for entry in entries {
        let meta = &entry.metadata;
        let permissions = format_permissions(meta.mode());
        let nlinks = meta.nlink();
        let size = meta.size();
        let modified = format_time(entry.modified_time());

        println!(
            "{} {:>3} {:>8} {}  {}",
            permissions, nlinks, size, modified, entry.name
        );
    }
}

fn format_permissions(mode: u32) -> String {
    let file_type = match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        _ => '-',
    };

    let perms = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];

    let mut s = String::with_capacity(10);
    s.push(file_type);
    for (bit, ch) in perms {
        s.push(if mode & bit != 0 { ch } else { '-' });
    }
    s
}

fn format_time(time: std::time::SystemTime) -> String {
    let duration = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs() as i64;

    let months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];

    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;

    let (year, month, day) = days_to_date(days_since_epoch);
    let month_str = months[month as usize];

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let six_months = 180 * 86400;
    if (now_secs - secs).abs() > six_months {
        format!("{} {:>2}  {:>4}", month_str, day, year)
    } else {
        format!("{} {:>2} {:02}:{:02}", month_str, day, hours, minutes)
    }
}

fn days_to_date(days: i64) -> (i64, i64, i64) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1461 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m - 1, d)
}
