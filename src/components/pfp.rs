use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfilePictureProps {
    pub src: AttrValue,
}

#[styled_component]
pub fn ProfilePicture(props: &ProfilePictureProps) -> Html {
    let img_css = css! {
        border-radius: 20px;
        min-width: 300px;
        width: min(40vh, 40vw);
        max-width: 450px;
        min-height: 300px;
        height: min(40vh, 40vw);
        max-height: 450px;
        object-fit: scale-down;
    };

    html! {
        <img
            alt="Spencer Imbleau"
            width="300px"
            height="300px"
            class={img_css}
            src={ props.src.clone() }
        />
    }
}
