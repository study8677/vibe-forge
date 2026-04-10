use exif::{In, Reader, Tag, Exif};
use std::io::BufReader;
use std::path::Path;

/// Extract EXIF date-taken and orientation from an image file.
/// Returns `(taken_at, orientation)`.  Falls back to `(None, 1)` on error.
pub fn extract_exif(path: &Path) -> (Option<String>, i32) {
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return (None, 1),
    };

    let exif: Exif = match Reader::new().read_from_container(&mut BufReader::new(file)) {
        Ok(e) => e,
        Err(_) => return (None, 1),
    };

    let taken_at = exif
        .get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .or_else(|| exif.get_field(Tag::DateTime, In::PRIMARY))
        .and_then(|f: &exif::Field| {
            let raw = f.display_value().with_unit(&exif).to_string();
            parse_exif_date(&raw)
        });

    let orientation = exif
        .get_field(Tag::Orientation, In::PRIMARY)
        .and_then(|f| f.value.get_uint(0))
        .map(|v| v as i32)
        .unwrap_or(1);

    (taken_at, orientation)
}

/// Convert EXIF date strings like `"2024:03:15 14:30:00"` or
/// `"2024-03-15 14:30:00"` into `"2024-03-15T14:30:00"`.
fn parse_exif_date(raw: &str) -> Option<String> {
    let s = raw.trim().trim_matches('"');
    if s.len() < 19 {
        return None;
    }

    let bytes = s.as_bytes();
    // Positions: 0-3 year, 4 sep, 5-6 month, 7 sep, 8-9 day, 10 space, 11-18 time
    let year = &s[0..4];
    let month = &s[5..7];
    let day = &s[8..10];
    let time = &s[11..19];

    // Quick validity check
    if year.parse::<u16>().is_err()
        || month.parse::<u8>().is_err()
        || day.parse::<u8>().is_err()
    {
        return None;
    }

    // Avoid "0000:00:00 00:00:00" entries
    if bytes[0..4] == *b"0000" {
        return None;
    }

    Some(format!("{year}-{month}-{day}T{time}"))
}
