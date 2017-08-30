use ggez::graphics::Image;
use ggez::Context;

use marker::SpriteData;
use sprite::MarkedTiles;

pub enum LevelType {
    Graveyard,
}

pub struct LevelAssets {
    ground: MarkedTiles,
    objects: MarkedTiles,
    background: Image,
}

impl LevelAssets {
    pub fn load_sprite_sheet(ctx: &mut Context, tpe: LevelType) -> LevelAssets {
        let location = match tpe {
            LevelType::Graveyard => unimplemented!(),
        };

        unimplemented!()
    }
}
