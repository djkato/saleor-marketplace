use leptos::*;
use strum::IntoEnumIterator;

use crate::server::types::{AppCategory, DeploymentType};

#[component]
pub fn app_editor() -> impl IntoView {
    let name = create_node_ref();
    let slug = create_node_ref();
    let authors = create_node_ref();
    let built_for_url = create_node_ref();
    let description = create_node_ref();
    let version = create_node_ref();
    let images = create_node_ref();
    let logo = create_node_ref();
    let manifest = create_node_ref();
    let supported_deployments = create_node_ref();
    let category = create_node_ref();
    let docker_compose = create_node_ref();

    view! {
        <form>
            <label>
                <input node_ref=name type="text" placeholder="Name"/>
            </label>
            <label>
                <input node_ref=slug type="text" placeholder="Slug"/>
            </label>
            <label>
                <input node_ref=authors type="text" placeholder="<Author, authors@email.tld>"/>
            </label>
            <label>
                <input
                    node_ref=built_for_url
                    id="built_for_url"
                    placeholder="https://superfaktura.sk/"
                />
            </label>

            <label>
                <textarea
                    node_ref=description
                    id="description"
                    placeholder="Description"
                ></textarea>
            </label>
            <label>
                <input node_ref=version id="version" type="text" placeholder="Version"/>
            </label>
            <label>
                <input node_ref=images id="images" type="file" placeholder="Example images"/>
            </label>
            <label>
                <input node_ref=logo id="logo" type="file" placeholder="Logo"/>
            </label>
            <label>
                <textarea node_ref=manifest id="manifest" placeholder="JSON"></textarea>
            </label>
            <label>
                <select multiple node_ref=supported_deployments id="supported_deployments">
                    {move || {
                        DeploymentType::iter()
                            .map(|d| {
                                let s: &'static str = d.into();
                                view! { <option value=s>{s}</option> }.into_view()
                            })
                            .collect_view()
                    }}

                </select>
            </label>
            <label>
                <select node_ref=category id="category">
                    {move || {
                        AppCategory::iter()
                            .map(|d| {
                                let s: &'static str = d.into();
                                view! { <option value=s>{s}</option> }
                            })
                            .collect_view()
                    }}

                </select>
            </label>
            <label>
                <textarea node_ref=docker_compose id="docker_compose"></textarea>
            </label>

        </form>
    }
}
