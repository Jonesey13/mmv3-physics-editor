use crate::image::EncodedImageCollection;
use crate::image::Image;
use crate::language::Language;

pub fn build_html_template() -> String {
    let images = EncodedImageCollection::build().images;

    let html = format!(
        include_str!("ui/index.html"),
        style = include_str!("ui/style.css"),
        script = format!(
            "{}\n{}",
            include_str!("ui/cash.min.js"),
            include_str!("ui/app.js")
        ),
        logo = images.get(&Image::Logo).unwrap(),
        english = images.get(&Image::Lang(Language::English)).unwrap(),
        french = images.get(&Image::Lang(Language::French)).unwrap(),
        german = images.get(&Image::Lang(Language::German)).unwrap(),
        italian = images.get(&Image::Lang(Language::Italian)).unwrap(),
        spanish = images.get(&Image::Lang(Language::Spanish)).unwrap(),
    );

    html
}