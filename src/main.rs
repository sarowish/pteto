mod client;
mod commands;
mod database;
mod entry;
mod server;
mod subject;
mod timer;
mod utils;
use clap::{crate_version, App, AppSettings, Arg};
use client::Client;
use server::Server;
use std::env;

fn main() {
    let matches = App::new("pteto")
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .subcommand(App::new("toggle"))
        .subcommand(App::new("stop"))
        .subcommand(
            App::new("add")
                .arg(Arg::new("seconds").value_name("Seconds").required(true))
                .arg(Arg::new("subject").value_name("Subject")),
        )
        .subcommand(App::new("status"))
        .subcommand(App::new("stats"))
        .subcommand(App::new("changesubject").arg(Arg::new("subject").value_name("Subject")))
        .subcommand(App::new("kill"))
        .get_matches();
    let subject_list = &[];
    if env::args().count() == 1 {
        let mut server = Server::new(subject_list);
        server.run();
    } else {
        let mut client = Client::new();
        if let Some(ref matches) = matches.subcommand_matches("add") {
            client.add(
                matches.value_of("subject").unwrap_or_default().to_string(),
                matches.value_of("seconds").unwrap().parse().unwrap(),
            )
        } else if matches.is_present("toggle") {
            client.toggle();
        } else if matches.is_present("stop") {
            client.stop();
        } else if matches.is_present("status") {
            println!("{}", client.status());
        } else if let Some(ref matches) = matches.subcommand_matches("changesubject") {
            client.change_subject(matches.value_of("subject").unwrap().to_string());
        } else if matches.is_present("stats") {
            let subjects = client.stats();
            for subject in subjects {
                println!("{}", subject);
            }
        } else if matches.is_present("kill") {
            client.kill();
        }
    }
}
