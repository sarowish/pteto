mod client;
mod commands;
mod database;
mod entry;
mod server;
mod session;
mod timer;
mod utils;
use clap::{crate_version, Arg, Command};
use client::Client;
use server::Server;

fn main() {
    let matches = Command::new("pteto")
        .version(crate_version!())
        .arg(Arg::new("no_daemon").long("no-daemon"))
        .subcommand(Command::new("toggle"))
        .subcommand(Command::new("stop"))
        .subcommand(Command::new("break").arg(Arg::new("long").short('l').long("long")))
        .subcommand(
            Command::new("add")
                .arg(Arg::new("seconds").value_name("Seconds").required(true))
                .arg(Arg::new("label").value_name("Label")),
        )
        .subcommand(Command::new("status"))
        .subcommand(Command::new("stats"))
        .subcommand(Command::new("changelabel").arg(Arg::new("label").value_name("Label")))
        .subcommand(Command::new("kill"))
        .get_matches();
    if matches.subcommand_name().is_none() {
        let mut server = Server::new();
        if !matches.is_present("no_daemon") {
            nix::unistd::daemon(true, false).unwrap();
        }
        server.run();
    } else {
        let mut client = Client::new();
        match matches.subcommand() {
            Some(("add", sub_matches)) => client.add(
                sub_matches
                    .value_of("label")
                    .unwrap_or_default()
                    .to_string(),
                sub_matches.value_of("seconds").unwrap().parse().unwrap(),
            ),
            Some(("toggle", _)) => client.toggle(),
            Some(("stop", _)) => client.stop(),
            Some(("break", sub_matches)) => client.take_break(sub_matches.is_present("long")),
            Some(("status", _)) => println!("{}", client.status()),
            Some(("changelabel", sub_matches)) => client.change_label(
                sub_matches
                    .value_of("label")
                    .unwrap_or("Unlabelled")
                    .to_string(),
            ),
            Some(("stats", _)) => {
                let labels = client.stats();
                for label in labels {
                    println!("{}", label);
                }
            }
            Some(("kill", _)) => client.kill(),
            _ => (),
        }
    }
}
