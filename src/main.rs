use yew::prelude::*;
use yew_app::{Video, VideoDetails, VideosList};

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);
    let cb = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(move |video| {
        html! {
            <VideoDetails video = {video.clone()}/>
        }
    });
    html! {
    <>
        <h1>{"RustConf Explorer"}</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <VideosList videos = {Video::default()} on_click = { cb.clone() }/>
        </div>
        {for details}
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
