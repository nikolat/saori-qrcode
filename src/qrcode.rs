use image::Luma;
use qrcode::QrCode;
use std::path::PathBuf;

pub(crate) fn get_image(text: &str, dist_path: &PathBuf) {
    let code = QrCode::new(text).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(dist_path).unwrap();
}

pub(crate) fn get_text(text: &str) -> String {
    let code = QrCode::new(text).unwrap();
    let string = code
        .render()
        .light_color(' ')
        .dark_color('#')
        .build()
        .escape_debug()
        .to_string();
    return string;
}
