use std::collections::HashMap;
use super::super::services::upload_file::upload_files;
use super::json_tree_viewer::view_file;

use gloo::file::callbacks::FileReader;
use gloo::file::File;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

pub struct FileDetails {
    pub name: String,
    pub file_type: String,
    pub data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
    Error(String)
}

pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    is_error: bool,
    is_loading: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
            is_error: false,
            is_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                self.files.push(FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                self.readers.remove(&file_name);
                self.is_error = false;
                self.is_loading = false;
                true
            }
            Msg::Files(files) => {
                self.is_loading = true;
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                    self.is_error = false;
                }
                self.is_loading = false;
                true
            }
            Msg::Error(_) => {
                self.files.clear();
                self.is_error = true;
                self.is_loading = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.files.is_empty() {
            html! {
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
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            upload_files(input.files())
                        })}
                    />
                    <div id="error-area">
                        { 
                            if self.files.is_empty() && self.is_error {
                                html! { <p>{"Error loading file"}</p> } 
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    {
                        if self.is_loading {
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
            }
        } else {
            html!{
                <div id="json-area">
                    { for self.files.iter().map(view_file) }
                </div>
            }
        }
    }
}
