use crate::utils::constants::PATCH_SIZE;

pub fn oja_learning_rule(patch_x: &[f32; PATCH_SIZE], weights: &mut [f32; PATCH_SIZE], lr: f32){
    let y = oja_y(patch_x, weights);

    for j in 0..(PATCH_SIZE - 1) {
        let temp_w: f32 = patch_x[j] - y*weights[j];
        weights[j] = weights[j] + lr*y*temp_w;
    }
}

pub fn oja_y(patch_x: &[f32; PATCH_SIZE], weights: &[f32; PATCH_SIZE]) -> f32{
    let mut y = 0.0 as f32;
    for i in 0..PATCH_SIZE {
        y += weights[i] * patch_x[i];
    }
    return y;
}