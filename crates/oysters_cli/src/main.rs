use clap::{Command, arg};
use oysters_client::Client;

fn cli() -> Command {
    Command::new("oysters-cli")
        .about("Run commands on the running Oysters server")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("dump").about("Dump the current map to an SQLite file"))
        .subcommand(Command::new("scan").about("Scan the current map for outdated items"))
        .subcommand(
            Command::new("get")
                .about("Get a value by its key")
                .arg(arg!(<KEY> "The key to select")),
        )
        .subcommand(
            Command::new("insert")
                .about("Insert a value into the map")
                .arg(arg!(<KEY> "The key to insert into"))
                .arg(arg!(<VALUE> "The value to insert")),
        )
        .subcommand(
            Command::new("incr")
                .about("Incremenet a key")
                .arg(arg!(<KEY> "The key to increment")),
        )
        .subcommand(
            Command::new("decr")
                .about("Decrement a key")
                .arg(arg!(<KEY> "The key to decrement")),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a value from the map")
                .arg(arg!(<KEY> "The key to remove")),
        )
        .subcommand(
            Command::new("filter")
                .about("Filter by the given pattern (and return BOTH keys AND values)")
                .arg(arg!(<PATTERN> "The pattern to filter by")),
        )
        .subcommand(
            Command::new("filter_keys")
                .about("Filter by the given pattern (and return ONLY keys)")
                .arg(arg!(<PATTERN> "The pattern to filter by")),
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let client = Client::new("http://localhost:5072".to_string());

    match matches.subcommand() {
        Some(("dump", _)) => client.dump().await,
        Some(("scan", _)) => client.scan().await,
        Some(("get", sub)) => println!(
            "{}",
            client.get(sub.get_one::<String>("KEY").unwrap()).await
        ),
        Some(("insert", sub)) => println!(
            "{}",
            client
                .insert(
                    sub.get_one::<String>("KEY").unwrap(),
                    sub.get_one::<String>("VALUE").unwrap()
                )
                .await
        ),
        Some(("incr", sub)) => println!(
            "{}",
            client.incr(sub.get_one::<String>("KEY").unwrap()).await
        ),
        Some(("decr", sub)) => println!(
            "{}",
            client.decr(sub.get_one::<String>("KEY").unwrap()).await
        ),
        Some(("remove", sub)) => println!(
            "{}",
            client.remove(sub.get_one::<String>("KEY").unwrap()).await
        ),
        Some(("filter", sub)) => {
            for (k, v) in client
                .filter(sub.get_one::<String>("PATTERN").unwrap())
                .await
            {
                println!("{} = {}", k, v.0)
            }
        }
        Some(("filter_keys", sub)) => {
            for k in client
                .filter_keys(sub.get_one::<String>("PATTERN").unwrap())
                .await
            {
                println!("{k}")
            }
        }
        _ => unreachable!(),
    }
}
