fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    // TODO: safety get from args
    println!("  mov rax, {}", args[1]);
    println!("  ret");
}
