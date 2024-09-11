use nalgebra_glm::{sign, Vec2};

use super::ray::Ray;

#[derive(Debug, Clone, Copy)]
/// Represent a single voxel.
pub struct Voxel<V> {
    /// The minimum edge of the [voxel](Voxel).
    min: V,
    /// The maximum edge of the [voxel](Voxel).
    max: V,
    /// The position of the center of the [voxel](Voxel).
    center: V,
    /// The [voxel](Voxel) size.
    size: f32,
}

impl Voxel<Vec2> {
    /// Create a new [Voxel].
    ///
    /// # Arguments
    ///
    /// * `center` - The position of the center of the voxel.
    /// * `size` - The size of the voxel *(not the extend)*.
    /// * `ray` - The ray that will traverse this voxel.
    pub fn new(center: Vec2, size: f32) -> Self {
        let half = Vec2::new(size, size) * 0.5;

        Self {
            center,
            size,
            min: center - half,
            max: center + half,
        }
    }

    /// Retrieve a reference to the [voxel](Voxel) center position.
    pub fn get_center(&self) -> &Vec2 {
        &self.center
    }

    /// Retrieve a reference to the minimum edge of the [voxel](Voxel).
    pub fn get_min(&self) -> &Vec2 {
        &self.min
    }

    /// Retrieve a reference to the maximum edge of the [voxel](Voxel).
    pub fn get_max(&self) -> &Vec2 {
        &self.max
    }

    /// Retrieve the size of the [voxel](Voxel).
    pub fn get_size(&self) -> f32 {
        self.size
    }

    /// Calculate the time that the ray reach the current [voxel](Voxel)
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray.
    ///
    pub fn ray_traverse(&self, ray: &Ray<Vec2>) -> Vec2 {
        let bmin = (self.min - self.center).component_mul(&sign(ray.get_direction())) + self.center;
        let bmax = (self.max - self.center).component_mul(&sign(ray.get_direction())) + self.center;

        let cmin = (bmin - ray.get_origin()).component_mul(ray.get_inverse_direction());
        let cmax = (bmax - ray.get_origin()).component_mul(ray.get_inverse_direction());

        let tmin = f32::min(cmin.max(), cmax.max()); // cmin.max().min(cmax.max());
        let tmax = f32::max(cmin.min(), cmax.min()); // cmin.min().max(cmax.min());

        Vec2::new(tmin, tmax)
    }

    /// Calculate the direction of a point relative
    /// to the center of the current [voxel](Voxel).
    ///
    /// # Arguments
    ///
    /// * `point` - The point.
    ///
    pub fn dir_of(&self, point: &Vec2) -> Vec2 {
        Vec2::new(
            if point.x < self.center.x { -1.0 } else { 1.0 },
            if point.y < self.center.y { -1.0 } else { 1.0 },
        )
    }
}
