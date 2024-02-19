use figlet_rs::FIGfont;

pub fn label(source: String) -> String {

    let font = FIGfont::standard().unwrap();
    let text = font.convert(&source).unwrap().to_string();
    text
}
