use super::app::FileDetails;
use yew::{html, Html};

pub fn view_file(file: &FileDetails) -> Html {
    let content = String::from_utf8_lossy(&file.data);
    let content = serde_json::from_str::<serde_json::Value>(&content);

    let highlighted_content = match &content.unwrap() {
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
            <div class="json-content">
                { highlighted_content }
            </div>
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
