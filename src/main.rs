use macroquad::prelude::*;

enum GameState {
    Menu,
    Game,
    Dead
}

struct Game {
    world: World,
    snake: Snake,
    elapsed: f32,
    font: Font
}

impl Game {
    async fn new(font: Font) -> Self {
        Self {
            world: World::new(Vec2 { x: screen_width(), y: screen_height() }),
            snake: Snake::new(32),
            elapsed: 0.0,
            font
        }
    }

    fn update(&mut self) {
        self.elapsed += get_frame_time();

        let timestep = 1.0 / self.snake.speed as f32;

        if self.elapsed >= timestep {
            self.snake.update();
            self.world.update(&mut self.snake);
            self.elapsed -= timestep;

            if self.snake.check_collisions() {
                self.snake.lives -= 1;
            }
        }
    }

    fn draw(&self) {
        self.world.draw();
        self.snake.draw();
        let text_params = TextParams { font: self.font, font_size: 30, color: WHITE, ..Default::default() };

        draw_text_ex(
            &format!("Lives: {}", self.snake.lives),
            30.0,
            30.0,
            text_params
        );

        draw_text_ex(
            &format!("Score: {}", self.snake.score),
            30.0,
            60.0,
            text_params
        );
    }

    fn reset(&mut self) {
        self.snake.lives = 3;
        self.snake.score = 0;
    }
}

struct Snake {
    body: Vec<Segment>,
    size: u16,
    direction: Direction,
    speed: u16,
    lives: u16,
    score: u16,
    body_rect: Rect,
}

impl Snake {
    fn new(size: u16) -> Self {
        Self {
            size,
            body: vec![
                Segment { position: Vec2 { x: 5.0, y: 7.0 } },
                Segment { position: Vec2 { x: 5.0, y: 6.0 } },
                Segment { position: Vec2 { x: 5.0, y: 5.0 } },
            ],
            body_rect: Rect {
                x: 0.0,
                y: 0.0,
                w: (size - 1) as f32,
                h: (size - 1) as f32,
            },
            direction: Direction::Down,
            speed: 10,
            lives: 3,
            score: 0,
        }
    }

    fn get_position(&self) -> Vec2 {
        if !self.body.is_empty() {
            self.body.first().unwrap().position
        } else {
            Vec2 { x: 1.0, y: 1.0 }
        }
    }

    fn check_collisions(&mut self) -> bool {
        if self.body.len() < 5 {
            return false;
        }

        if let Some(head) = self.body.get(0) {
            for (index, body) in self.body.iter().skip(1).enumerate() {
                if body.position == head.position {
                    let i = self.body.len() - index;
                    self.cut(i);
                    return true;
                }
            }
        }

        false
    }

    fn cut(&mut self, index: usize) {
        for _ in 0..index {
            self.body.pop();
        }

        // self.lives -= 1;

        // TODO : if lives == 0 the game has been lost
    }

    fn grow(&mut self) {
        let tail_head = self.body.last().unwrap();

        if self.body.len() > 1 {
            let tail_bone = self.body.get(self.body.len() - 2).unwrap();

            if tail_head.position.x == tail_bone.position.x {
                if tail_head.position.y > tail_bone.position.y {
                    self.body.push(Segment {
                        position: Vec2 {
                            x: tail_head.position.x,
                            y: tail_head.position.y + 1.0,
                        },
                    });
                } else {
                    self.body.push(Segment {
                        position: Vec2 {
                            x: tail_head.position.x,
                            y: tail_head.position.y - 1.0,
                        },
                    });
                }
            } else if tail_head.position.y == tail_bone.position.y {
                if tail_head.position.x > tail_bone.position.x {
                    self.body.push(Segment {
                        position: Vec2 {
                            x: tail_head.position.x + 1.0,
                            y: tail_head.position.y,
                        },
                    });
                } else {
                    self.body.push(Segment {
                        position: Vec2 {
                            x: tail_head.position.x - 1.0,
                            y: tail_head.position.y,
                        },
                    });
                }
            }
        } else {
            match self.direction {
                Direction::Down => self.body.push(Segment {
                    position: Vec2 {
                        x: tail_head.position.x,
                        y: tail_head.position.y - 1.0,
                    },
                }),
                Direction::Left => {
                    self.body.push(Segment {
                        position: Vec2 {
                            x: tail_head.position.x + 1.0,
                            y: tail_head.position.y,
                        },
                    });
                }
                Direction::Right => {
                    self.body.push(Segment {
                        position: Vec2 {
                            x: tail_head.position.x - 1.8,
                            y: tail_head.position.y,
                        },
                    });
                }
                Direction::Up => self.body.push(Segment {
                    position: Vec2 {
                        x: tail_head.position.x,
                        y: tail_head.position.y + 1.0,
                    },
                }),
            }
        }
    }

    fn r#move(&mut self) {
        let b = self.body.clone();
        let mut i = self.body.len() - 1;
        for body in self.body.iter_mut().skip(1).rev() {
            // println!("#{} From {:?} to {:?}", i, body.position, b.get(i - 1).unwrap().position);
            // println!("{:?}",  b.get(i - 1).unwrap().position);

            body.position = b.get(i - 1).unwrap().position;
            i -= 1;
        }

        if let Some(head) = self.body.get_mut(0) {
            match self.direction {
                Direction::Down => {
                    head.position.y += 1.0;
                }
                Direction::Left => {
                    head.position.x -= 1.0;
                }
                Direction::Right => {
                    head.position.x += 1.0;
                }
                Direction::Up => {
                    head.position.y -= 1.0;
                }
            }
        }
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Right) && self.direction != Direction::Left {
            self.direction = Direction::Right;
        } else if is_key_down(KeyCode::Down) && self.direction != Direction::Up {
            self.direction = Direction::Down
        } else if is_key_down(KeyCode::Left) && self.direction != Direction::Right {
            self.direction = Direction::Left;
        } else if is_key_down(KeyCode::Up) && self.direction != Direction::Down {
            self.direction = Direction::Up;
        }

        self.r#move();
    }

    fn draw(&self) {
        if let Some(head) = self.body.first() {
            // Draw the head
            draw_rectangle(
                head.position.x * self.size as f32,
                head.position.y * self.size as f32,
                (self.size - 1) as f32,
                (self.size - 1) as f32,
                YELLOW,
            );

            for body in self.body.iter().skip(1) {
                draw_rectangle(
                    body.position.x * self.size as f32,
                    body.position.y * self.size as f32,
                    (self.size - 1) as f32,
                    (self.size - 1) as f32,
                    RED,
                );
            }
        }
    }
}

#[derive(Clone)]
struct Segment {
    position: Vec2,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct World {
    // player: Snake,
    item: Vec2,
    block_size: i32,
    window_size: Vec2,
}

impl World {
    fn new(window_size: Vec2) -> Self {
        let mut s = Self {
            window_size,
            block_size: 32,
            item: Vec2 { x: 0.0, y: 0.0 },
            // player: Snake::new(32)
        };
        s.spawn_apple();
        s
    }

    fn spawn_apple(&mut self) {
        let max_x = (self.window_size.x / self.block_size as f32) - 2.0;
        let max_y = (self.window_size.y / self.block_size as f32) - 2.0;

        self.item = Vec2 {
            x: rand::gen_range(0.0, max_x + 1.0).round(),
            y: rand::gen_range(0.0, max_y + 1.0).round(),
        };
        println!("Aplpe: {:?}", self.item);
    }

    fn update(&mut self, snake: &mut Snake) {
        if snake.get_position() == self.item {
            snake.grow();
            snake.score += 1;
            // TODO : increase score
            self.spawn_apple();
        }

        if is_key_down(KeyCode::Space) {
            self.spawn_apple();
        }

        let grid_size_x = self.window_size.x / self.block_size as f32;
        let grid_size_y = self.window_size.y / self.block_size as f32;

        // if self.player.get_position().x <= 0.0
        //     || self.player.get_position().y <= 0.0
        //     || self.player.get_position().x >= grid_size_x - 1
        //     || self.player.get_position().y >= grid_size_y - 1 {

        //     }
    }

    fn draw(&self) {
        draw_circle(
            (self.item.x * self.block_size as f32) + (self.block_size / 2) as f32,
            (self.item.y * self.block_size as f32) + (self.block_size / 2) as f32,
            self.block_size as f32 / 2.0,
            RED,
        );
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let font = load_ttf_font("res/Heebo.ttf").await.expect("Cannot load font");
    let mut game = Game::new(font.clone()).await;
    let mut game_state = GameState::Menu;

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Dead => {
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Game;
                    game.reset();
                }
            }
            GameState::Game => {
                game.update();

                if game.snake.lives <= 0 {
                    game_state = GameState::Dead;
                }
            }
        }
        
        clear_background(BLACK);
        
        match game_state {
            GameState::Menu => {
                let text = "Welcome to the Snake game! Press Space to start";
                let dim = measure_text(&text, Some(font), 30, 1.0);

                draw_text_ex(
                    &text,
                    (screen_width() - dim.width) * 0.5,
                    (screen_height() - dim.height) * 0.5,
                    TextParams {
                        font,
                        font_size: 30,
                        ..Default::default()
                    }
                );
            }
            GameState::Dead => {
                let text = format!("You lost the game! Score: {}", game.snake.score);
                let dim = measure_text(&text, Some(font), 30, 1.0);

                draw_text_ex(
                    &text,
                    (screen_width() - dim.width) * 0.5,
                    (screen_height() - dim.height) * 0.5,
                    TextParams {
                        font,
                        font_size: 30,
                        ..Default::default()
                    }
                );
            }
            GameState::Game => {
                game.draw();

            }
        }


        next_frame().await;
    }
}
