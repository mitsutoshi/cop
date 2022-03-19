use clap::*;
use std::process;

mod db;

use clipboard::{ClipboardContext, ClipboardProvider};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// database file. columns: index, text
const DATA_FILE: &'static str = concat!(env!("HOME"), "/.cop.csv");

fn main() {
    let mut db = db::DB::from(DATA_FILE);

    // define sub commands
    let sub_get = Command::new("get")
        .about("Copy text to clipboard.")
        .alias("g")
        .arg(Arg::new("index").required(true));
    let sub_add = Command::new("add")
        .about("Add new text.")
        .alias("a")
        .arg(Arg::new("text").required(true));
    let sub_rm = Command::new("rm")
        .about("Remove text of specified index.")
        .alias("r")
        .arg(Arg::new("index").required(true));
    let sub_ls = Command::new("ls")
        .about("Show current text list.")
        .alias("l")
        .about("");

    let app = Command::new("cop")
        .subcommand(sub_add)
        .subcommand(sub_rm)
        .subcommand(sub_get)
        .subcommand(sub_ls)
        .version(VERSION)
        .arg(
            Arg::new("version")
                .help("Show version")
                .short('v')
                .long("version"),
        );

    match app.get_matches().subcommand() {
        Some(("add", sub_m)) => {
            let text = sub_m.value_of("text").unwrap();
            let index = db.add(text);
            println!("Added {}:{}", index, text);
        }
        Some(("rm", sub_m)) => {
            let index = match sub_m.value_of("index").unwrap().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Index must be number.");
                    process::exit(1);
                }
            };
            let text = match db.get(index) {
                Some(record) => &record.text,
                None => "",
            };
            println!("Remove {}", text);
            db.del(index);
        }
        Some(("get", sub_m)) => {
            let index = match sub_m.value_of("index").unwrap().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Index must be number.");
                    process::exit(1);
                }
            };
            let text = match db.get(index) {
                Some(record) => &record.text,
                None => "",
            };
            println!("{}", text);

            // copy text to clipboard
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(text.to_owned())
                .expect("Failed to copy text.");
        }
        Some(("ls", _)) => {
            show_list(&db)
        }
        _ => {
            show_list(&db)
        }
    }
}

fn show_list(db: &db::DB) {
    let records = db.list();
    println!("{} | Text", format!("{:<5}", "Index"));
    println!("{}", format!("{:-<50}", "-"));
    for r in records {
        println!("{}", format!("{:>5} | {}", r.index, r.text));
    }
}
