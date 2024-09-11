use nalgebra_glm::Vec2;

use crate::graphics::context2d::Context2D;

use super::{ray::Ray, voxel::Voxel};

pub struct StackElement<V> {
    voxel: Voxel<V>,
    t: Vec2,
}

pub struct Stack<V> {
    elements: Vec<StackElement<V>>,
}

impl Stack<Vec2> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, element: StackElement<Vec2>) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<StackElement<Vec2>> {
        self.elements.pop()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

pub fn traverse(ray: &Ray<Vec2>, voxel: Voxel<Vec2>, context: &Context2D) {
    // The minimum size that can reach a voxel.
    let min_voxel_size = 6.25;
    let mut voxel = voxel;

    // Contains tmin (x component) and tmax (y component).
    let mut t = voxel.ray_traverse(&ray);

    // Indicate if the previous voxel was poped (`true`)
    // or not (`false`)
    let mut poped = false;

    let mut stack = Vec::<StackElement<Vec2>>::new();

    // The ray don't hit the voxel so we
    // can return void.
    if t.x >= t.y {
        return;
    }

    loop {
        // The position at where the ray enter in the voxel.
        let ray_in = ray.get_origin() + ray.get_direction() * t.x;
        // Determine the position of the subvoxel.
        let subvoxel_pos = voxel.get_center() + voxel.dir_of(&ray_in) * voxel.get_size() * 0.25;

        if voxel.get_size() > min_voxel_size && !poped {
            // Create the subvoxel.
            let subvoxel = Voxel::new(subvoxel_pos, voxel.get_size() * 0.5);
            stack.push(StackElement::<Vec2> { voxel, t });

            context.draw_box(
                &voxel.get_center(),
                voxel.get_size(),
                Some("#0000000F".into()),
            );

            voxel = subvoxel;
            t = voxel.ray_traverse(&ray);

            continue;
        }

        if !poped {
            context.draw_box(&voxel.get_center(), voxel.get_size(), None);
        }

        if t.y >= stack[stack.len() - 1].t.y {
            poped = false;

            let parent = stack.pop().unwrap();

            if stack.len() == 0 {
                break;
            }

            let ray_out = ray.get_origin() + ray.get_direction() * (parent.t.y + 0.01);
            let subvoxel_pos = stack[stack.len() - 1].voxel.get_center()
                + stack[stack.len() - 1].voxel.dir_of(&ray_out) * parent.voxel.get_size() * 0.5;

            if subvoxel_pos == *parent.voxel.get_center() {
                poped = true;
            }

            let subvoxel = Voxel::new(subvoxel_pos, parent.voxel.get_size());
            voxel = subvoxel;
            t = voxel.ray_traverse(&ray);

            continue;
        }

        let ray_out = ray.get_origin() + ray.get_direction() * (t.y + 0.01);
        let subvoxel_pos = stack[stack.len() - 1].voxel.get_center()
            + stack[stack.len() - 1].voxel.dir_of(&ray_out) * voxel.get_size() * 0.5;

        let subvoxel = Voxel::new(subvoxel_pos, voxel.get_size());
        voxel = subvoxel;
        t = voxel.ray_traverse(&ray);
    }
}
