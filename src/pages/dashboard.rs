use yew::prelude::*;

use crate::router::Route;
use yew_router::prelude::*;

pub struct Dashboard {
    counter:String,
   
}

pub enum Msg {}



impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Dashboard {
           
          counter:String::from("halaman dashboard")
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
       
        false
    }

    

     fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-light bg-light">
              <div class="container-fluid">
              <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
              
              </div>

              
              <div class="container-fluid">
              <p>{self.counter.clone()}</p>
            </div>
            </nav>
         
        }
    }
}
