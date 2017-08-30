use super::MarkedTiles;
use ggez::graphics::Rect;

pub struct Animated {
    pub marked_tiles: MarkedTiles,
    pub current_frame: usize,
    pub length: usize,
}

impl Animated {
    pub fn new(mt: MarkedTiles) -> Animated {
        let length = mt.data.len();

        Animated {
            marked_tiles: mt,
            current_frame: 0,
            length,
        }
    }

    pub fn cycle_frames(&mut self) {
        if self.next_frame() {
        } else {
            self.reset()
        }
    }

    pub fn is_over(&self) -> bool {
        self.current_frame + 1 >= self.length
    }

    pub fn reset(&mut self) {
        self.current_frame = 0
    }

    pub fn next_frame(&mut self) -> bool {
        let cf = self.current_frame;

        if cf < (self.length - 1) {
            self.current_frame = cf + 1;
            true
        } else {
            false
        }
    }

    pub fn current_frame_rect(&self) -> Rect {
        Rect::from(
            self.marked_tiles.data[self.current_frame]
                .on_screen_frame
                .clone(),
        )
    }
}
