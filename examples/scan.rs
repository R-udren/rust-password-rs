fn main() {
    tracing_subscriber::fmt::init();

    match rust_password::scan() {
        Ok(data) => {
            println!("Last code: {}", data.last_code);
            println!("Console history:");
            for command in data.history {
                println!("  {command}");
            }
        }
        Err(error) => {
            tracing::error!(%error, "scan failed");
            std::process::exit(1);
        }
    }
}
