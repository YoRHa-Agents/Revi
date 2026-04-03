use common_cli::{execute, parse_cli_args};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let invocation = match parse_cli_args(&args) {
        Ok(invocation) => invocation,
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("usage:");
            eprintln!("  common dev [--surface <web|desktop|docs|demo>] [--host HOST] [--port PORT]");
            eprintln!("  common demo");
            eprintln!("  common docs");
            eprintln!("  common release");
            std::process::exit(2);
        }
    };

    match execute(&invocation) {
        Ok(bundle) => {
            for message in bundle.messages {
                println!("{message}");
            }
        }
        Err(error) => {
            eprintln!("dispatch error: {error}");
            std::process::exit(1);
        }
    }
}
