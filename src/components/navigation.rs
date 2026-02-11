use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::NavDestination;
use crate::components::NavLink;
use crate::router::Route;

#[styled_component(Navigation)]
pub fn view() -> Html {
    // If updating the links, update the sitemap!
    let nav_links: [(NavDestination, Html); 3] = [
        (NavDestination::Internal(Route::Home), html!({ "home" })),
        (NavDestination::Internal(Route::Resume), html!({ "résumé" })),
        (
            NavDestination::External(AttrValue::Static("https://nuzzles.github.io/")),
            html! {{ "blog" }},
        ),
    ];

    let links_css = css! {
        list-style-type: none;
        padding: 0;
        margin: 0;

        & li {
            padding-left: 10px;
            padding-right: 10px;
            display: inline-block;
        }

        a,a:hover {
            text-decoration:none;
        }
    };

    html! {
        <nav>
            <ul class={links_css}>
            {
                nav_links.into_iter().map(|(domain, display)| {
                    html!{
                        <li>
                            <NavLink to={domain}>
                                { display }
                            </NavLink>
                        </li>
                    }
                })
                .collect::<Html>()
            }
            </ul>
        </nav>
    }
}
