use crate::error_template::{AppError, ErrorTemplate};
use crate::server::types::SaleorApp;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/saleor-marketplace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
pub async fn get_all_apps() -> Result<Vec<SaleorApp>, ServerFnError>{
    let conn = crate::server::db::connect()?;
    let res = conn.query("SELECT * FROM saleor_app WHERE saleor_app.name ='Sitemap generator' ");
    dbg!(&res);
    _ = res.await;
    Err(ServerFnError::new("rip"))
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let apps = create_resource(|| (), |_| async move {get_all_apps().await});
    view! {
        <h1>"Welcome to Leptos!"</h1>
        {move || match apps.get() {
            Some(_d) => view! { <p>"loaded smt"</p> }.into_view(),
            None => view! { <p>"loading..."</p> }.into_view(),
        }}

        <div></div>
    }
}
