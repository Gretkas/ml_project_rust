use rand::Rng;
use rust_ml::data::mnist::MnistData;
use rust_ml::model::oja::{oja_learning_rule};

fn main(){
    const NUM_WEIGHTS: usize = 25;
    const ITERS: usize = 50000;

    let mut rng = rand::thread_rng();
    let lr = 0.01;
    let mut weights: [f32; NUM_WEIGHTS] = rng.gen();

    let mnist_data = MnistData::new(1);

    for i in 0..ITERS {
        let training_randomized_patches = mnist_data.get_section_vector(i);
        oja_learning_rule(&training_randomized_patches[0], &mut weights, lr);
    }

    println!("{:?}", weights);
}