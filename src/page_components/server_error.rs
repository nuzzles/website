use yew::prelude::*;

use crate::components::Construction;

#[function_component(ServerErrorPage)]
pub fn view() -> Html {
    html! { <Construction title="Error" message="This is embarassing." /> }
}
