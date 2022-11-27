use cargo_tag::cli::Cli;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        cmd.exec();
        return Ok(());
    }

    Ok(())
}
