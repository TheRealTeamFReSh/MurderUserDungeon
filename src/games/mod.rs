mod laby;

use bevy::prelude::*;

use crate::console::event::PrintConsoleEvent;

pub struct ConsoleGamesPlugin;

impl Plugin for ConsoleGamesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ConsoleGamesData::default());
        app.add_startup_system(setup.system());
        app.add_plugin(laby::LabyrinthGamePlugin);
    }
}

#[derive(Default)]
pub struct ConsoleGamesData {
    //games: Vec<String>,
}

fn setup() {
    info!("Loading ConsoleGamesPlugin");
}

pub fn handle_play_command(
    args: &[&str],
    mut console_writer: &mut EventWriter<PrintConsoleEvent>,
) {
    // if there is only the command
    if args.len() == 1 {
        console_writer.send(PrintConsoleEvent("No game specified...".to_string()));
        print_games_list(&mut console_writer);
        return;
    }

    match args[1].to_lowercase().as_str() {
        "labyrinth" => laby::start_game(),
        _ => {
            console_writer.send(PrintConsoleEvent(format!("The game '{}' isn't installed yet...", args[1])));
            print_games_list(&mut console_writer);
        }
    }
}

fn print_games_list(console_writer: &mut EventWriter<PrintConsoleEvent>) {
    let mut res = String::from("Printing the list of available games :\n\n");
    res.push_str("CONSOLE GAMES INSTALLED\n");
    res.push_str("=======================\n");
    res.push_str("- Labyrinth: a labyrinth game\n\n");

    console_writer.send(PrintConsoleEvent(res));
}