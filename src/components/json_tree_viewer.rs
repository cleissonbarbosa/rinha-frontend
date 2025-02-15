use super::root::FileDetails;
use yew::{html, Html};

pub fn get_page_size(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Array(a) => {
            if a.len() == 1 {
                if a[0].is_array() {
                    return a[0].as_array().unwrap().len();
                } else if a[0].is_object() {
                    return a[0].as_object().unwrap().len();
                }
            }
            a.len()
        }
        serde_json::Value::Object(o) => {
            if o.len() == 1 {
                if o.values().next().unwrap().is_array() {
                    return o.values().next().unwrap().as_array().unwrap().len();
                } else if o.values().next().unwrap().is_object() {
                    return o.values().next().unwrap().as_object().unwrap().len();
                }
            }
            o.len()
        }
        _ => 1,
    }
}

pub fn view_file(file: &FileDetails, page: usize, page_size: usize) -> Html {
    let content = &file.data.clone();
    let content_paged = page_size > 10;

    let content_vec: Vec<serde_json::Value> = match content_paged {
        true => page_json(content, page_size),
        false => vec![content.clone()],
    };

    let highlighted_content = match &content_vec[page] {
        serde_json::Value::String(s) => {
            html! { <span class="json-string">{ s }</span> }
        }
        serde_json::Value::Array(a) => {
            html! {
                <ul class="json-array">
                    { for a.iter().map(view_json_value) }
                </ul>
            }
        }
        serde_json::Value::Object(o) => {
            html! {
                <ul class="json-object">
                    { for o.iter().map(|(k, v)| view_json_property(k, v)) }
                </ul>
            }
        }
        v => {
            html! { <span>{ serde_json::to_string_pretty(&v).unwrap() }</span> }
        }
    };

    html! {
        <div class="json">
            <p class="json-title">{ format!("{}", file.name) }</p>
            <div class="json-content lazy">
                { highlighted_content }
            </div>
            {
                if content_paged {
                    html! {
                        <div class="json-pagination-stats">
                            <p>{format!("Page {} of {}", page + 1, page_size)}</p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

        </div>
    }
}

fn view_json_property(key: &str, value: &serde_json::Value) -> Html {
    html! {
        <li class="json-property">
            <span class="json-key">{ key }</span>
            <span class="json-separator">{": "}</span>
            { view_json_value(value) }
        </li>
    }
}

fn view_json_value(value: &serde_json::Value) -> Html {
    match value {
        serde_json::Value::String(s) => {
            html! { <span class="json-string">{ format!("\"{}\"", s) }</span> }
        }
        serde_json::Value::Array(a) => {
            let mut index = 0;
            html! {
                <span class="json-array">
                    <p class="f-brackets">{ "["}</p>
                        {

                            for a.iter().map(|v| {
                                let item_index = index; // Captura o índice atual
                                index += 1; // Incrementa o índice
                                html!{
                                    <p class="json-array-item">
                                        <span class="json-array-item-idx"> { format!("{}: ", item_index) } </span>
                                        <span class="json-array-item-value"> { view_json_value(v) } </span>
                                    </p>
                                }
                            })
                        }
                    <p class="l-brackets">{ "]"}</p>
                </span>
            }
        }
        serde_json::Value::Object(o) => {
            html! {
                <ul class="json-object">
                    { for o.iter().map(|(k, v)| view_json_property(k, v)) }
                </ul>
            }
        }
        _ => {
            html! { <span>{ format!("{}", value) }</span> }
        }
    }
}

//page the json and save the parts in memory
fn page_json(value: &serde_json::Value, page_size: usize) -> Vec<serde_json::Value> {
    let mut pages = Vec::new();
    let mut page = Vec::new();
    let mut index = 0;
    match value {
        serde_json::Value::Array(a) => {
            for v in a.iter() {
                if a.len() == 1 && v.is_array() {
                    for b in v.as_array().unwrap().iter() {
                        pages.push(serde_json::Value::Array([(b.clone())].to_vec()));
                        index += 1;
                        if index == page_size {
                            page = Vec::new();
                            index = 0;
                        }
                    }
                } else {
                    pages.push(v.clone());
                    index += 1;
                    if index == page_size {
                        page = Vec::new();
                        index = 0;
                    }
                }
            }
        }
        serde_json::Value::Object(o) => {
            for (k, v) in o.iter() {
                if o.len() == 1 && v.is_array() {
                    for a in v.as_array().unwrap().iter() {
                        pages.push(serde_json::Value::Object(
                            [(k.clone(), a.clone())].iter().cloned().collect(),
                        ));
                        index += 1;
                        if index == page_size {
                            page = Vec::new();
                            index = 0;
                        }
                    }
                } else {
                    page.push(serde_json::Value::Object(
                        [(k.clone(), v.clone())].iter().cloned().collect(),
                    ));
                    index += 1;
                    if index == page_size {
                        pages.push(serde_json::Value::Array(page));
                        page = Vec::new();
                        index = 0;
                    }
                }
            }
        }
        _ => {}
    }
    if !page.is_empty() {
        pages.push(serde_json::Value::Array(page));
    }
    pages
}

pub fn toggle_theme() {
    let document = web_sys::window().unwrap().document().unwrap();
    let html = document.document_element().unwrap();

    let class_name = html.get_attribute("class").unwrap_or_default();
    if class_name.contains("dark") {
        html.set_attribute("class", &class_name.replace("dark", ""))
            .unwrap();
    } else {
        html.set_attribute("class", &format!("{} dark", class_name))
            .unwrap();
    }
}

pub fn get_system_theme() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
            return media_query.matches();
        }
    }
    false
}

pub fn get_theme() -> String {
    let document = web_sys::window().unwrap().document().unwrap();
    let html = document.document_element().unwrap();
    let class_name = html.get_attribute("class").unwrap_or_default();
    if class_name.contains("dark") {
        return "dark".to_string();
    }
    "light".to_string()
}
