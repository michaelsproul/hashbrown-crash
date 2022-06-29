use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct Input {
    hello: String
}

fn main() {
    let input = r#"hello = "world""#;

    let output1: Input = toml::from_str(input).unwrap();
    let output2: Input = toml::from_str(input).unwrap();

    assert_eq!(output1, output2);
}
