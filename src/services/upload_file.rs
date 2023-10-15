use super::super::components::app::Msg;
use gloo::file::File;
use web_sys::FileList;

pub fn upload_files(files: Option<FileList>) -> Msg {
    let mut result = Vec::new();

    if let Some(files) = files {
        let files = js_sys::try_iter(&files)
            .unwrap()
            .unwrap()
            .map(|v| web_sys::File::from(v.unwrap()))
            .map(File::from)
            .collect::<Vec<File>>();

        for file in files.clone() {
            if !file.raw_mime_type().contains("json") {
                return Msg::Error(format!(
                    "File {} is not a JSON file",
                    file.name()
                ));
            }
        }
        result.extend(files);
    }
    Msg::Files(result)
}
