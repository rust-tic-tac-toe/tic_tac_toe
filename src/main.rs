mod app_runner;
mod io;
mod board_printer;
mod board;
mod game;
mod game_types;
mod players;
mod computer;
mod human;
mod board_formatter;
mod lines;

use app_runner::start;

fn main() {
    start();
}
