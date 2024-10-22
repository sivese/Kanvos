use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=5/>
    }
}

#[component]
fn StaticList(
    length: usize,
) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));

    let counter_buttons = counters.map(|(count, set_count)| {
        view! {
            <li>
                <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                >
                    {count}
                </button>
            </li>
        }
    })
    .collect::<Vec<_>>();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

#[component]
fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counter = (0..initial_length).map(|id| (id, create_signal(id +1)))
        .collect::<Vec<_>>();
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}
