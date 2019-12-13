//! https://adventofcode.com/2019/day/13

use cursive::event::{Event, EventResult, Key};
use cursive::traits::*;
use cursive::{theme, Cursive, Printer};

use advent_of_code_2019::{Intcode, Result};

fn main() {
    let input = include_str!("../../input/2019/day13.txt").trim();
    let mut code = Intcode::parse(input);
    code.prog[0] = 2;

    let width = 42;
    let height = 26;
    let map = vec![vec![0; width]; height];

    let mut game = Game::new(code, map);
    // Make sure everything is drawn first
    game.run_until_input();

    let mut cursive = Cursive::default();
    cursive.add_layer(game.fixed_size((width * 2, height + 2)));
    cursive.run();
}

struct Game {
    code: Intcode,
    map: Vec<Vec<i64>>,
    score: i64,
    save: Intcode,
    saved_map: Vec<Vec<i64>>,
    initial: Intcode,
}

impl Game {
    fn new(code: Intcode, map: Vec<Vec<i64>>) -> Self {
        let save = code.clone();
        let saved_map = map.clone();
        let initial = code.clone();
        Game {
            code,
            map,
            score: 0,
            save,
            saved_map,
            initial,
        }
    }
}

impl View for Game {
    fn draw(&self, printer: &Printer) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let (text, color) = match col {
                    1 => ("##", theme::BaseColor::Black),
                    2 => ("**", theme::BaseColor::Blue),
                    3 => ("==", theme::BaseColor::Black),
                    4 => ("()", theme::BaseColor::Red),
                    _ => {
                        continue;
                    }
                };

                printer.with_color(theme::Color::Light(color).into(), |printer| {
                    printer.print((x * 2, y), text);
                });
            }
        }

        if self.score == 0 {
            printer.print(
                (8, 26),
                "Arrows = move, Space = stay, Enter = save, Esc = load, R = restart",
            );
        } else {
            printer.print((15, 26), &format!("score: {}", self.score));
        };
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
            Event::Char('r') => {
                self.code = self.initial.clone();
            }
            _ => {
                self.code.add_input(0);
            }
        }

        self.run_until_input();

        EventResult::Consumed(None)
    }
}

impl Game {
    fn run_until_input(&mut self) {
        loop {
            let x = match self.code.run() {
                Result::Output(x) => x,
                _ => {
                    return;
                }
            };

            let y = match self.code.run() {
                Result::Output(y) => y,
                _ => return,
            };

            let t = match self.code.run() {
                Result::Output(t) => t,
                _ => {
                    return;
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

fn play_game() -> (usize, i64) {
    let input = include_str!("../../input/2019/day13.txt").trim();
    let mut code = Intcode::parse(input);
    code.prog[0] = 2;
    let width = 42;
    let height = 26;
    let mut map = vec![vec![0; width]; height];
    let mut score = 0;
    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    let mut finishing = false;
    let mut all_blocks = 0;
    loop {
        let x = match code.run() {
            Result::Output(x) => x,
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
            Result::Halt => {
                panic!("Unexpected halt before game finished");
            }
        };

        let y = match code.run() {
            Result::Output(y) => y,
            other => {
                panic!("Unexpected result {:?}", other);
            }
        };

        let t = match code.run() {
            Result::Output(t) => t,
            other => {
                panic!("Unexpected result {:?}", other);
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

        let blocks = map
            .iter()
            .flat_map(|row| row.iter().filter(|&t| t == &2))
            .count();
        all_blocks = all_blocks.max(blocks);

        if score == 0 {
            continue;
        }

        // Wait before checking the blocks until we have some score. In the first few instructions,
        // the game just paints the walls so there are no blocks yet.
        if score != 0 && blocks == 0 {
            if finishing {
                break;
            }
            // Need to get the new score first, so do one more iteration
            finishing = true;
        }
    }
    (all_blocks, score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let (number_of_blocks, score) = play_game();

        assert_eq!(number_of_blocks, 380);
        assert_eq!(score, 18647);
    }
}
