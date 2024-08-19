use std::time::Duration;

use futures::{Stream, StreamExt};
use yew::{platform::time::interval, prelude::*};

enum Message {
    Tick,
}

struct Game {
    tick: u8,
}

fn initialise_ticker() -> impl Stream<Item = ()> {
    interval(Duration::new(1, 0))
}

impl Component for Game {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ticks = initialise_ticker();
        ctx.link().send_stream(ticks.map(|_| Message::Tick));
        Game { tick: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p> {self.tick}</p>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.tick = self.tick + 1;
        true
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div>
                <Game/>
            </div>
        </main>
    }
}
