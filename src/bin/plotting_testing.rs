use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::{draw_piston_window};
use rust_ml::model::network::MtNetwork;
use rust_ml::plotting::cpu_plotting::CpuPlotter;

const FPS: u32 = 10;

fn main() {
    let mut cpu_plotter = CpuPlotter::new();

    let mut window: PistonWindow = WindowSettings::new("Oja monitoring", [1024, 768])
        .samples(4)
        .build()
        .unwrap();

    window.set_max_fps(FPS as u64);

    let network = MtNetwork::new(10, 30, 10000, 0.01);
    network.train_complete_iterations(50000);

    while let Some(_) = draw_piston_window(&mut window, |b| {

        let root = b.into_drawing_area();

        let (left, right) = root.split_horizontally((75).percent_width());

        left.fill(&WHITE).unwrap();
        right.fill(&WHITE).unwrap();

        cpu_plotter.draw(&left);

        Ok(())
    }) {}
}

