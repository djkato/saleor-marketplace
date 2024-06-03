use crate::components::search::Search;
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <div class="z-10 w-full px-4 lg:px-8 h-12 md:h-18 items-center align-center
            bg-brand-sea-500 font-sans grid grid-cols-3">
                <ul class="flex flex-row gap-4 text-brand-white">
                    <li>
                        <a href="/">home</a>
                    </li>

                    <li>
                        <a href="/apps">apps</a>
                    </li>

                    <li>
                        <a href="/guides">guides</a>
                    </li>
                </ul>
                <span class="text-xl font-bold text-brand-sea-100 w-fit mx-auto">
                    <a href="/">"Saleors Harbour"</a>
                </span>

                <Search/>
                <div>
                    <script src="https://storage.ko-fi.com/cdn/scripts/overlay-widget.js"></script>
                    <script>
                        kofiWidgetOverlay.draw("djkato", {
                          "type": "floating-chat",
                          "floating-chat.donateButton.text": "Support me",
                          "floating-chat.donateButton.background-color": "#094074",
                          "floating-chat.donateButton.text-color": "#FEF9EF"
                        });
                    </script>
                </div>
            </div>
        </header>
    }
}
