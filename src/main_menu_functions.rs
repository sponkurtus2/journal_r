use dialoguer::Input;
use std::fs::File;
use std::io::{BufRead, BufReader};



use crate::{Message, MessageFunctions};

pub fn write()  {
    let text: String = Input::new()
    .with_prompt("Write something")
    .interact_text()
    .unwrap();

    let m = Message{content: text.as_str()};

    println!("You wrote: {}", m.show_message());
}

pub fn read_content(file_name: String) {
    let file = File::open(file_name).expect("Couldn't open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("Couldn't read line"))
    };

}

pub fn help() {
    println!("This is a bla bla bla");
    std::process::exit(0);
}

pub fn exit_program() {
    println!("Bye bye ^^");
    std::process::exit(0);
}