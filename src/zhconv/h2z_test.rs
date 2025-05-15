#[cfg(test)]
mod tests {
    use super::*;
    use crate::tables::ConversionTables;

    #[test]
    fn test_h2z() {
        let tables = ConversionTables::new();
        let tests = vec![
            ("", ""),
            ("ｱｲｳｴｵ", "アイウエオ"),
            ("123", "１２３"),
            ("ｶﾞｷﾞｸﾞｹﾞｺﾞ", "ガギグゲゴ"), // 元のキーが 'ｶ' なので、ここでは "ｶﾞ" ではなく "カ" になるはず。
            ("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ", "パピプペポ"), // 元のキーが 'ﾊ' なので、ここでは "ﾊﾟ" ではなく "パ" になるはず。
            ("Hello, World!", "Ｈｅｌｌｏ，　Ｗｏｒｌｄ！"),
        ];

        for (input, expected) in tests {
            let result = h2z(input, &tables);
            assert_eq!(result, expected, "Input: {}", input);
        }
    }
}