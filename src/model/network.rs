use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Sender};
use rand::Rng;
use crate::data::mnist::MnistData;
use crate::model::oja::oja_learning_rule;
use crate::threading::thread_pool::ThreadPool;
use crate::utils::constants::PATCH_SIZE;

pub struct StNetwork{

}

pub struct MtNetwork{
    section_size: usize,
    threads: usize,
    neurons: usize,
    thread_pool: ThreadPool,
    lr: f32,
    mnist_data: MnistData,
    weights: Vec<[f32; PATCH_SIZE]>
}

impl MtNetwork {
    pub fn new(section_size: usize, threads: usize, neurons: usize, lr: f32) -> MtNetwork {
        let pool = ThreadPool::new(threads).unwrap();
        assert_eq!(neurons % section_size, 0);
        let mnist_data = MnistData::new(section_size);

        let mut rng = rand::thread_rng();
        let mut weights = Vec::new();
        for _ in 0..neurons{
            let weight: [f32; PATCH_SIZE] = rng.gen();
            weights.push(weight);
        }

        MtNetwork { section_size, threads , neurons, thread_pool: pool, lr, mnist_data, weights}
    }


    // Why even bother?
    pub fn train_iteration(&mut self, _epoch: usize) -> Vec<[f32; PATCH_SIZE]> {


        let (w_response, receiver) = mpsc::channel();
        let w_response = Arc::new(Mutex::new(w_response));

        for i in 0..self.threads {
            let thread_sender = w_response.clone();
            let mut local_weights: Vec<[f32; PATCH_SIZE]> = Vec::from(&self.weights[i*self.section_size..self.section_size + i*self.section_size]);
            let training_randomized_patches = self.mnist_data.get_section_vector(i);
            let lr_new = self.lr;

            self.thread_pool.execute(move || {
                for i in 0..local_weights.len() {
                    oja_learning_rule(&training_randomized_patches[i],&mut local_weights[i], lr_new);
                }
                thread_sender.lock().unwrap().send(local_weights).unwrap();
            });
        }

        let mut new_weights = Vec::new();

        for _ in 0..(self.neurons / self.section_size){
            new_weights.append(receiver.recv().unwrap().as_mut());
        }

        return new_weights;
    }

    pub fn train_complete_iterations(&self, _epochs: usize) {
        let (w_response, receiver) = mpsc::channel();
        let w_response :Arc<Mutex<Sender<Vec<[f32 ;PATCH_SIZE]>>>> = Arc::new(Mutex::new(w_response));
        let training_data_root = Arc::new(self.mnist_data.get_sized_patch(_epochs));

        self.thread_pool.execute(move || {
            let mut new_weights: Vec<[f32; PATCH_SIZE]> = Vec::new();
            for i in 0..(10000 / 10) {
                new_weights.append(receiver.recv().unwrap().as_mut());
                println!("Percentage done: {:?}", i as f32 / 1000.0);
            }
        });

        for _ in 0..(self.neurons / self.section_size){
            let w_response_copy = Arc::clone(&w_response);
            let local_lr = self.lr;
            let training_data = Arc::clone(&training_data_root);
            let sections = self.section_size;

            self.thread_pool.execute(move || {
                let mut local_weights = Vec::new();
                for _ in 0..sections {
                    let mut rng = rand::thread_rng();
                    let mut weights: [f32; PATCH_SIZE] = rng.gen();
                    for i in 0..((_epochs as i32) - 1) {
                        oja_learning_rule(&training_data[i as usize], &mut weights, local_lr);
                    }
                    local_weights.push(weights);
                }
                w_response_copy.lock().unwrap().send(local_weights).unwrap();
            });
        }


    }
}