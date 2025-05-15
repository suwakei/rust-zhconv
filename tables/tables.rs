// /home/keito/work/rust-zhconv/src/tables.rs

use std::collections::HashMap;

pub struct ConversionTables {
    // ASCII_Z2H_CHARS is a Full Width ASCII characters map.
    pub ascii_z2h_chars_map: HashMap<char, char>,
    // ASCII_H2Z_CHARS is a Half Width ASCII characters map.
    pub ascii_h2z_chars_map: HashMap<char, char>,
    // KANA_Z2H_CHARS is a Full Width KANA characters map.
    pub kana_z2h_chars_map: HashMap<char, char>,
    // KANA_H2Z_CHARS is a Half Width KANA characters map.
    pub kana_h2z_chars_map: HashMap<char, char>,
    // DIGIT_Z2H_CHARS is a Full Width number characters map.
    pub digit_z2h_chars_map: HashMap<char, char>,
    // DIGIT_H2Z_CHARS is a Half Width number characters map.
    pub digit_h2z_chars_map: HashMap<char, char>,
    // KANA_TEN_MAP is a Full Width DAKUTEN_KANA characters map.
    pub kana_z2h_dakuten_map: HashMap<char, char>,
    // KANA_MARU_MAP is a Full Width HANDAKUTEN_KANA characters map.
    pub kana_z2h_maru_map: HashMap<char, char>,
    // KANA_H2Z_DAKUTEN_MAP is a Half Width DAKUTEN_KANA characters map.
    pub kana_h2z_dakuten_map: HashMap<char, char>,
    // KANA_H2Z_MARU_MAP is a Half Width HANDAKUTEN_KANA characters map.
    pub kana_h2z_maru_map: HashMap<char, char>,
}

impl ConversionTables {
    pub fn new() -> Self {
        ConversionTables {
            ascii_z2h_chars_map: Self::init_ascii_z2h_chars_map(),
            ascii_h2z_chars_map: Self::init_ascii_h2z_chars_map(),
            kana_z2h_chars_map: Self::init_kana_z2h_chars_map(),
            kana_h2z_chars_map: Self::init_kana_h2z_chars_map(),
            digit_z2h_chars_map: Self::init_digit_z2h_chars_map(),
            digit_h2z_chars_map: Self::init_digit_h2z_chars_map(),
            kana_z2h_dakuten_map: Self::init_kana_z2h_dakuten_map(),
            kana_z2h_maru_map: Self::init_kana_z2h_maru_map(),
            kana_h2z_dakuten_map: Self::init_kana_h2z_dakuten_map(),
            kana_h2z_maru_map: Self::init_kana_h2z_maru_map(),
        }
    }

    fn init_ascii_h2z_chars_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('a', 'ａ'); map.insert('b', 'ｂ'); map.insert('c', 'ｃ'); map.insert('d', 'ｄ'); map.insert('e', 'ｅ'); map.insert('f', 'ｆ'); map.insert('g', 'ｇ'); map.insert('h', 'ｈ'); map.insert('i', 'ｉ'); map.insert('j', 'ｊ'); map.insert('k', 'ｋ'); map.insert('l', 'ｌ'); map.insert('m', 'ｍ'); map.insert('n', 'ｎ'); map.insert('o', 'ｏ'); map.insert('p', 'ｐ'); map.insert('q', 'ｑ'); map.insert('r', 'ｒ'); map.insert('s', 'ｓ'); map.insert('t', 'ｔ'); map.insert('u', 'ｕ'); map.insert('v', 'ｖ'); map.insert('w', 'ｗ'); map.insert('x', 'ｘ'); map.insert('y', 'ｙ'); map.insert('z', 'ｚ');
        map.insert('A', 'Ａ'); map.insert('B', 'Ｂ'); map.insert('C', 'Ｃ'); map.insert('D', 'Ｄ'); map.insert('E', 'Ｅ'); map.insert('F', 'Ｆ'); map.insert('G', 'Ｇ'); map.insert('H', 'Ｈ'); map.insert('I', 'Ｉ'); map.insert('J', 'Ｊ'); map.insert('K', 'Ｋ'); map.insert('L', 'Ｌ'); map.insert('M', 'Ｍ'); map.insert('N', 'Ｎ'); map.insert('O', 'Ｏ'); map.insert('P', 'Ｐ'); map.insert('Q', 'Ｑ'); map.insert('R', 'Ｒ'); map.insert('S', 'Ｓ'); map.insert('T', 'Ｔ'); map.insert('U', 'Ｕ'); map.insert('V', 'Ｖ'); map.insert('W', 'Ｗ'); map.insert('X', 'Ｘ'); map.insert('Y', 'Ｙ'); map.insert('Z', 'Ｚ');
        map.insert('!', '！'); map.insert('"', '”'); map.insert('#', '＃'); map.insert('$', '＄'); map.insert('%', '％'); map.insert('&', '＆'); map.insert('\'', '’'); map.insert('(', '（'); map.insert(')', '）'); map.insert('*', '＊'); map.insert('+', '＋'); map.insert(',', '，'); map.insert('-', '－'); map.insert('.', '．'); map.insert('/', '／'); map.insert(':', '：'); map.insert(';', '；'); map.insert('<', '＜'); map.insert('=', '＝'); map.insert('>', '＞'); map.insert('?', '？'); map.insert('@', '＠'); map.insert('[', '［'); map.insert('\\', '￥'); map.insert(']', '］'); map.insert('^', '＾');
        map.insert('_', '＿'); map.insert('`', '‘'); map.insert('{', '｛'); map.insert('|', '｜'); map.insert('}', '｝'); map.insert('~', '～'); map.insert(' ', '　'); map.insert('\\', '＼');
    }

    fn init_ascii_z2h_chars_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('ａ', 'a'); map.insert('ｂ', 'b'); map.insert('ｃ', 'c'); map.insert('ｄ', 'd'); map.insert('ｅ', 'e'); map.insert('ｆ', 'f'); map.insert('ｇ', 'g'); map.insert('ｈ', 'h'); map.insert('ｉ', 'i'); map.insert('ｊ', 'j'); map.insert('ｋ', 'k'); map.insert('ｌ', 'l'); map.insert('ｍ', 'm'); map.insert('ｎ', 'n'); map.insert('ｏ', 'o'); map.insert('ｐ', 'p'); map.insert('ｑ', 'q'); map.insert('ｒ', 'r'); map.insert('ｓ', 's'); map.insert('ｔ', 't'); map.insert('ｕ', 'u'); map.insert('ｖ', 'v'); map.insert('ｗ', 'w'); map.insert('ｘ', 'x'); map.insert('ｙ', 'y'); map.insert('ｚ', 'z');
        map.insert('Ａ', 'A'); map.insert('Ｂ', 'B'); map.insert('Ｃ', 'C'); map.insert('Ｄ', 'D'); map.insert('Ｅ', 'E'); map.insert('Ｆ', 'F'); map.insert('Ｇ', 'G'); map.insert('Ｈ', 'H'); map.insert('Ｉ', 'I'); map.insert('Ｊ', 'J'); map.insert('Ｋ', 'K'); map.insert('Ｌ', 'L'); map.insert('Ｍ', 'M'); map.insert('Ｎ', 'N'); map.insert('Ｏ', 'O'); map.insert('Ｐ', 'P'); map.insert('Ｑ', 'Q'); map.insert('Ｒ', 'R'); map.insert('Ｓ', 'S'); map.insert('Ｔ', 'T'); map.insert('Ｕ', 'U'); map.insert('Ｖ', 'V'); map.insert('Ｗ', 'W'); map.insert('Ｘ', 'X'); map.insert('Ｙ', 'Y'); map.insert('Ｚ', 'Z');
        map.insert('！', '!'); map.insert('”', '"'); map.insert('＃', '#'); map.insert('＄', '$'); map.insert('％', '%'); map.insert('＆', '&'); map.insert('’', '\''); map.insert('（', '('); map.insert('）', ')'); map.insert('＊', '*'); map.insert('＋', '+'); map.insert('，', ','); map.insert('－', '-'); map.insert('．', '.'); map.insert('／', '/'); map.insert('：', ':'); map.insert('；', ';'); map.insert('＜', '<'); map.insert('＝', '='); map.insert('＞', '>'); map.insert('？', '?'); map.insert('＠', '@'); map.insert('［', '['); map.insert('￥', '\\'); map.insert('］', ']'); map.insert('＾', '^');
        map.insert('＿', '_'); map.insert('‘', '`'); map.insert('｛', '{'); map.insert('｜', '|'); map.insert('｝', '}'); map.insert('～', '~'); map.insert('　', ' ');
        map
    }

    fn init_kana_h2z_chars_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('ｱ', 'ア'); map.insert('ｲ', 'イ'); map.insert('ｳ', 'ウ'); map.insert('ｴ', 'エ'); map.insert('ｵ', 'オ');
        map.insert('ｶ', 'カ'); map.insert('ｷ', 'キ'); map.insert('ｸ', 'ク'); map.insert('ｹ', 'ケ'); map.insert('ｺ', 'コ');
        map.insert('ｻ', 'サ'); map.insert('ｼ', 'シ'); map.insert('ｽ', 'ス'); map.insert('ｾ', 'セ'); map.insert('ｿ', 'ソ');
        map.insert('ﾀ', 'タ'); map.insert('ﾁ', 'チ'); map.insert('ﾂ', 'ツ'); map.insert('ﾃ', 'テ'); map.insert('ﾄ', 'ト');
        map.insert('ﾅ', 'ナ'); map.insert('ﾆ', 'ニ'); map.insert('ﾇ', 'ヌ'); map.insert('ﾈ', 'ネ'); map.insert('ﾉ', 'ノ');
        map.insert('ﾊ', 'ハ'); map.insert('ﾋ', 'ヒ'); map.insert('ﾌ', 'フ'); map.insert('ﾍ', 'ヘ'); map.insert('ﾎ', 'ホ');
        map.insert('ﾏ', 'マ'); map.insert('ﾐ', 'ミ'); map.insert('ﾑ', 'ム'); map.insert('ﾒ', 'メ'); map.insert('ﾓ', 'モ');
        map.insert('ﾔ', 'ヤ'); map.insert('ﾕ', 'ユ'); map.insert('ﾖ', 'ヨ');
        map.insert('ﾗ', 'ラ'); map.insert('ﾘ', 'リ'); map.insert('ﾙ', 'ル'); map.insert('ﾚ', 'レ'); map.insert('ﾛ', 'ロ');
        map.insert('ﾜ', 'ワ'); map.insert('ｦ', 'ヲ'); map.insert('ﾝ', 'ン');
        map.insert('ｧ', 'ァ'); map.insert('ｨ', 'ィ'); map.insert('ｩ', 'ゥ'); map.insert('ｪ', 'ェ'); map.insert('ｫ', 'ォ');
        map.insert('ｯ', 'ッ');
        map.insert('ｬ', 'ャ'); map.insert('ｭ', 'ュ'); map.insert('ｮ', 'ョ');
        map.insert('｡', '。'); map.insert('､', '、'); map.insert('･', '・'); map.insert('ﾞ', '゛'); map.insert('ﾟ', '゜');
        map.insert('｢', '「'); map.insert('｣', '」'); map.insert('ｰ', 'ー');
        map
    }

    fn init_kana_z2h_chars_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('ア', 'ｱ'); map.insert('イ', 'ｲ'); map.insert('ウ', 'ｳ'); map.insert('エ', 'ｴ'); map.insert('オ', 'ｵ');
        map.insert('カ', 'ｶ'); map.insert('キ', 'ｷ'); map.insert('ク', 'ｸ'); map.insert('ケ', 'ｹ'); map.insert('コ', 'ｺ');
        map.insert('サ', 'ｻ'); map.insert('シ', 'ｼ'); map.insert('ス', 'ｽ'); map.insert('セ', 'ｾ'); map.insert('ソ', 'ｿ');
        map.insert('タ', 'ﾀ'); map.insert('チ', 'ﾁ'); map.insert('ツ', 'ﾂ'); map.insert('テ', 'ﾃ'); map.insert('ト', 'ﾄ');
        map.insert('ナ', 'ﾅ'); map.insert('ニ', 'ﾆ'); map.insert('ヌ', 'ﾇ'); map.insert('ネ', 'ﾈ'); map.insert('ノ', 'ﾉ');
        map.insert('ハ', 'ﾊ'); map.insert('ヒ', 'ﾋ'); map.insert('フ', 'ﾌ'); map.insert('ヘ', 'ﾍ'); map.insert('ホ', 'ﾎ');
        map.insert('マ', 'ﾏ'); map.insert('ミ', 'ﾐ'); map.insert('ム', 'ﾑ'); map.insert('メ', 'ﾒ'); map.insert('モ', 'ﾓ');
        map.insert('ヤ', 'ﾔ'); map.insert('ユ', 'ﾕ'); map.insert('ヨ', 'ﾖ');
        map.insert('ラ', 'ﾗ'); map.insert('リ', 'ﾘ'); map.insert('ル', 'ﾙ'); map.insert('レ', 'ﾚ'); map.insert('ロ', 'ﾛ');
        map.insert('ワ', 'ﾜ'); map.insert('ヲ', 'ｦ'); map.insert('ン', 'ﾝ');
        map.insert('ァ', 'ｧ'); map.insert('ィ', 'ｨ'); map.insert('ゥ', 'ｩ'); map.insert('ェ', 'ｪ'); map.insert('ォ', 'ｫ');
        map.insert('ッ', 'ｯ');
        map.insert('ャ', 'ｬ'); map.insert('ュ', 'ｭ'); map.insert('ョ', 'ｮ');
        map.insert('。', '｡'); map.insert('、', '､'); map.insert('・', '･'); map.insert('゛', 'ﾞ'); map.insert('゜', 'ﾟ');
        map.insert('「', '｢'); map.insert('」', '｣'); map.insert('ー', 'ｰ');
        map
    }

    fn init_digit_h2z_chars_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('0', '０'); map.insert('1', '１'); map.insert('2', '２'); map.insert('3', '３'); map.insert('4', '４');
        map.insert('5', '５'); map.insert('6', '６'); map.insert('7', '７'); map.insert('8', '８'); map.insert('9', '９');
        map
    }

    fn init_digit_z2h_chars_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('０', '0'); map.insert('１', '1'); map.insert('２', '2'); map.insert('３', '3'); map.insert('４', '4');
        map.insert('５', '5'); map.insert('６', '6'); map.insert('７', '7'); map.insert('８', '8'); map.insert('９', '9');
        map
    }

    fn init_kana_h2z_dakuten_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('ｶﾞ', 'ガ'); map.insert('ｷﾞ', 'ギ'); map.insert('ｸﾞ', 'グ'); map.insert('ｹﾞ', 'ゲ'); map.insert('ｺﾞ', 'ゴ');
        map.insert('ｻﾞ', 'ザ'); map.insert('ｼﾞ', 'ジ'); map.insert('ｽﾞ', 'ズ'); map.insert('ｾﾞ', 'ゼ'); map.insert('ｿﾞ', 'ゾ');
        map.insert('ﾀﾞ', 'ダ'); map.insert('ﾁﾞ', 'ヂ'); map.insert('ﾂﾞ', 'ヅ'); map.insert('ﾃﾞ', 'デ'); map.insert('ﾄﾞ', 'ド');
        map.insert('ﾊﾞ', 'バ'); map.insert('ﾋﾞ', 'ビ'); map.insert('ﾌﾞ', 'ブ'); map.insert('ﾍﾞ', 'ベ'); map.insert('ﾎﾞ', 'ボ');
        map.insert('ｳﾞ', 'ヴ');
        map
    }

        fn init_kana_z2h_dakuten_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('ガ', 'ｶﾞ'); map.insert('ギ', 'ｷﾞ'); map.insert('グ', 'ｸﾞ'); map.insert('ゲ', 'ｹﾞ'); map.insert('ゴ', 'ｺﾞ');
        map.insert('ザ', 'ｻﾞ'); map.insert('ジ', 'ｼﾞ'); map.insert('ズ', 'ｽﾞ'); map.insert('ゼ', 'ｾﾞ'); map.insert('ゾ', 'ｿﾞ');
        map.insert('ダ', 'ﾀﾞ'); map.insert('ヂ', 'ﾁﾞ'); map.insert('ヅ', 'ﾂﾞ'); map.insert('デ', 'ﾃﾞ'); map.insert('ド', 'ﾄﾞ');
        map.insert('バ', 'ﾊﾞ'); map.insert('ビ', 'ﾋﾞ'); map.insert('ブ', 'ﾌﾞ'); map.insert('ベ', 'ﾍﾞ'); map.insert('ボ', 'ﾎﾞ');
        map.insert('ヴ', 'ｳﾞ');
        map
    }

    fn init_kana_h2z_maru_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('ﾊﾟ', 'パ'); map.insert('ﾋﾟ', 'ピ'); map.insert('ﾌﾟ', 'プ'); map.insert('ﾍﾟ', 'ペ'); map.insert('ﾎﾟ', 'ポ');
        map
    }

    fn init_kana_z2h_maru_map() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('パ', 'ﾊﾟ'); map.insert('ピ', 'ﾋﾟ'); map.insert('プ', 'ﾌﾟ'); map.insert('ペ', 'ﾍﾟ'); map.insert('ポ', 'ﾎﾟ');
        map
    }
}