mod tables;
use tables::ConversionTables;

pub fn h2z(s: &str, tables: &ConversionTables) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if let Some(&full) = tables.ascii_h2z_chars_map.get(&c) {
            result.push(full);
            i += 1;
        } else if let Some(&full) = tables.kana_h2z_chars_map.get(&c) {
            result.push(full);
            i += 1;
        } else if let Some(&full) = tables.digit_h2z_chars_map.get(&c) {
            result.push(full);
            i += 1;
        } else if i + 1 < chars.len() {
            let next_c = chars[i + 1];
            let combined = String::from_iter(&[c, next_c]);
            if let Some(&full) = tables.kana_h2z_dakuten_map.get(&combined.chars().nth(0).unwrap()) {
                result.push(full);
                i += 2;
            } else if let Some(&full) = tables.kana_h2z_maru_map.get(&combined.chars().nth(0).unwrap()) {
                result.push(full);
                i += 2;
            } else {
                result.push(c);
                i += 1;
            }
        } else {
            result.push(c);
            i += 1;
        }
    }
    result
}