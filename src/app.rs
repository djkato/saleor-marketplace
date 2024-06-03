use crate::components::header::Header;
use crate::error_template::{AppError, ErrorTemplate};
use crate::routes::apps::Apps;
use crate::routes::guides::Guides;
use crate::routes::home::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
pub struct UrlAppParams {
    slug: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/saleor-marketplace.css"/>

        // sets the document title
        <Title text="Saleors Harbour"/>

        <Header/>
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="p-4 md:p-8 md:px-16">
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/apps" view=Apps/>
                    <Route path="/apps/" view=Apps/>
                    <Route path="/apps/create" view=Apps/>
                    <Route path="/apps/update/:slug" view=Apps/>
                    <Route path="/guides" view=Guides/>
                </Routes>
            </main>
        </Router>
    }
}
