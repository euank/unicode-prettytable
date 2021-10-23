use unicode_prettytable::TableBuilder;

#[test]
fn test_cases() {
    struct Case {
        input: Vec<Vec<&'static str>>,
        output: &'static str,
    }

    let cases = vec![
        Case{
            input: vec![vec!["foo", "bar"]],
            output: "┌───┬───┐\n│foo│bar│\n└───┴───┘",
        },
        Case{
            input: vec![vec!["💩💩💩💩", "🕴️🕴️🕴️🕴️"]],
            output: "┌────┬────────┐\n│💩💩💩💩│🕴\u{fe0f}🕴\u{fe0f}🕴\u{fe0f}🕴\u{fe0f}│\n└────┴────────┘",
        },
    ];

    for tc in cases {
        let output = TableBuilder::default()
            .rows(&tc.input)
            .build().unwrap();

        assert_eq!(tc.output, format!("{}", output));
    }

}
