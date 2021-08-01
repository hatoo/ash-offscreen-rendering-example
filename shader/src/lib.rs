#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std::glam::{vec3, vec4, Vec3, Vec4};

#[spirv(fragment)]
pub fn main_fs(output: &mut Vec4, color: Vec3) {
    *output = color.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
    color: &mut Vec3,
) {
    *out_pos = vec4(
        (vert_id - 1) as f32,
        ((vert_id & 1) * 2 - 1) as f32,
        0.0,
        1.0,
    );

    *color = [
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 1.0),
    ][vert_id as usize];
}

#[spirv(miss)]
pub fn main_miss(#[spirv(incoming_ray_payload)] out: &mut Vec3) {
    *out = vec3(1.0, 1.0, 1.0);
}

#[spirv(closest_hit)]
pub fn main_closest_hit(
    #[spirv(incoming_ray_payload)] out: &mut Vec3,
    #[spirv(instance_id)] id: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 2)] colors: &[Vec3; 3],
    #[spirv(hit_attribute)] attribs: &Vec3,
) {
    // let barycentrics = vec3(1.0 - attribs.x - attribs.y, attribs.x, attribs.y);
    // *out = *attribs;
    *out = colors[id as usize];
}
