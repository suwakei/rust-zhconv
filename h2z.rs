mod tables {
    pub struct ConversionTables {
        pub ASCII_HANKAKU_CHARS: Vec<char>,
        pub ASCII_ZENKAKU_CHARS: Vec<char>,
        pub KANA_HANKAKU_CHARS: Vec<char>,
        pub KANA_ZENKAKU_CHARS: Vec<char>,
        pub DIGIT_HANKAKU_CHARS: Vec<char>,
        pub KANA_TEN_MAP: std::collections::HashMap<char, char>,
        pub KANA_MARU_MAP: std::collections::HashMap<char, char>,
    }

    impl ConversionTables {
        pub fn new() -> Self {
            // ConversionTablesを適切な値で初期化
            Self {
                ASCII_HANKAKU_CHARS: vec![],
                ASCII_ZENKAKU_CHARS: vec![],
                KANA_HANKAKU_CHARS: vec![],
                KANA_ZENKAKU_CHARS: vec![],
                DIGIT_HANKAKU_CHARS: vec![],
                KANA_TEN_MAP: std::collections::HashMap::new(),
                KANA_MARU_MAP: std::collections::HashMap::new(),
            }
        }
    }
}

fn h2z(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let t = tables::ConversionTables::new();
    let chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let char = chars[i];
        if let Some(idx) = t.ASCII_HANKAKU_CHARS.iter().position(|&c| c == char) {
            result.push(t.ASCII_ZENKAKU_CHARS[idx]);
        } else if let Some(idx) = t.KANA_HANKAKU_CHARS.iter().position(|&c| c == char) {
            result.push(t.KANA_ZENKAKU_CHARS[idx]);
        } else if let Some(idx) = t.DIGIT_HANKAKU_CHARS.iter().position(|&c| c == char) {
            result.push(t.DIGIT_ZENKAKU_CHARS[idx]);
        } else if i + 1 < chars.len() && chars[i + 1] == 'ﾞ' {
            if let Some(&mapped_char) = t.KANA_TEN_MAP.get(&char) {
                result.push(mapped_char);
                i += 1; // 次の文字をスキップ
            } else {
                result.push(char);
            }
        } else if i + 1 < chars.len() && chars[i + 1] == 'ﾟ' {
            if let Some(&mapped_char) = t.KANA_MARU_MAP.get(&char) {
                result.push(mapped_char);
                i += 1; // 次の文字をスキップ
            } else {
                result.push(char);
            }
        } else {
            result.push(char);
        }
        i += 1;
    }
    result
}

fn main() {
    let input = "your_input_string_here";
    let output = h2z(input);
    println!("{}", output);
}
