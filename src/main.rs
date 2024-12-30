use std::{fs::File, io, vec};
use dialoguer::{Select, Input};
mod main_menu_functions;

enum MainMenu {
    Write,
    ViewFile(String),
    Help,
    Exit,
}

fn enum_action(action: MainMenu) {
    match action {
        MainMenu::Write => main_menu_functions::write(),
        MainMenu::Help => main_menu_functions::help(),
        MainMenu::ViewFile(file) => main_menu_functions::read_content(file),
        MainMenu::Exit => main_menu_functions::exit_program(),
    }
}

trait MessageFunctions<M> {
    fn show_message(&self) -> M;
}

#[derive(Clone, Copy)]
struct Message<M> {
    content: M,
}

impl<M: Copy> MessageFunctions<M> for Message<M> {
    fn show_message(&self) -> M {
        self.content
    }
}

fn main() {
    let options: Vec<&str> = vec!["1 -> Write", "2 -> View file", "3 -> Help","4 -> Exit"];

    let input_int: usize = Select::new()
    .with_prompt("What do you want to do")
    .items(&options)
    .interact()
    .unwrap();
    

    match input_int {
        0 => enum_action(MainMenu::Write),
        1 => enum_action(MainMenu::ViewFile(ask_file_name())),
        2 => enum_action(MainMenu::Help),
        3 => enum_action(MainMenu::Exit),
        _ => println!("Not valid option"),
    }

}

fn ask_file_name() -> String {
    let file_name: String = Input::new()
    .with_prompt("What is the name of the file you're searching?")
    .interact_text()
    .unwrap();

    file_name
}