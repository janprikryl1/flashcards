use yew::prelude::*;

#[function_component(PencilIcon)]
pub fn pencil_icon() -> Html {

    html! {
        <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M15.232 5.232l3.536 3.536M4 20l4.586-1.146a2 2 0 00.894-.516l9.768-9.768a2 2 0 000-2.828l-1.414-1.414a2 2 0 00-2.828 0L5.238 13.238a2 2 0 00-.516.894L4 20z" />
        </svg>
    }
}