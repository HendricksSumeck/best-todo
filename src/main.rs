use dialoguer::{theme::ColorfulTheme, Select};
use best_todo::{Todo, clear_terminal_screen};
use config::{Config, File, FileFormat};

fn main() {
    let path: &str;

    if cfg!(debug_assertions) {
        path = "D:/Back/Rust/best-todo/config.json";
    } else {
        path = "D:/Back/Rust/best-todo/target/release/config.json";
    }

    let builder = Config::builder()
        .add_source(File::new(path, FileFormat::Json))
        .build();

    let database_url: String = builder.unwrap().get_string("database.url").unwrap();

    let todo = Todo::new(database_url).expect("Couldn't create the todo instance");

    let mut flag = false;

    while !flag{
        let items = &["List", "Add", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option:")
            .items(items)
            .interact()
            .unwrap();

        let command = items[selection];
        match &command[..] {
            "List" => todo.list(),
            "Add" => todo.add(),
            "Exit" => flag = true,
            _ => { }
        }
    };

    clear_terminal_screen()
}


