extern crate notify_rust;
extern crate regex;
use std::thread;
use std::time::Duration;
use regex::Regex;
use notify_rust::Notification;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::env;


fn main() {

    //get path
    let mut home = "".to_owned();
    match env::var("HOME") {
        Ok(lang) => home = lang.to_owned(),
        Err(e) => println!("Couldnt get $HOME env({})", e),
    };
    let home = format!("{}/todo.md", home);
    let path = Path::new(&home);

    //If the file already exists
    if Path::new(path).exists() {
        loop {
            let file = File::open(path).unwrap();
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

            }
            //sleep for ten minutes
            let ten_minutes = Duration::new(600, 0);
            thread::sleep(ten_minutes);

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
