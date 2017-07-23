extern crate notify_rust; extern crate regex;
extern crate chrono;
#[macro_use] extern crate clap;
use clap::{Arg, App, SubCommand};
use notify_rust::Notification;
use std::time::Duration;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::process::Command;
use std::path::Path;
use std::thread;
use std::env;
use chrono::Local;
use regex::Regex;

fn main() {
    let matches = App::new("nowdo")
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(Arg::with_name("interval")
             .short("t")
             .long("time")
             .value_name("TIME")
             .takes_value(true)
             .help("How long in minutes to wait between notifications"))
        .subcommand(SubCommand::with_name("edit")
                    .about("Edit the todo file with $EDITOR"))
        .get_matches();

    if matches.subcommand_matches("edit").is_some() {
        edit();
    }
    else {
        let t = matches.value_of("interval").unwrap_or("10");
        let time: u64 = t.parse().unwrap_or(10);
        notify(time);
    }
}

///Get the home directory
fn get_path() -> String{
    //check if file exists
    let mut home = "".to_owned();
    match env::var("HOME") {
        Ok(lang) => home = lang.to_owned(),
        Err(e) => {
            println!("Couldnt get $HOME env({})", e);
        }
    };
    let home = format!("{}/todo.md", home);
    let path = Path::new(&home);
    //Create the file if it doesnt exist
    if !Path::new(path).exists() {
        println!("todo.md doesnt exist, please run ./nowdo edit to add tasks");
        File::create(&home).unwrap();
        std::process::exit(1);
    }
    return path.to_str().unwrap().to_owned();
}

///Open a file in $EDITOR
fn edit() {

    let file = get_path();
    let mut editor = "nano".to_owned();
    if env::var("EDITOR").is_ok() {
        editor = env::var("EDITOR").unwrap();
    }
    Command::new(editor).arg(file).status().unwrap();
}

///Send a notification every x minutes
fn notify(time: u64) {
    //get path
    //If the file already exists
    let path = get_path();
    if Path::new(&path).exists() {
        loop {
            let file = File::open(&path).unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();

            let re = Regex::new(r"^#").unwrap();
            let item: Vec<&str> = re.split(&contents).collect();

            //Get the first section
            if item.len() > 1 {
                //Get title
                let top: String = item[1].to_owned();
                let item: Vec<&str> = top.split("\n").collect();
                let title: String = item[0].to_owned();

                //Get body
                let mut lines: Vec<String> = Vec::new();
                for i in 1..item.len() {
                    if !item[i].is_empty() {
                        if re.is_match(&item[i]) { break;}
                        else {
                            lines.push(item[i].to_owned());
                        }
                    }
                }
                let body = lines.join("\n");
                //Send notification
                Notification::new()
                    .summary(&title)
                    .body(&body)
                    .show()
                    .unwrap();
                println!("{}\n{}", title, body);
                let date = Local::now();
                println!("{}\n", date.format("%Y-%m-%d %H:%M:%S"));

            }
            //sleep for x minutes
            let wait = Duration::new(time * 60, 0);
            thread::sleep(wait);

        }
    }
    // Create todo.md if it doesnt exist and explain how to use it
    else {
        File::create(path).unwrap();
        println!("Creating todo file");
        Notification::new()
            .summary("~/todo.md doesnt exist, creating it")
            .body("You should add some things to the todo file, use the format
                      #this is a title
                      then everything below it until a line starts with # is the body")
            .show()
            .unwrap();
    }

}

