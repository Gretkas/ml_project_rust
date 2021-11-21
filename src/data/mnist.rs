use mnist::{Mnist, MnistBuilder};
use ndarray::{Array3, s};
use rand::Rng;
use crate::utils::constants::PATCH_SIZE;

pub struct MnistData {
    section_size: usize,
    training_data: Array3<f32>,
}

impl MnistData {
    pub fn new(section_size: usize) -> MnistData {
        let Mnist {
            trn_img,
            ..
        } = MnistBuilder::new()
            .label_format_digit()
            .training_set_length(50_000)
            .finalize();

        let train_data = Array3::from_shape_vec((50_000, 28, 28), trn_img)
            .expect("Error converting images to Array3 struct")
            .map(|x| *x as f32 / 256.0);

        MnistData {
            section_size,
            training_data: train_data
        }
    }


    pub fn get_section_vector(&self, index: usize) -> Vec<[f32; PATCH_SIZE]>{
        let mut sections: Vec<[f32; PATCH_SIZE]> = Vec::new();
        for section_index in 0..self.section_size{
            sections.push(self.get_random_patch(index + section_index))
        }
        return sections;
    }

    pub fn get_sized_patch(&self, length: usize) -> Vec<[f32; PATCH_SIZE]>{
        let mut sections: Vec<[f32; PATCH_SIZE]> = Vec::new();
        for i in 0..length{
            sections.push(self.get_random_patch(i))
        }
        return sections;
    }

    fn get_random_patch(&self, index: usize) -> [f32; PATCH_SIZE]{
        let mut rng = rand::thread_rng();

        let mut training_randomized_patches: [f32; PATCH_SIZE] = rng.gen();

        let rand_training_column = rng.gen_range(0..22);
        let rand_training_row = rng.gen_range(0..22);

        for row in 0..5 {
            for column in 0..5 {
                training_randomized_patches[row * 5 + column] = *self.training_data.slice(s![index,rand_training_row + row, rand_training_column + column]).into_scalar();
            }
        }

        training_randomized_patches
    }



}