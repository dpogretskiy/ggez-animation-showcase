use ggez::graphics::Image;
use ggez::{ Context, GameResult };

use sprite::MarkedTiles;
use sprite::Loader;

pub enum LevelType {
    Graveyard,
}

pub struct LevelAssets {
    ground: MarkedTiles,
    objects: MarkedTiles,
    background: Image,
}

impl LevelAssets {
    pub fn load_assets(ctx: &mut Context, tpe: LevelType) -> GameResult<LevelAssets> {
        let (g, o, bg) = match tpe {
            LevelType::Graveyard => {
                let g = Loader::load_sprite_sheet(ctx, "/graveyard/ground")?;
                let o = Loader::load_sprite_sheet(ctx, "/graveyard/objects")?;
                let bg = Image::new(ctx, "/graveyard/background.png")?;
                (g, o, bg)
            }
        };
        Ok(LevelAssets {
            ground: g,
            objects: o,
            background: bg,
        })
    }
}

pub struct Level {
    terrain: Vec<Vec<usize>>,
    assets: LevelAssets
}