use std;
use std::collections::{HashMap, HashSet};
use marker::*;
use marker::geom::Rect;
use super::MarkedTiles;

use rand;

#[derive(Debug)]
struct GroundIndex {
    vertical: HashMap<Vertical, Vec<Rect>>,
    horizontal: HashMap<Horizontal, Vec<Rect>>,
}
#[derive(Debug)]
struct PlatformIndex {
    horizontal: HashMap<Horizontal, Vec<Rect>>,
}
#[derive(Debug)]
struct ObjectIndex {
    ground: Vec<Rect>,
    surface: Vec<Rect>,
}

#[derive(Debug)]
pub struct LevelAssetIndex {
    ground: GroundIndex,
    platforms: PlatformIndex,
    objects: ObjectIndex,
}

impl LevelAssetIndex {
    pub fn build(ground: &MarkedTiles, objects: &MarkedTiles) -> LevelAssetIndex {
        let mut ground_ver: HashMap<Vertical, Vec<Rect>> = HashMap::with_capacity(3);
        let mut ground_hor: HashMap<Horizontal, Vec<Rect>> = HashMap::with_capacity(3);
        let mut platform_hor: HashMap<Horizontal, Vec<Rect>> = HashMap::with_capacity(3);
        let mut ground_obj = vec![];
        let mut surface_obj = vec![];

        for gd in ground.data.iter() {
            match &gd.markers {
                &SpriteType::Ground {
                    horizontal: ref hor,
                    vertical: ref ver,
                } => {
                    for h in hor.iter() {
                        let mut p = true;
                        {
                            let entry = ground_hor.get_mut(h);
                            if let Some(e) = entry {
                                e.push(gd.on_screen_frame.clone());
                                p = false;
                            };
                        }
                        if p {
                            ground_hor.insert(h.clone(), vec![gd.on_screen_frame.clone()]);
                        };
                    }
                    for v in ver.iter() {
                        let mut p = true;
                        {
                            let entry = ground_ver.get_mut(v);
                            if let Some(e) = entry {
                                e.push(gd.on_screen_frame.clone());
                                p = false;
                            };
                        }
                        if p {
                            ground_ver.insert(v.clone(), vec![gd.on_screen_frame.clone()]);
                        };
                    }
                }
                &SpriteType::Platform { horizontal: ref hor } => {
                    for h in hor.iter() {
                        platform_hor.entry(h.clone()).or_insert({
                            vec![gd.on_screen_frame.clone()]
                        });
                    }
                }
                &SpriteType::Object => ground_obj.push(gd.on_screen_frame.clone()),
            }
        }

        for od in objects.data.iter() {
            match od.markers {
                SpriteType::Object => surface_obj.push(od.on_screen_frame.clone()),
                _ => (),
            }
        }

        let index = LevelAssetIndex {
            ground: GroundIndex {
                vertical: ground_ver,
                horizontal: ground_hor,
            },
            objects: ObjectIndex {
                ground: ground_obj,
                surface: surface_obj,
            },
            platforms: PlatformIndex { horizontal: platform_hor },
        };

        println!("{:?}", index);
        index
    }

    pub fn find_ground(&self, hor: Horizontal, ver: Vertical) -> Option<Rect> {
        let mut result = vec![];

        for opt_vec_h in self.ground.horizontal.get(&hor).iter() {
            for h in opt_vec_h.iter() {
                for opt_vec_v in self.ground.vertical.get(&ver).iter() {
                    for v in opt_vec_v.iter() {
                        if v == h {
                            result.push(v.clone())
                        };
                    }
                }
            }
        }

        println!("{:?}", result);

        random_from(&result)
    }
    pub fn find_platform(&self, hor: Horizontal) -> Option<Rect> {
        self.platforms.horizontal.get(&hor).and_then(random_from)
    }

    pub fn find_object(&self, surface: bool) -> Option<Rect> {
        let r = if surface {
            &self.objects.surface
        } else {
            &self.objects.ground
        };
        random_from(&r)
    }
}


fn random_from<T: Clone>(from: &Vec<T>) -> Option<T> {
    if from.len() > 0 {
        let ix = rand::random::<usize>() % from.len();
        Some(from[ix].clone())
    } else {
        None
    }
}