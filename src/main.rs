fn main() {
    println!("{:#?}", hf_parser::parse(r#"
:main{
    +++
}"#.to_string()));
}
