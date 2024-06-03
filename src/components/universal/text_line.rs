use leptos::*;

#[component]
pub fn TextLine(
    title: &'static str,
    required: bool,
    value_setter: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="![&_*]:text-brand-white">
            <label class="relative h-base rounded-sm border-[3px] overflow-hidden
            flex flex-row flex-nowrap items-center px-4 pr-6 pt-4 pb-0.5">
                <input
                    required=required
                    on:input=move |ev| value_setter.update(|t| *t = event_target_value(&ev))
                    placeholder=" "
                    class="font-serif appearance-none peer text-base w-full bg-inherit"
                />
                <span class="absolute font-sans text-sm -top-0.5 transition-all duration-75
                peer-placeholder-shown:top-1.5 peer-focus:-top-0.5 peer-focus:text-sm">
                    {title}
                    {match required {
                        true => "*",
                        false => "",
                    }}

                </span>
            </label>

        </div>
    }
}
