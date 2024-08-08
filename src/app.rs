use yew::prelude::*;

enum Direction {
    North(u8, u8),
    East(u8, u8),
    South(u8, u8),
    West(u8, u8),
}

struct Snake {
    current_direction: Direction,
}

enum SnakePart {
    Head,
    Body,
}

enum State {
    Snake(SnakePart),
    Apple,
    Nothing,
}

struct Square {
    state: State,
}

impl Square {
    pub fn new() -> Self {
        Square {
            state: State::Nothing,
        }
    }
}

struct Board {
    squares: Squares,
}

type Squares = [[Square; 32]; 32];

impl Board {
    pub fn new() -> Self {
        let squares: Squares = core::array::from_fn(|_| core::array::from_fn(|_| Square::new()));

        Board { squares }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let square_size = "30px";
    let grid_style = format!(
        "display: grid; grid-template-columns: repeat(32, {square_size}); \
             grid-template-rows: repeat(32, {square_size});",
    );
    let board = Board::new();
    let squares: Html = board.squares.iter().flat_map(|row| row.iter().map(|_| {
        let color = format!("rgb({}, {}, {})", 166, 166, 166);

        html! {
            <div style={format!("background-color: {}; width: {square_size}; height: {square_size}; border-style: solid", color)}></div>
        }
    })).collect::<Html>();

    html! {
        <main>
        <div style={grid_style}>
            { squares }
        </div>
        // <div class="snake"> </div>
        </main>
    }
}
