use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::dashboard::Dashboard;


#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/Home")]
    Home,
    #[at("/Dashboard")]
    Dashboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home/>},
        Route::Dashboard => html! {<Dashboard/>},
        Route::NotFound => html! {<h1>{"404 not found"}</h1>}
    }
}



  





