use super::*;
use camera::Camera;
use player::Player;
use game_box::GameBox;
use self::quad_tree::*;

pub struct World {
    players: Vec<Player>,
    boxes: Vec<GameBox>,
}

impl World {
    pub fn new() -> World {
        World {
            players: vec![],
            boxes: vec![],
        }
    }

    pub fn add_player(&mut self, p: Player) {
        self.players.push(p);
    }

    pub fn add_box(&mut self, b: GameBox) {
        self.boxes.push(b)
    }

    pub fn update_areas(&mut self, camera: &Camera) {
        let vs = camera.size();
        let vc = camera.location();
        let xy = vc - vs * 0.666;
        let rect = quad_tree::Rect::new(xy.x, xy.y, vs.x * 1.5, vs.y * 1.5);

        // let qt = quad_tree::QuadTree::new(0, rect);
    }
}
