use ggez::graphics::Image;
use ggez::{Context, GameResult};

use std::rc::Rc;

use sprite::MarkedTiles;
use sprite::Loader;
use marker::*;
use marker::geom::*;

pub mod index;

use ggez::graphics::DrawParam;
use ggez::graphics;

use self::index::LevelAssetIndex;

pub enum LevelType {
    Graveyard,
}

pub struct LevelAssets {
    pub ground: MarkedTiles,
    pub objects: MarkedTiles,
    pub background: Image,
}

impl LevelAssets {
    pub fn load_assets<'a>(ctx: &mut Context, tpe: LevelType) -> GameResult<LevelAssets> {
        let (g, o, bg) = match tpe {
            LevelType::Graveyard => {
                let g = Loader::load_sprite_sheet(ctx, "/level/graveyard/level_ground")?;
                let o = Loader::load_sprite_sheet(ctx, "/level/graveyard/level_objects")?;
                let bg = Image::new(ctx, "/level/graveyard/background.png")?;
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
    pub terrain: Vec<Vec<usize>>,
    pub assets: LevelAssets,
    pub index: LevelAssetIndex,
}

impl Level {
    pub fn load(ctx: &mut Context, lt: LevelType) -> GameResult<Level> {
        let assets = LevelAssets::load_assets(ctx, lt)?;
        let terrain = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 5, 5, 0, 0, 1],
            vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 1, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let index = LevelAssetIndex::build(&assets.ground, &assets.objects);

        Ok(Level {
            assets,
            terrain,
            index,
        })
    }
}

pub struct TileAttributes {
    ground: bool,
}

pub struct RenderableLevel {
    pub level: Level,
    pub sprites: Vec<(Rc<Image>, DrawParam, TileAttributes)>,
}

impl RenderableLevel {
    pub fn build(l: Level) -> RenderableLevel {
        let mut sprites: Vec<(Rc<Image>, DrawParam, TileAttributes)> = vec![];

        {
            let height = l.terrain.len();
            let width = l.terrain[0].len();

            let is_left_wall = |h| h == 0;
            let is_right_wall = |h| h == width - 1;
            let is_floor = |v| v == height - 1;
            let is_roof = |v| v == 0;
            let is_corner =
                |h, v| (is_left_wall(h) || is_right_wall(h)) && (is_floor(v) || is_roof(v));

            let lookup = |(h, v, t): (usize, usize, &Vec<Vec<usize>>)| -> Option<Rect> {
                let it = t[v][h];
                if it == 0 {
                    None
                } else {
                    let some_above = t[v - 1][h] != 0;
                    let some_below = t[v + 1][h] != 0;
                    let some_on_left = t[v][h - 1] != 0;
                    let some_on_right = t[v][h + 1] != 0;

                    let hor = if some_on_left && some_on_right {
                        Horizontal::Center
                    } else if some_on_left {
                        Horizontal::Right
                    } else {
                        Horizontal::Left
                    };


                    let ver = if some_above && some_below {
                        Vertical::Center
                    } else if some_above {
                        Vertical::Bottom
                    } else {
                        Vertical::Top
                    };

                    l.index.find_ground(hor, ver)
                }
            };

            let t: &Vec<Vec<usize>> = &l.terrain;

            for h in 0..width {
                for v in 0..height {
                    let rect = if is_corner(h, v) {
                        l.index.find_ground(Horizontal::Center, Vertical::Center)
                    } else if is_left_wall(h) {
                        l.index.find_ground(Horizontal::Right, Vertical::Center)
                    } else if is_right_wall(h) {
                        l.index.find_ground(Horizontal::Left, Vertical::Center)
                    } else if is_floor(v) {
                        l.index.find_ground(Horizontal::Center, Vertical::Top)
                    } else if is_roof(v) {
                        l.index.find_ground(Horizontal::Center, Vertical::Bottom)
                    } else {
                        lookup((h, v, t))
                    };

                    if let Some(rect) = rect {
                        sprites.push((
                            l.assets.ground.image.clone(),
                            DrawParam {
                                src: graphics::Rect::from(rect),
                                dest: graphics::Point::new((h * 64) as f32, (v * 64) as f32),
                                scale: graphics::Point::new(0.5, 0.5),
                                ..Default::default()
                            },
                            TileAttributes { ground: true },
                        ));
                    };
                }
            }
        }
        RenderableLevel { sprites, level: l }
    }
}
