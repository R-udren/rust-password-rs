fn main() {
    tracing_subscriber::fmt::init();

    match rust_password::scan() {
        Ok(data) => {
            println!("Last code: {}", data.last_code);
            println!("Last Steam game: {}", data.steam.last_game);
            println!(
                "SteamID64: {}",
                data.steam
                    .steam_id
                    .map_or_else(|| "None".to_owned(), |id| id.to_string())
            );
            println!("Game: {}", data.rust.name);
            println!("Installed: {}", data.rust.installed);
            println!("Running: {}", data.rust.running);
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
