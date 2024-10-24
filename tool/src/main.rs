use postgres::{Client, NoTls};
use clap::{Parser, Subcommand, Args};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("../iot-manager/migration");
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    MigrateDatabase(MigrateDatabase),
}

#[derive(Args)]
struct MigrateDatabase {
    #[arg(short, long)]
    dsn: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::MigrateDatabase(arg) => {
            let mut conn = Client::connect(arg.dsn.unwrap().as_str(), NoTls).unwrap();
            embedded::migrations::runner().run(&mut conn).unwrap();
        }
    }
}
