use super::super::*;

#[function_component(RawHtml)]
pub(crate) fn raw() -> Html {
    html! {
        <p class="bg-yellow-100 dark:bg-orange-400">{"If this string's background is red, it means there is multiple components and there is tailwindcss support."}</p>
    }
}
