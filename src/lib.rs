use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    id: i32,
    title: String,
    speaker: String,
    url: String,
}

impl Video {
    pub fn default() -> Vec<Self> {
        vec![
            Video {
                id: 1,
                title: "Building and breaking things".to_string(),
                speaker: "John Doe".to_string(),
                url: "https://youtu.be/PsaFVLr8t4E".to_string(),
            },
            Video {
                id: 2,
                title: "The development process".to_string(),
                speaker: "Jane Smith".to_string(),
                url: "https://youtu.be/PsaFVLr8t4E".to_string(),
            },
            Video {
                id: 3,
                title: "The Web 7.0".to_string(),
                speaker: "Matt Miller".to_string(),
                url: "https://youtu.be/PsaFVLr8t4E".to_string(),
            },
            Video {
                id: 4,
                title: "Mouseless development".to_string(),
                speaker: "Tom Jerry".to_string(),
                url: "https://youtu.be/PsaFVLr8t4E".to_string(),
            },
        ]
    }
}

#[derive(Properties, PartialEq)]
pub struct VideoDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct VideoListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideoListProps { videos, on_click }: &VideoListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            let call_back = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| {
                    on_click.emit(video.clone())
                })
            };
            html! {
                <p key={video.id} onclick={call_back}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
