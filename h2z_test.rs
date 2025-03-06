#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn benchmark_h2z(b: &mut test::Bencher) {
        let input = "ｱｲｳｴｵ123ｶﾞｷﾞｸﾞｹﾞｺﾞﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟHello, World!";
        b.iter(|| {
            h2z(input);
        });
    }

    #[test]
    fn test_h2z() {
        let tests = vec![
            ("", ""),
            ("ｱｲｳｴｵ", "アイウエオ"),
            ("123", "１２３"),
            ("ｶﾞｷﾞｸﾞｹﾞｺﾞ", "ガギグゲゴ"),
            ("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ", "パピプペポ"),
            ("Hello, World!", "Ｈｅｌｌｏ，　Ｗｏｒｌｄ！"),
        ];

        for (input, expected) in tests {
            let result = h2z(input);
            assert_eq!(result, expected);
        }
    }

    // Dummy implementation of h2z function for demonstration purposes
    fn h2z(input: &str) -> String {
        input.to_string() // Replace this with the actual implementation
    }
}
