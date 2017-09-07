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

use na::Vector2;

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
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let index = LevelAssetIndex::build(&assets.ground, &assets.objects);

        Ok(Level {
            assets,
            terrain,
            index,
        })
    }
}

pub struct Terrain {
    pub terrain: Vec<Vec<TileType>>,
    pub position: Vector2<f64>,
    pub width: usize,
    pub height: usize,
    pub tile_size: f64,
}

impl Terrain {
    pub fn get_tile_at_point(&self, point: Vector2<f64>) -> Vector2<isize> {
        Vector2::new(
            ((point.x - self.position.x as f64 + self.tile_size / 2.0) / self.tile_size) as isize,
            ((point.y - self.position.y as f64 + self.tile_size / 2.0) / self.tile_size) as isize,
        )
    }

    pub fn get_tile_y_at_point(&self, y: f64) -> isize {
        ((y - self.position.y + self.tile_size / 2.0) / self.tile_size) as isize
    }

    pub fn get_tile_x_at_point(&self, x: f64) -> isize {
        ((x - self.position.x + self.tile_size / 2.0) / self.tile_size) as isize
    }

    pub fn get_map_tile_position(&self, x: isize, y: isize) -> Vector2<f64> {
        Vector2::new(
            x as f64 * self.tile_size + self.position.x,
            y as f64 * self.tile_size + self.position.y,
        )
    }

    pub fn get_map_tile_position_vec(&self, coords: Vector2<isize>) -> Vector2<f64> {
        self.get_map_tile_position(coords.x, coords.y)
    }

    #[inline]
    fn in_bounds(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    pub fn get_tile(&self, x: isize, y: isize) -> TileType {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.terrain[y][x]
        } else {
            TileType::Block
        }
    }

    pub fn is_obstacle(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) == TileType::Block
    }

    pub fn is_ground(&self, x: isize, y: isize) -> bool {
        if let Some((x, y)) = self.in_bounds(x, y) {
            let t = self.terrain[y][x];
            t == TileType::Block || t == TileType::OneWay
        } else {
            false
        }
    }

    pub fn is_one_way_platform(&self, x: isize, y: isize) -> bool {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.terrain[y][x] == TileType::OneWay
        } else {
            false
        }
    }

    pub fn is_empty(&self, x: isize, y: isize) -> bool {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.terrain[y][x] == TileType::Empty
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Empty,
    Block,
    OneWay,
}

pub struct RenderableLevel {
    pub level: Level,
    pub sprites: Vec<(Rc<Image>, DrawParam)>,
    pub terrain: Terrain,
}

impl RenderableLevel {
    pub fn build(level: Level) -> RenderableLevel {
        let mut sprites: Vec<(Rc<Image>, DrawParam)> = vec![];
        let mut terrain_vec: Vec<Vec<TileType>> = vec![];

        let height = level.terrain.len();
        let pixel_height = height * 128;
        let width = level.terrain[0].len();

        for v_vec in level.terrain.iter() {
            let mut h_vec = vec![];

            for tile in v_vec.iter() {
                match tile {
                    &0 => h_vec.push(TileType::Empty),
                    &1 => h_vec.push(TileType::Block),
                    &2 => h_vec.push(TileType::OneWay),
                    _ => h_vec.push(TileType::Empty),
                }
            }


            h_vec.shrink_to_fit();
            terrain_vec.push(h_vec);
        }

        {
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

                    if it == 1 {
                        match mat {
                            ((l, 1, r), (1, 1, 1), (_, 1, _)) => {
                                if l != 0 && r != 0 {
                                    level.index.find_ground(Square::MM)
                                } else if r != 0 {
                                    level.index.find_ground(Square::IBR)
                                } else {
                                    level.index.find_ground(Square::IBL)
                                }
                            }
                            ((l, 0, r), (1, 1, 1), (_, 1, _)) => {
                                if l != 0 {
                                    level.index.find_ground(Square::ILT)
                                } else if r != 0 {
                                    level.index.find_ground(Square::IRT)
                                } else {
                                    level.index.find_ground(Square::MT)
                                }
                            }
                            ((_, a, _), (l, 1, r), (_, b, _)) => {
                                if a == 0 {
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
                                }
                            }
                            _ => None,
                        }
                    } else if it == 2 {
                        if on_left != 0 && on_right != 0 {
                            level.index.find_platform(Horizontal::Center)
                        } else if on_left == 0 {
                            level.index.find_platform(Horizontal::Left)
                        } else if on_right == 0 {
                            level.index.find_platform(Horizontal::Right)
                        } else {
                            level.index.find_platform(Horizontal::Center)
                        }
                    } else {
                        None
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
                                dest: graphics::Point::new(
                                    (h * 128) as f32,
                                    (pixel_height - v * 128) as f32,
                                ),
                                scale: graphics::Point::new(1.0, 1.0),
                                ..Default::default()
                            },
                        ));
                    };
                }
            }
        };

        terrain_vec.reverse();
        RenderableLevel {
            sprites: sprites,
            level,
            terrain: Terrain {
                terrain: terrain_vec,
                position: Vector2::new(0.0, 128.0),
                width: width,
                height: height,
                tile_size: 128.0,
            },
        }
    }
}
