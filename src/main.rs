#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("invalid arguments");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut arg = args.get(1).unwrap().chars().peekable();
    let mut num = 0;
    let mut op: Option<Operator> = None;
    loop {
        let peek = arg.peek();
        if let Some(' ') = peek {
            arg.next();
            continue;
        }

        if peek.map_or(false, |v| v.is_digit(10)) {
            num *= 10;
            num += arg.next().unwrap().to_digit(10).unwrap();
            continue;
        }

        match arg.peek() {
            Some('+') | Some('-') => {
                gen(&op, num);

                num = 0;
                op = arg.next().map(|c| {
                    if c == '+' {
                        Operator::Plus
                    } else {
                        Operator::Minus
                    }
                });
            }
            None => {
                gen(&op, num);
                break;
            }
            _ => panic!(""),
        }
    }

    println!("  ret");
}

fn gen(op: &Option<Operator>, num: u32) {
    match op {
        None => println!("  mov rax, {}", num),
        Some(Operator::Plus) => println!("  add rax, {}", num),
        Some(Operator::Minus) => println!("  sub rax, {}", num),
    }
}
