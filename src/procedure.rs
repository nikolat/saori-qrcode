use std::path::PathBuf;

use crate::qrcode::{get_image, get_text};
use crate::request::*;
use crate::response::*;

/// load時に呼ばれる関数
pub fn load(_path: &str) {}

/// unload時に呼ばれる関数
pub fn unload(_path: &str) {}

/// request GET Version時に呼ばれる関数
pub fn get_version(_path: &str, _request: &SaoriRequest, response: &mut SaoriResponse) {
    response.set_result(String::from(env!("CARGO_PKG_VERSION")));
}

/// request EXECUTE時に呼ばれる関数
/// メインの処理はここに記述する
pub fn execute(path: &str, request: &SaoriRequest, response: &mut SaoriResponse) {
    let args = request.argument();
    let mut path = PathBuf::from(path);
    if !path.is_dir() {
        path.pop();
    }

    if let Some(func) = args.first() {
        match func.as_str() {
            "image" => {
                if let (Some(text), Some(output_path_str)) = (args.get(1), args.get(2)) {
                    let output_path = path.join(output_path_str);
                    get_image(text, &output_path);
                    response.set_result("".to_string());
                }
            }
            "text" => {
                if let Some(text) = args.get(1) {
                    let v = get_text(text);
                    response.set_result(v);
                }
            }
            _ => {}
        }
    }
}
