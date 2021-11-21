use rust_ml::model::network::MtNetwork;

fn main(){
    let network = MtNetwork::new(10, 30, 1000, 0.01);
   network.train_complete_iterations(50000);
}