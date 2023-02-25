use plotters::prelude::*;
use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::status::NotFound;

#[get("/plot")]
pub async fn get_plot() -> Result<NamedFile, NotFound<String>> {
    let path: String = String::from("static/1.png");
    // Create a 800*600 bitmap and start drawing
    let mut backend = BitMapBackend::new(&path, (300, 200));
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    backend
        .draw_rect((50, 50), (200, 150), &RED, true)
        .map_err(|err| println!("{:?}", err))
        .ok();
    backend.present().map_err(|err| println!("{:?}", err)).ok();
    println!("HELLO");
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}
