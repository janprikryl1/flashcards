use yew::{html, Classes, Html};

#[derive(Clone, PartialEq)]
pub enum IconKind {
    Plus,
    FolderOpen,
    Brain,
    Download,
    BookOpen,
}

pub fn render_icon(kind: &IconKind, class: Classes) -> Html {
    match kind {
        IconKind::Plus => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                 stroke="currentColor" class={class}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M12 4v16m8-8H4" />
            </svg>
        },
        IconKind::FolderOpen => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                 stroke="currentColor" class={class}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M3 7a2 2 0 012-2h4l2 2h8a2 2 0 012 2v2H6a2 2 0 00-1.94 1.5l-1.6 6A2 2 0 004.4 21h13.2a2 2 0 001.94-1.5L22 11" />
            </svg>
        },
        IconKind::Brain => html! {
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
                 fill="none" stroke="currentColor" class={class}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M8 9a3 3 0 016 0m-6 0a3 3 0 00-3 3 3 3 0 003 3m6-6a3 3 0 013 3 3 3 0 01-3 3m-6 0a3 3 0 006 0m-3-9v12" />
            </svg>
        },
        IconKind::Download => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                 stroke="currentColor" class={class}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2M12 4v10m0 0l-4-4m4 4l4-4" />
            </svg>
        },
        IconKind::BookOpen => html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none"
                 viewBox="0 0 24 24" stroke="currentColor" class={class}>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M12 6v12M4 19a2 2 0 012-2h5V5H6a2 2 0 00-2 2v12zm16 0a2 2 0 01-2-2h-5V5h5a2 2 0 012 2v12z" />
            </svg>
        },
    }
}