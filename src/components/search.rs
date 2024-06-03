use crate::components::universal::text_line::TextLine;
use leptos::*;

#[component]
pub fn Search() -> impl IntoView {
    let (search_text, search_text_setter) = create_signal("".to_string());
    view! {
        <div class="transition-all focus-within:fixed focus-within:z-50 top-12 left-0 right-0
        focus-within:p-4 focus-within:pt-8 focus-within:h-screen focus-within:bg-brand-monochrome-100
        md:focus-within:static md:focus-within:h-auto md:focus-within:bg-transparent md:focus-within:p-0">
            <div class="relative">
                // <TextLine {theme_inverse} error_atom={_error} value={_search_text} type={"text"} title="Čo hľadáte?" name="productSearch" />
                <TextLine title="Čo hľadáte?" required=false value_setter=search_text_setter/>
            </div>
        // {#if products}
        // <div
        // class="hidden group-focus-within/search:block pt-8 md:pt-0 md:absolute md:w-[24rem] rounded-base md:gentle-shadow
        // bg-brand-monochrome-100 overflow-scroll h-screen md:h-[80vh]">
        // <div class="grid grid-cols-1 gap-4 h-fit w-full md:fit p-4">
        // {#each products as product}
        // <ProductCardCompact product={product.node} />
        // {/each}
        // {#if products.length == 0}
        // <div class="w-full min-h-[8rem] relative flex items-center justify-center text-brand-cyan-600">
        // <span class="text-lg">Nič sa nenašlo...</span>
        // </div>
        // {/if}
        // </div>
        // </div>
        // {/if}
        </div>
    }
}
