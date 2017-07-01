extern crate termion;
extern crate rand;

use termion::raw::IntoRawMode;
use termion::color;
use termion::style;
use termion::cursor;
use termion::clear;
use termion::input::TermRead;
use termion::event::Key;

use rand::Rng;

use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::iter;

fn draw_box(text: &str, position: (u16, u16)) {
    for (index, line) in text.split("\n").enumerate() {
        print!("{}{}",
               cursor::Goto(position.0, (position.1 as usize + index) as u16),
               line);
    }
}
#[derive(PartialEq, Eq)]
enum State {
    MainMenu,
    Game,
}
struct Game {
    word: String,
    guesses: Vec<char>,
}
const words: &'static str = include_str!("words.txt");
impl Game {
    fn new() -> Game {
        Game {
            word: (*rand::thread_rng()
                        .choose(words.split("\n").collect::<Vec<&str>>().as_slice())
                        .unwrap())
                    .to_lowercase(),
            guesses: vec![],
        }
    }
    fn display(&self) -> String {
        let (w, h) = termion::terminal_size().unwrap_or((80, 24));
        let mut won = true;
        let underscores = self.word
            .chars()
            .map(|c| if self.guesses.contains(&c) || c == ' ' || c == '-' {
                     //hyphens and spaces are shown already
                     c.to_string()
                 } else {
                     won = false;
                     "_".to_owned()
                 })
            .collect::<Vec<String>>()
            .join(" ");
        let chars = self.word.chars().collect::<Vec<char>>();
        let garbage = self.guesses
            .iter()
            .filter(|x| !chars.contains(x))
            .collect::<String>();
        if won {
            text_box(format!("     YOU WIN!     \n\n{}\n\nIncorrect letters:\n{}\n\n\nCtrl-C to go back to main menu",
                             underscores,
                             garbage)
                             .as_str(),
                     (w / 2, h / 2),
                     5,
                     2)
        } else {
            text_box(format!("Guess the word:\n\n{}\n\nIncorrect letters:\n{}",
                             underscores,
                             garbage)
                             .as_str(),
                     (w / 2, h / 2),
                     5,
                     2)
        }
    }
}
fn text_box(input: &str, position: (u16, u16), margin_side: usize, margin_vert: usize) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let length = lines.len();
    let longest_line: usize = lines.iter().max_by_key(|l| l.len()).unwrap_or(&"").len();
    let width = longest_line + margin_side * 2;
    let height = lines.len() + margin_vert * 2;
    let left = position.0 - width as u16 / 2;
    let top = position.1 - height as u16 / 2;
    let mut output = format!("{}┏", cursor::Goto(left, top)) +
                     iter::repeat('━').take(width).collect::<String>().as_str() +
                     "┓";
    for i in 0..margin_vert {
        output += format!("{}{}",
                          cursor::Goto(left, top + 1 + i as u16),
                          "┃".to_owned() +
                          iter::repeat(' ').take(width).collect::<String>().as_str() +
                          "┃")
                .as_str();
    }
    for (index, line) in lines.into_iter().enumerate() {
        output += format!("{}┃",
                          cursor::Goto(left, top + index as u16 + 1 + margin_vert as u16))
                .as_str();
        let padding = width - line.len();
        let padding_left = padding / 2;
        let padding_right = padding - padding_left;
        assert_eq!(padding_left + padding_right, padding);
        assert_eq!(padding + line.len(), width);
        output += iter::repeat(' ')
            .take(padding_left)
            .collect::<String>()
            .as_str(); //leftward padding
        output += line;
        output += iter::repeat(' ')
            .take(padding_right)
            .collect::<String>()
            .as_str(); //rightward padding
        output += "┃";
    }
    for i in 0..margin_vert {
        output += format!("{}{}",
                          cursor::Goto(left, top + 3 + length as u16 + i as u16),
                          "┃".to_owned() +
                          iter::repeat(' ').take(width).collect::<String>().as_str() +
                          "┃")
                .as_str();
    }
    output += (format!("{}┗", cursor::Goto(left, top + height as u16 + 1)) +
               iter::repeat('━').take(width).collect::<String>().as_str() + "┛")
            .as_str();
    output
}
fn main() {
    let mut state = State::MainMenu;
    let mut game = Game::new();
    let (w, h) = termion::terminal_size().unwrap_or((80, 24));
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();
    draw_box(format!(
r#"{clear}{hide}┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                                    ┃	
┃            {yellow}{bold}H A N G M A N{reset}           ┃	
┃                                    ┃	
┃           {aqua}1) play game{reset}             ┃	
┃           {aqua}2) quit{reset}                  ┃	
┃                                    ┃	
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
            "#,
             clear = clear::All,
             hide=cursor::Hide,
             yellow = color::Fg(color::Yellow),
             bold = style::Bold,
             aqua = color::Fg(color::LightCyan),
             reset = color::Fg(color::Reset)).as_str(), (w/2-19, h/2-4));
    stdout.flush().unwrap();
    for key in stdin.keys() {
        use Key::*;
        match key.unwrap() {
            Ctrl('c') => {
                if state == State::MainMenu {
                    print!("{}{}", cursor::Show, clear::All);
                    stdout.flush().unwrap();
                    break;
                } else {
                    state = State::MainMenu;
                    draw_box(format!(
                r#"{clear}{hide}┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                                    ┃	
┃            {yellow}{bold}H A N G M A N{reset}           ┃	
┃                                    ┃	
┃           {aqua}1) play game{reset}             ┃	
┃           {aqua}2) quit{reset}                  ┃	
┃                                    ┃	
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
                            "#,
                             clear = clear::All,
                             hide=cursor::Hide,
                             yellow = color::Fg(color::Yellow),
                             bold = style::Bold,
                             aqua = color::Fg(color::LightCyan),
                             reset = color::Fg(color::Reset)).as_str(), (w/2-19, h/2-4));
                    stdout.flush().unwrap();
                }
            }
            Char('1') => {
                if state == State::MainMenu {
                    //start game
                    game = Game::new();
                    print!("{}{}", clear::All, game.display());
                    stdout.flush().unwrap();

                    state = State::Game;
                }
            }
            Char('2') => {
                if state == State::MainMenu {
                    print!("{}{}", cursor::Show, clear::All);
                    stdout.flush().unwrap();
                    break;
                }
            }
            Char(x) => {
                if state == State::Game {
                    if !game.guesses.contains(&x) && x.is_alphabetic() {
                        game.guesses.push(x);
                        print!("{}{}", clear::All, game.display());
                        stdout.flush().unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}
