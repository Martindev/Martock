//! Camera is responsible for keeping track of the current viewing window. It is not coupled to the
//! player and may move independently.

use piston::input::Button;
use piston::input::Key;
use piston::input::Input;

use interactive;
use point::Point;

// The amount of pixels by which the camera is nudged by input keys.
// TODO(jacob-zimmerman): Implement natural camera motion in response to input.
const NUDGE: f64 = 0.1;

/// View represents a focal area of the world.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct View {
    pub top_left: Point,

    /// width of window in blocks.
    pub width: usize,
    /// height of window in blocks.
    pub height: usize,
}

impl View {
    fn new(x: f64, y: f64, width: usize, height: usize) -> Self {
        View {
            top_left: Point::from(x, y),
            width: width,
            height: height,
        }
    }

    fn renew(&mut self, xdiff: f64, ydiff: f64) {
        *self = View::new(self.top_left.x() + xdiff,
                          self.top_left.y() + ydiff,
                          self.width,
                          self.height)
    }

    fn move_up(&mut self) {
        self.renew(0.0, -NUDGE)
    }

    fn move_down(&mut self) {
        self.renew(0.0, NUDGE)
    }

    fn move_right(&mut self) {
        self.renew(NUDGE, 0.0)
    }

    fn move_left(&mut self) {
        self.renew(-NUDGE, 0.0)
    }
}

/// Camera represents a viewing window with a position which is the top left of the viewing window,
/// and dimensions in blocks.
pub struct Camera {
    view: View,
}

impl Camera {
    pub fn new(x: f64, y: f64, width: usize, height: usize) -> Self {
        Camera { view: View::new(x, y, width, height) }
    }

    /// view returns the focal area.
    pub fn view(&self) -> View {
        self.view
    }
}

impl interactive::Interactive for Camera {
    fn interact(&mut self, i: &Input) {
        if let Input::Press(b) = *i {
            if let Button::Keyboard(k) = b {
                match k {
                    Key::Left => self.view.move_left(),
                    Key::Right => self.view.move_right(),
                    Key::Up => self.view.move_up(),
                    Key::Down => self.view.move_down(),
                    _ => (),
                }
            }
        }
    }
}
