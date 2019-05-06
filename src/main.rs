fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let arg: i32 = match args[1].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("引数が整数ではありません\n{}", err);
            std::process::exit(1);
        },
    };

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", arg);
    println!("  ret");
}
