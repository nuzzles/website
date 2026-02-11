use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::Navigation;
use crate::components::ThemeSwitcher;

#[styled_component(DesktopHeader)]
pub fn view() -> Html {
    let header_css = css! {
        margin-bottom: 20px;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 10px;
    };

    html! {
        <header class={ header_css }>
            <Navigation />
            <div class={ css!("transform: scale(0.75);") }>
                <ThemeSwitcher />
            </div>
        </header>
    }
}
