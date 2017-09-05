use super::Vector2;

#[derive(Debug, Clone)]
pub struct AABB {
    pub center: Vector2,
    half_size_internal: Vector2,
    pub scale: Vector2,
    pub offset: Vector2,
}

impl AABB {
    pub fn new_full(center: Vector2, full_size: Vector2, scale: Vector2) -> AABB {
        let half_size = full_size / 2.0;

        let offset_y = -half_size.y * (1.0 - scale.y);

        AABB {
            center,
            half_size_internal: half_size,
            scale,
            offset: Vector2::new(0.0, offset_y),
        }
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        !(self.center.x - other.center.x > self.half_size().x + other.half_size().x) &&
            !(self.center.y - other.center.y > self.half_size().y + other.half_size().y)
    }

    pub fn half_size(&self) -> Vector2 {
        Vector2::new(
            self.half_size_internal.x * self.scale.x,
            self.half_size_internal.y * self.scale.y,
        )
    }
}
