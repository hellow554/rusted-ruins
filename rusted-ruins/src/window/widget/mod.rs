
mod border;
mod guage;
mod image;
mod label;
mod list;

use game::Command;
use sdl2::render::WindowCanvas;
use sdlvalues::SdlValues;

pub trait WidgetTrait {
    type Response;
    fn process_command(&mut self, _command: &Command) -> Option<Self::Response> { None }
    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues);
}

pub use self::border::*;
pub use self::guage::*;
pub use self::image::*;
pub use self::label::*;
pub use self::list::*;

