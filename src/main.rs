mod client;
mod commands;
mod database;
mod entry;
mod server;
mod subject;
mod timer;
mod utils;
use clap::{crate_version, Arg, Command};
use client::Client;
use server::Server;
use std::env;

fn main() {
    let matches = Command::new("pteto")
        .version(crate_version!())
        .subcommand(Command::new("toggle"))
        .subcommand(Command::new("stop"))
        .subcommand(
            Command::new("add")
                .arg(Arg::new("seconds").value_name("Seconds").required(true))
                .arg(Arg::new("subject").value_name("Subject")),
        )
        .subcommand(Command::new("status"))
        .subcommand(Command::new("stats"))
        .subcommand(Command::new("changesubject").arg(Arg::new("subject").value_name("Subject")))
        .subcommand(Command::new("kill"))
        .get_matches();
    let subject_list = &[];
    if env::args().count() == 1 {
        let mut server = Server::new(subject_list);
        server.run();
    } else {
        let mut client = Client::new();
        match matches.subcommand() {
            Some(("add", sub_matches)) => client.add(
                sub_matches
                    .value_of("subject")
                    .unwrap_or_default()
                    .to_string(),
                sub_matches.value_of("seconds").unwrap().parse().unwrap(),
            ),
            Some(("toggle", _)) => client.toggle(),
            Some(("stop", _)) => client.stop(),
            Some(("status", _)) => println!("{}", client.status()),
            Some(("changesubject", sub_matches)) => {
                client.change_subject(sub_matches.value_of("subject").unwrap().to_string())
            }
            Some(("stats", _)) => {
                let subjects = client.stats();
                for subject in subjects {
                    println!("{}", subject);
                }
            }
            Some(("kill", _)) => client.kill(),
            _ => (),
        }
    }
}
