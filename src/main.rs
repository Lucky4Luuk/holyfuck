fn main() {
    let module = hf_parser::parse("main".to_string(), r#"
:test{
    [-]
}
:main{
    +
    @test
}"#.to_string()).unwrap();
    // println!("{:#?}", module);
    let machine_code = hf_codegen::gen_code(hf_codegen::Target::X86_64, 0x1234_5678, vec![module]).unwrap();
    // println!("code: {:?}", machine_code);
    // println!("code:\n{}", machine_code.into_iter().map(|x| format!("0x{:x}", x)).collect::<Vec<String>>().join(", "));
    println!("Running code, brace yourself...");
    unsafe { run_code(machine_code); }
}

unsafe fn run_code(code: Vec<u8>) {
    use mmap_rs::MmapOptions;
    let mapping = MmapOptions::new(MmapOptions::page_size())
        .unwrap()
        .map()
        .unwrap();
    let mut mmap_mut = mapping.make_mut().unwrap();
    for i in 0..code.len() {
        mmap_mut[i] = code[i];
    }
    let map = mmap_mut.make_exec().unwrap();
    let func: unsafe extern "C" fn() = std::mem::transmute(map.as_ptr());
    func();
    println!("you lived!");
}
