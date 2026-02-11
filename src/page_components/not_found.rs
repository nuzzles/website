use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::Construction;

#[styled_component(NotFoundPage)]
pub fn view() -> Html {
    html! { <Construction title="Yikes" message="How did you get here?" /> }
}
