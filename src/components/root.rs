use std::collections::HashMap;

use gloo::file::{callbacks::FileReader, File};
use serde_json::Value;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::html::Scope;
use yew::prelude::*;
use yew::{html, html::TargetCast, Callback, Component, Context, Html};
use yew_hooks::prelude::*;

use crate::components::json_tree_viewer::get_page_size;

use super::super::services::upload_file::upload_files;
use super::json_tree_viewer::view_file;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Value,
}

pub enum Msg {
    Loaded(String, String, Value),
    Files(Vec<File>),
    Error(String),
    Loading(bool),
}

#[derive(Debug)]
pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    is_error: (bool, String),
    is_loading: bool,
}

impl From<()> for Msg {
    fn from(_: ()) -> Self {
        Msg::Error("".to_string())
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        {
            use_favicon("https://crates.io/favicon.ico".to_string());
            use_event_with_window("keypress", move |e: KeyboardEvent| {
                gloo::dialogs::alert("key pressed: {}");
                if e.key() == "Escape" {
                    // link.clone().send_message(Msg::Files(Vec::new()));
                    // link.clone().send_message(Msg::Loading(false));
                }
            });
        }
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
            is_error: (false, String::default()),
            is_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use_event_with_window("keypress", move |e: KeyboardEvent| {
            gloo::dialogs::alert("key pressed: {}");
            if e.key() == "Escape" {
                // link.clone().send_message(Msg::Files(Vec::new()));
                // link.clone().send_message(Msg::Loading(false));
            }
        });
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                self.files.push(FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                self.readers.remove(&file_name);
                self.is_error = (false, String::default());
                self.is_loading = false;
                true
            }
            Msg::Files(files) => {
                self.is_loading = true;
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();
                    let link = ctx.link().clone();
                    let reader =
                        gloo::file::callbacks::read_as_bytes(&file, move |res| match res {
                            Ok(data) => {
                                if let Ok(reader) =
                                    serde_json::from_slice::<serde_json::Value>(&data)
                                {
                                    link.send_message(Msg::Loaded(
                                        file_name.clone(),
                                        file_type,
                                        reader,
                                    ));
                                    return;
                                }

                                link.send_message(Msg::Error(format!(
                                    "failed to parse file: {}",
                                    file_name
                                )));
                            }
                            Err(_) => {
                                link.send_message(Msg::Error("failed to read file".to_string()));
                            }
                        });
                    self.readers.insert(file.clone().name(), reader);
                    self.is_error = (false, String::default());
                }
                self.is_loading = false;
                true
            }
            Msg::Error(message) => {
                self.files.clear();
                self.is_error = (true, message);
                self.is_loading = false;
                true
            }
            Msg::Loading(is_loading) => {
                self.is_loading = is_loading;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (is_error, message) = self.is_error.clone();

        if self.files.is_empty() {
            html! {
                <>
                    <Root scope={ctx.link()} />
                    <div
                        for="file-upload"
                        id="wrapper"
                        ondragover={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                        ondragenter={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                        title="Drag and drop JSON files here"
                        ondrop={ctx.link().callback(|event: DragEvent| {
                            event.prevent_default();
                            let files = event.data_transfer().unwrap().files();
                            upload_files(files)
                        })}
                        onchange={ctx.link().clone().callback(move |_: Event| Msg::Loading(true))}
                    >
                        <p id="title" title="JSON Tree Viewer">{ "JSON Tree Viewer" }</p>
                        <p id="subtitle">{ "Simple JSON Viewer that runs completely on-client. No data exchange"}</p>
                        <label for="file-upload">
                            <p>{"Load JSON"}</p>
                        </label>
                        <input
                            id="file-upload"
                            type="file"
                            accept="application/json"
                            multiple={false}
                            onchange={ctx.link().clone().callback(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                upload_files(input.files())
                            })}
                        />
                        {

                            if self.files.is_empty() && is_error {
                                html! { <div id="error-area"><p>{message}</p></div> }
                            } else {
                                html! {}
                            }
                        }

                        {
                            if self.is_loading && !is_error {
                                html! {
                                    <div id="loading">
                                        <div class="fa fa-spinner fa-spin"></div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </>
            }
        } else {
            html! {
                <>
                    <Root scope={ctx.link()} />
                    <Suspense>
                        <Content files={self.files.clone()} />
                    </Suspense>
                </>
            }
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct ContentProps {
    files: Vec<FileDetails>,
}

#[function_component(Content)]
fn content(props: &ContentProps) -> HtmlResult {
    let current_page = use_state(|| 0);
    let page_size = get_page_size(&props.files[0].data);
    let onclick_prev = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            if *current_page == 0 {
                return;
            }
            current_page.set(*current_page - 1);
        })
    };
    let onclick_next = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            if *current_page >= page_size - 1 {
                return;
            }
            current_page.set(*current_page + 1);
        })
    };
    Ok(html! {
        <div id="json-area">
            <div class="json-tree-viewer">
                { for props.files.iter().map(|v| view_file(v, *current_page, page_size)) }
            </div>
            {
                if get_page_size(&props.files[0].data) > 10 {
                    html! {
                        <div class="json-pagination">
                            <button class="json-pagination-button" onclick={onclick_prev}>{"<"}</button>
                            <button class="json-pagination-button" onclick={onclick_next}>{">"}</button>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    })
}

#[derive(Properties)]
struct RootProps {
    scope: Scope<App>,
}

impl PartialEq for RootProps {
    fn eq(&self, other: &Self) -> bool {
        self.scope.get_component().unwrap().files == other.scope.get_component().unwrap().files
    }
}

impl Clone for RootProps {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.clone(),
        }
    }
}

#[function_component(Root)]
fn root(_: &RootProps) -> Html {
    use_favicon(
        "https://cdn4.iconfinder.com/data/icons/developer-vol-2/100/developer-1-27-512.png"
            .to_string(),
    );

    // Initialize theme based on system preference
    use_effect(|| {
        if super::json_tree_viewer::get_system_theme() {
            let document = web_sys::window().unwrap().document().unwrap();
            let html = document.document_element().unwrap();
            let btn_toggle = html.query_selector(".theme-toggle").unwrap().unwrap();

            html.set_class_name("dark");
            btn_toggle.set_inner_html("<i class=\"fa fa-lightbulb-o\"></i> Light");
        }
        || ()
    });

    let onclick = Callback::from(|_| {
        super::json_tree_viewer::toggle_theme();

        let document = web_sys::window().unwrap().document().unwrap();
        let html = document.document_element().unwrap();
        let btn_toggle = html.query_selector(".theme-toggle").unwrap().unwrap();
        if super::json_tree_viewer::get_theme() == "dark" {
            btn_toggle.set_inner_html("<i class=\"fa fa-lightbulb-o\"></i> Light");
        } else {
            btn_toggle.set_inner_html("<i class=\"fa fa-moon-o\"></i> Dark");
        }
    });

    html! {
        <>
            <button onclick={onclick} class="theme-toggle">
                <i class="fa fa-moon-o"></i>
                {"Theme"}
            </button>
        </>
    }
}
