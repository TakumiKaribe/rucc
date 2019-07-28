fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let arg = args[1]
        .parse::<i32>()
        .unwrap_or_else(|e| panic!("引数が整数ではありません{}", e));

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", arg);
    println!("  ret");
}
