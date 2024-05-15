use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode
    },
    execute,
    terminal
};

use std::io::{self, Write};

use colorized::*;

pub enum Colour {
    Blue,
    Black,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
    Default,
}

pub struct Xmenu{
    options: Vec<String>,
}

impl Xmenu {
    pub fn new() -> Self {
        Self{
            options: Vec::new()
        }
    }

    pub fn add(&mut self, option: &str) {
        self.options.push(option.to_string());
    }

    pub fn remove(&mut self, option:&str) {
        let mut new = Vec::new();
        for x in &self.options{
            if x!=option {
                new.push(x.to_string());
            }
        }

        self.options = new;
    }

    pub fn run(&mut self, c: Colour) -> String {
        let col = match c {
            Colour::Black => "black",
            Colour::Blue => "blue",
            Colour::Cyan => "cyan",
            Colour::Green => "green",
            Colour::Magenta => "magenta",
            Colour::Red => "red",
            Colour::White => "white",
            Colour::Yellow => "yellow",
            Colour::Default => "none",
        };

        let mut stdout = io::stdout();

        // set terminal to raw mode
        terminal::enable_raw_mode().expect("Error turning on raw mode.");
        execute!(stdout, cursor::Hide).expect("Failed to hide cursor");

        // get menu items
        let menu_items = self.options.clone();
        self.options.push("Abort".to_string()); // update for result

        let mut selected_index: usize = 0;

        // save current cursor position
        let (orig_x, orig_y) = cursor::position().expect("Failed to get current cursor position");
        let menu_height = menu_items.len() as u16;

        // Draw the initial menu
        Xmenu::draw_menu(&menu_items, selected_index, orig_x.clone(), orig_y.clone(), col);

        loop {
            if let Event::Key(key_event) = event::read().expect("Failed to read key") {
                match key_event.code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < menu_items.len()-1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Esc => {
                        selected_index = menu_items.len(); // indicate cancellation
                        break;
                    }
                    _ => {}
                }

                // redraw the menu
                Xmenu::draw_menu(&menu_items, selected_index, orig_x.clone(), orig_y.clone() - menu_height, col);
            }
        }

        // restore the cursor position and show it.
        execute!(stdout, cursor::MoveTo(orig_x, orig_y + menu_height), cursor::Show).expect("Failed to restore terminal");
        terminal::disable_raw_mode().expect("Failed to disable raw mode");

        return self.options[selected_index].clone();
    }

    // fn draw_menu(menu_items: &Vec<String>, selected_index: usize, x: u16, y: u16, c: &str) {
    //     let mut stdout = io::stdout();
    //     for (i, item) in menu_items.iter().enumerate() {
    //         let msg = "Failed to insert menu item ".to_string() + &i.to_string();
    //         execute!(stdout, cursor::MoveTo(x, y+i as u16)).expect(&msg);

    //         if i == selected_index {
    //             if c == "red" {
    //                 writeln!(stdout, "> {}", item.color(Colors::RedFg)).expect("Failed to update menu item.");
    //             } else if c == "yellow" {
    //                 writeln!(stdout, "> {}", item.color(Colors::YellowFg)).expect("Failed to update menu item.");
    //             } else if c == "green" {
    //                 writeln!(stdout, "> {}", item.color(Colors::GreenFg)).expect("Failed to update menu item.");
    //             } else if c == "cyan" {
    //                 writeln!(stdout, "> {}", item.color(Colors::CyanFg)).expect("Failed to update menu item.");
    //             } else if c == "magenta" {
    //                 writeln!(stdout, "> {}", item.color(Colors::MagentaFg)).expect("Failed to update menu item.");
    //             } else if c == "blue" {
    //                 writeln!(stdout, "> {}", item.color(Colors::BlueFg)).expect("Failed to update menu item.");
    //             } else if c == "black" {
    //                 writeln!(stdout, "> {}", item.color(Colors::BlackFg)).expect("Failed to update menu item.");
    //             } else if c == "white" {
    //                 writeln!(stdout, "> {}", item.color(Colors::WhiteFg)).expect("Failed to update menu item.");
    //             } else {
    //                 writeln!(stdout, "> {}", item).expect("Failed to update menu item.");
    //             }
    //         } else {
    //             writeln!(stdout, "  {}", item).expect("Failed to update menu item.");
    //         }
    //     }

    //     stdout.flush().expect("Error flushing output");
    // }
    fn draw_menu(menu_items: &Vec<String>, selected_index: usize, x: u16, mut y: u16, c: &str) {
        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(x, y)).expect("Failed to move cursor");
    
        for (i, item) in menu_items.iter().enumerate() {
            if i == selected_index {
                let colored_item = match c {
                    "red" => item.color(Colors::RedFg),
                    "yellow" => item.color(Colors::YellowFg),
                    "green" => item.color(Colors::GreenFg),
                    "cyan" => item.color(Colors::CyanFg),
                    "magenta" => item.color(Colors::MagentaFg),
                    "blue" => item.color(Colors::BlueFg),
                    "black" => item.color(Colors::BlackFg),
                    "white" => item.color(Colors::WhiteFg),
                    _ => item.to_string(), // Default color
                };
                writeln!(stdout, "> {}", colored_item).expect("Failed to update menu item.");
            } else {
                writeln!(stdout, "  {}", item).expect("Failed to update menu item.");
            }
            y += 1; // Move cursor to the next line
            execute!(stdout, cursor::MoveTo(x, y)).expect("Failed to move cursor");
        }
    
        stdout.flush().expect("Error flushing output");
    }
    
}