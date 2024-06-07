fn main() {
    let module = hf_parser::parse("main".to_string(), r#"
:test{
    -
}
:main{
    +
    @test
}"#.to_string()).unwrap();
    // println!("{:#?}", module);
    let machine_code = hf_codegen::gen_code(hf_codegen::Target::X86_64, 0x1234_5678, vec![module]).unwrap();
    // println!("code: {:?}", machine_code);
    println!("code:\n{}", machine_code.into_iter().map(|x| format!("0x{:x}", x)).collect::<Vec<String>>().join(", "));
}
