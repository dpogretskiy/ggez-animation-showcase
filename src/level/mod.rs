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
            vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
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
    pub fn build(level: Level) -> RenderableLevel {
        let mut sprites: Vec<(Rc<Image>, DrawParam, TileAttributes)> = vec![];

        {
            let height = level.terrain.len();
            let pixel_height = height * 128;
            let width = level.terrain[0].len();

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
                    let itself = t[v][h];
                    let above = t[v - 1][h];
                    let below = t[v + 1][h];
                    let on_left = t[v][h - 1];
                    let on_right = t[v][h + 1];
                    let above_right = t[v - 1][h + 1];
                    let above_left = t[v - 1][h - 1];
                    let bottom_left = t[v + 1][h - 1];
                    let bottom_right = t[v + 1][h + 1];

                    let mat = (
                        (above_left, above, above_right),
                        (on_left, itself, on_right),
                        (bottom_left, below, bottom_right),
                    );

                    match mat {
                        ((l, 1, r), (1, 1, 1), (_, 1, _)) => if l != 0 && r != 0 {
                            level.index.find_ground(Square::MM)
                        } else if r != 0 {
                            level.index.find_ground(Square::IBR)
                        } else {
                            level.index.find_ground(Square::IBL)
                        },
                        ((l, 0, r), (1, 1, 1), (_, 1, _)) => if l != 0 {
                            level.index.find_ground(Square::ILT)
                        } else if r != 0 {
                            level.index.find_ground(Square::IRT)
                        } else {
                            level.index.find_ground(Square::MT)
                        },
                        ((_, a, _), (l, 1, r), (_, b, _)) => if a == 0 {
                            if l == 0 {
                                level.index.find_ground(Square::LT)
                            } else if r == 0 {
                                level.index.find_ground(Square::RT)
                            } else {
                                level.index.find_ground(Square::MT)
                            }
                        } else if b == 0 {
                            if l == 0 {
                                level.index.find_ground(Square::LB)
                            } else if r == 0 {
                                level.index.find_ground(Square::RB)
                            } else {
                                level.index.find_ground(Square::MB)
                            }
                        } else {
                            if l == 0 {
                                level.index.find_ground(Square::LM)
                            } else if r == 0 {
                                level.index.find_ground(Square::RM)
                            } else {
                                level.index.find_ground(Square::MM)
                            }
                        },
                        _ => None,
                    }
                }
            };

            let t: &Vec<Vec<usize>> = &level.terrain;

            for h in 0..width {
                for v in 0..height {
                    let rect: Option<Rect> = if is_left_wall(h) && is_floor(v) {
                        level.index.find_ground(Square::IBL)
                    } else if is_floor(v) && h == 1 {
                        level.index.find_ground(Square::ILT)
                    } else if is_floor(v) && h == (width - 2) {
                        level.index.find_ground(Square::IRT)
                    } else if is_right_wall(h) && is_floor(v) {
                        level.index.find_ground(Square::IBR)
                    } else if is_corner(h, v) {
                        level.index.find_ground(Square::MM)
                    } else if is_left_wall(h) {
                        if t[v][h + 1] != 0 {
                            level.index.find_ground(Square::MM)
                        } else {
                            level.index.find_ground(Square::RM)
                        }
                    } else if is_right_wall(h) {
                        if t[v][h - 1] != 0 {
                            level.index.find_ground(Square::MM)
                        } else {
                            level.index.find_ground(Square::LM)
                        }
                    } else if is_floor(v) {
                        if t[v - 1][h] != 0 {
                            level.index.find_ground(Square::MM)
                        } else {
                            level.index.find_ground(Square::MT)
                        }
                    } else if is_roof(v) {
                        level.index.find_ground(Square::MB)
                    } else {
                        lookup((h, v, t))
                    };

                    if let Some(rect) = rect {
                        sprites.push((
                            level.assets.ground.image.clone(),
                            DrawParam {
                                src: graphics::Rect::from(rect),
                                dest: graphics::Point::new((h * 128) as f32, pixel_height as f32 - (v * 128) as f32),
                                scale: graphics::Point::new(1.0, 1.0),
                                ..Default::default()
                            },
                            TileAttributes { ground: true },
                        ));
                    };
                }
            }
        };
        RenderableLevel { sprites, level }
    }
}
