use rust_ml::model::network::MtNetwork;

fn main() {
    for rounds in 0..3 {
        println!("starting round: {}", rounds);
        for threads in 1..40 {
            println!("starting batch with: {} threads", threads);
            let network = MtNetwork::new(100, threads + 1, 10000, 0.01);
            network.train_complete_iterations(50000);
        }
    }
}