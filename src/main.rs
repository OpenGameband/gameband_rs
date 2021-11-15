use fltk::{prelude::*, *};
use fltk::button::Button;
use fltk::window::DoubleWindow;
use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;

#[tokio::main]
async fn main(){
    let app = app::App::default();
    let mut wind: DoubleWindow = window::Window::new(400, 300, 800, 600, "Gameband");
    wind.set_color(fltk::enums::Color::rgb_color(35,35,35));
    let mut btn: Button = button::Button::new(50,50,150,120,"bean");
    btn.set_callback(move |_| println!("hello"));
    wind.end();
    wind.show();
    app.run().unwrap();
}

async fn dingus() {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}