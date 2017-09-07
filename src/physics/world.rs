use super::*;
use camera::Camera;
use player::Player;
use self::quad_tree::*;

pub struct World {
    players: Vec<Player>,
}

impl World {
    pub fn new() -> World {
        World { players: vec![] }
    }

    pub fn add_player(&mut self, p: Player) {
        self.players.push(p);
    }

    pub fn update_areas(&mut self, camera: &Camera) {
        let vs = camera.size();
        let vc = camera.location();

        let xy = vc - vs * 0.666;

        let rect = quad_tree::Rect::new(xy.x, xy.y, vs.x * 1.5, vs.y * 1.5);
        // let qt = quad_tree::QuadTree::new(0, rect);
    }
}
