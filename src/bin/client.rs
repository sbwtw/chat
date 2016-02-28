
#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate conrod;
extern crate piston_window;

use conrod::{
    color,
    Widget,
    Canvas,
    Text,
    TextBox,
    Theme,
    Sizeable,
    Colorable,
    Positionable,
};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

struct App {
    display: String,
}

impl App {
    fn new() -> App {
        App {
            //display: String::new(),
            display: "aaaaaaaaaa".to_owned(),
        }
    }

    fn set_widgets(&mut self, ui: &mut UiCell) {

        Canvas::new()
            .pad(8.0)
            .color(color::WHITE)
            .set(CANVAS, ui);

        Text::new(self.display.as_str())
            //.middle()
            //.up(0.0)
            .color(color::RED)
            .align_top_of(CANVAS)
            .set(TEXT_AREA, ui);

        TextBox::new(&mut self.display)
            .font_size(20)
            .w_h(200.0, 50.0)
            //.down_from(TEXT_AREA, 0.0)
            .align_bottom_of(CANVAS)
            .react(|_string: &mut String| {})
            .set(INPUT_BOX, ui);
    }
}

fn main() {
    let window: PistonWindow =
        WindowSettings::new("main window", [400, 300])
            .exit_on_esc(true).vsync(true).build().unwrap();

    let mut ui = {

        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&std::path::Path::new("/home/Projects/conrod/assets/fonts/NotoSans/NotoSans-Regular.ttf"), window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    let mut app = App::new();

    for event in window.ups(60) {
        ui.handle_event(&event);

        event.update(|_| ui.set_widgets(|mut ui| app.set_widgets(&mut ui)));
        event.draw_2d(|c, g| ui.draw_if_changed(c, g));
    }
}

widget_ids! {
    CANVAS,
    TEXT_AREA,
    INPUT_BOX,
}

