use yew::prelude::*;

pub struct App {
    counter:i64,
   
}

pub enum Msg {
    AddOne,
    RemoveOne
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
           
            counter:0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne =>  self.counter += 1,
            Msg::RemoveOne => self.counter -= 1,
        }
        true
    }

    

     fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <p> {"Counter: "} { self.counter }</p>
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "Add 1" }</button>
                <button onclick={ctx.link().callback(|_| Msg::RemoveOne)}>{ "Remove 1" }</button>
            </div>
        }
    }
}
