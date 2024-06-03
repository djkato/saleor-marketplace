use leptos::*;

use crate::server::functions::get_all_apps;


#[component]
pub fn app_list() -> impl IntoView {
    let apps = create_resource(|| (), |_| async move {get_all_apps().await});
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                apps.get()
                    .map(|apps| {
                        match apps {
                            Ok(apps) => {
                                view! {
                                    <ul>

                                        {apps
                                            .into_iter()
                                            .map(|app| view! { <li>{app.name}</li> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_view()
                            }
                            Err(e) => {
                                view! { <p>"Failed fetching apps, " {e.to_string()}</p> }
                                    .into_view()
                            }
                        }
                    })
            }}

        </Suspense>
    }
}
