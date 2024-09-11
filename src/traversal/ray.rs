use nalgebra_glm::Vec2;

pub struct Ray<V> {
    /// The ray origin.
    origin: V,
    /// The ray direction.
    direction: V,
    /// The ray inverse direction.
    inverse_direction: V,
}

impl Ray<Vec2> {
    /// Create a new [Ray].
    ///
    /// # Arguments
    ///
    /// * `origin` - The ray origin.
    /// * `direction` - The ray direction ***(must be normalized)***.
    ///
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        // Avoid division by zero.
        let direction = Vec2::new(
            direction.x.abs().max(0.01) * if direction.x < 0.0 { -1.0 } else { 1.0 },
            direction.y.abs().max(0.01) * if direction.y < 0.0 { -1.0 } else { 1.0 },
        );

        Self {
            origin,
            direction,
            inverse_direction: Vec2::new(1.0, 1.0).component_div(&direction),
        }
    }

    /// Retrieve a reference to the [ray](Ray) origin.
    pub fn get_origin(&self) -> &Vec2 {
        &self.origin
    }

    /// Retrieve a reference to the [ray](Ray) direction.
    pub fn get_direction(&self) -> &Vec2 {
        &self.direction
    }

    /// Retrieve a reference to the inverse [ray](Ray) direction.
    pub fn get_inverse_direction(&self) -> &Vec2 {
        &self.inverse_direction
    }
}
