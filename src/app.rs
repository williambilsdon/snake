use std::{cell::RefCell, time::Duration};

use futures::{Stream, StreamExt};
use yew::{platform::time::interval, prelude::*};
const SQUARE_SIZE: &str = "30px";

enum Message {
    Tick,
}

struct Game {
    tick: u16,
    squares: Squares,
    snake: Snake,
}

fn initialise_ticker() -> impl Stream<Item = ()> {
    interval(Duration::from_millis(150))
}

struct Square {
    is_snake: bool,
}

impl Square {
    fn set_is_snake(&mut self) {
        self.is_snake = !self.is_snake
    }
}

type Squares = [[Square; 32]; 32];

struct Snake {
    x: usize,
    y: usize,
}

impl Component for Game {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ticks = initialise_ticker();
        ctx.link().send_stream(ticks.map(|_| Message::Tick));
        let mut game = Game {
            tick: 0,
            squares: core::array::from_fn(|_| core::array::from_fn(|_| Square { is_snake: false })),
            snake: Snake { x: 0, y: 0 },
        };

        game.squares[game.snake.x][game.snake.y].set_is_snake();

        game
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let squares = self.squares.iter().flat_map(|row| row.iter().map(|square| {
            let colour = format!("rgb({}, {}, {})", 255, 255, 255);
            println!("{}", square.is_snake);

            if square.is_snake {
                println!("HELLO WORLD");
                html! {
                    <div style={format!("background-color: rgb(0,0,0); width: {SQUARE_SIZE}; height: {SQUARE_SIZE};")}></div>
                }
            } else {
                html! {
                    <div style={format!("background-color: {}; width: {SQUARE_SIZE}; height: {SQUARE_SIZE};", colour)}></div>
                }
            }
        })).collect::<Html>();

        let grid_style = format!(
            "display: grid; grid-template-columns: repeat(32, {SQUARE_SIZE}); \
                 grid-template-rows: repeat(32, {SQUARE_SIZE});",
        );

        html! {
            <div style={grid_style}>
                {squares}
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Tick => {
                self.squares[self.snake.x][self.snake.y].set_is_snake();

                if self.snake.x + 1 >= 32 {
                    self.snake.x = 0
                } else {
                    self.snake.x = self.snake.x + 1;
                }

                self.squares[self.snake.x][self.snake.y].set_is_snake();
                true
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Game/>
        </main>
    }
}
