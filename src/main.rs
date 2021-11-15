use fltk::{prelude::*, *};
use fltk::app::screen_size;
use fltk::button::Button;
use fltk::group::Group;
use fltk::window::{DoubleWindow};

#[tokio::main]
async fn main(){
    let app = app::App::default();
    let (w,h) = screen_size();
    let mut wind: DoubleWindow = window::Window::new((w as i32/2)-400, (h as i32/2)-300, 800, 600, "Gameband");
    wind.set_color(fltk::enums::Color::rgb_color(35,35,35));
    let mut play_btn: Button = button::Button::new(100, 250, 150, 120, "Play Minecraft");
    let mut lightfurnace_btn: Button = button::Button::new(310,250,150,120,"PixelForge");

    let mut settings_window: Group = group::Group::new(0,0,800,600, "Settings");
    settings_window.end();
    settings_window.hide();

    setMenubar();
    play_btn.set_callback(move |_| println!("hello"));
    lightfurnace_btn.set_callback(move |_| println!("lightfurnacebutton"));
    play_btn.set_color(fltk::enums::Color::rgb_color(100,100,100));
    wind.end();
    wind.show();
    app.run().unwrap();
}

#[cfg(target_os = "macos")]
fn setMenubar() {

}