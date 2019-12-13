//! https://adventofcode.com/2019/day/13

use cursive::event::{Event, EventResult, Key};
use cursive::traits::*;
use cursive::{Cursive, Printer, theme};

use advent_of_code_2019::{Intcode, Result};

fn main() {
    let input = include_str!("../../input/2019/day13.txt").trim();
    let mut code = Intcode::parse(input);

    let mut code2 = code.clone();

    let width = 42;
    let height = 26;

    let mut map = vec![vec![0; width]; height];
    loop {
        let x = match code.run() {
            Result::Output(x) => {
                x
            }
            _ => {
                break;
            }
        };

        let y = match code.run() {
            Result::Output(y) => {
                y
            }
            _ => {
                break;
            }
        };

        let t = match code.run() {
            Result::Output(t) => {
                t
            }
            _ => {
                break;
            }
        };

        if x == -1 && y == 0 {
            // score
        } else {
            map[y as usize][x as usize] = t;
        }
    }

    println!("Part 1: {}", 0);

    code2.prog[0] = 2;
    code = code2.clone();

    let mut score = 0;
    let mut map = vec![vec![0; width]; height];
    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    let mut finishing = false;
    let mut all_blocks = 0;

    loop {
        let x = match code.run() {
            Result::Output(x) => {
                x
            }
            Result::NeedInput => {
                if ball.0 < paddle.0 {
                    code.add_input(-1);
                } else if ball.0 > paddle.0 {
                    code.add_input(1);
                } else {
                    code.add_input(0);
                }
                continue;
            }
            _ => {
                break;
            }
        };

        let y = match code.run() {
            Result::Output(y) => {
                y
            }
            _ => {
                break;
            }
        };

        let t = match code.run() {
            Result::Output(t) => {
                t
            }
            _ => {
                break;
            }
        };

        if x == -1 && y == 0 {
            score = t;
        } else {
            map[y as usize][x as usize] = t;
            match t {
                3 => {
                    paddle = (x, y);
                }
                4 => {
                    ball = (x, y);
                }
                _ => {}
            }
        }

        let blocks = map.iter()
            .flat_map(|row| row.iter().filter(|&t| t == &2))
            .count();
        all_blocks = all_blocks.max(blocks);
        if score != 0 && blocks == 0 {
            if finishing {
                break;
            }
            // Need to get the new score first, so do one more iteration
            finishing = true;
        }
    }

    // 18570 wrong 18647
    println!("Part 1: {}", all_blocks);
    println!("Part 2: {}", score);

//    let mut cursive = Cursive::default();
//    cursive.add_layer(Game::new(code2, map).full_width().fixed_height(height + 2));
//
//    cursive.run();
}


struct Game {
    code: Intcode,
    map: Vec<Vec<i64>>,
    score: i64,
    save: Intcode,
    saved_map: Vec<Vec<i64>>,
}

impl Game {
    fn new(code: Intcode, map: Vec<Vec<i64>>) -> Self {
        let save = code.clone();
        let saved_map = map.clone();
        Game {
            code,
            map,
            score: 0,
            save,
            saved_map
        }
    }
}

// Let's implement the `View` trait.
// `View` contains many methods, but only a few are required.
impl View for Game {
    fn draw(&self, printer: &Printer) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                match col {
                    1 => {
                        printer.with_color(theme::Color::Light(theme::BaseColor::Black).into(), |printer| {
                            printer.print((x, y), "#");
                        });
                    }
                    2 => {
                        printer.with_color(theme::Color::Light(theme::BaseColor::Blue).into(), |printer| {
                            printer.print((x, y), "*");
                        });
                    }
                    3 => {
                        printer.with_color(theme::Color::Light(theme::BaseColor::Black).into(), |printer| {
                            printer.print((x, y), "=");
                        });
                    }
                    4 => {
                        printer.with_color(theme::Color::Light(theme::BaseColor::Red).into(), |printer| {
                            printer.print((x, y), "o");
                        });
                    }
                    _ => {}
                }
//                printer.print_box((x, y), (1, 1), )
            }
        }

        printer.print((15, 26), &format!("score: {}", self.score));
//        printer.print((0, y), line);
        // We simply draw every event from the history.
//        for (y, line) in self.history.iter().enumerate() {

//        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left) => {
                self.code.add_input(-1);
            }
            Event::Key(Key::Right) => {
                self.code.add_input(1);
            }
            Event::Key(Key::Enter) => {
                self.save = self.code.clone();
                self.saved_map = self.map.clone();
            }
            Event::Key(Key::Esc) => {
                self.code = self.save.clone();
                self.map = self.saved_map.clone();
            }
            _ => {
                self.code.add_input(0);
            }
        }

        loop {
            let x = match self.code.run() {
                Result::Output(x) => {
                    x
                }
                _ => {
                    return EventResult::Consumed(None);
                }
            };

            let y = match self.code.run() {
                Result::Output(y) => {
                    y
                }
                _ => {
                    return EventResult::Consumed(None);
                }
            };

            let t = match self.code.run() {
                Result::Output(t) => {
                    t
                }
                _ => {
                    return EventResult::Consumed(None);
                }
            };

            if x == -1 && y == 0 {
                self.score = t;
            } else {
                self.map[y as usize][x as usize] = t;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        // TODO, part 1: 380
        assert_eq!(0, 0);
    }
}
