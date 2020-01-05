use winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("Micro.ico");
    res.compile().unwrap();
}