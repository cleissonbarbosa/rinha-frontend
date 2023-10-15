use std::collections::HashMap;

use gloo::file::{callbacks::FileReader, File};
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::html::Scope;
use yew::prelude::*;
use yew::{html, html::TargetCast, Callback, Component, Context, Html};
use yew_hooks::prelude::*;

use super::super::services::upload_file::upload_files;
use super::json_tree_viewer::view_file;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
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
                                if let Some(reader) =
                                    serde_json::from_slice::<serde_json::Value>(&data).err()
                                {
                                    link.send_message(Msg::Error(format!(
                                        "failed to parse file: {}",
                                        reader
                                    )));
                                    return;
                                }
                                link.send_message(Msg::Loaded(file_name.clone(), file_type, data));
                            }
                            Err(_) => {
                                link.send_message(Msg::Error("failed to read file".to_string()));
                            }
                        });
                    self.readers.insert(file.name(), reader);
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
                    <Root scope={ctx.clone().link()} />
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
                    <Root scope={ctx.clone().link()} />
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
    Ok(html! {
        <div id="json-area">
            { for props.files.iter().map(view_file) }
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
    use_favicon("https://crates.io/favicon.ico".to_string());

    html! {
        <span></span>
    }
}