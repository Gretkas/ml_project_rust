//! Based on plotters-piston CPUstat example. https://github.com/plotters-rs/plotters-piston/blob/master/examples/cpustat.rs
//! Used to display live CPU stats in any given plotters window.
use plotters::prelude::*;
use plotters_piston::{PistonBackend};
use systemstat::platform::common::Platform;
use systemstat::{CPULoad, DelayedMeasurement, System};
use std::collections::vec_deque::VecDeque;
use plotters::coord::Shift;
use systemstat::platform::PlatformImpl;

const FPS: u32 = 10;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

pub struct CpuPlotter {
    load_measurement: Vec<DelayedMeasurement<Vec<CPULoad>>>,
    epoch: usize,
    data: Vec<VecDeque<f32>>,
    sys: PlatformImpl
}

impl CpuPlotter {
    pub fn new() -> CpuPlotter {
        let sys = System::new();
        let load_measurement: Vec<_> = (0..FPS).map(|_| sys.cpu_load().unwrap()).collect();
        let epoch = 0;
        let data: Vec<VecDeque<f32>> = vec![];

        CpuPlotter {
            load_measurement,
            epoch,
            data,
            sys
        }
    }

    pub fn draw(&mut self, window: &DrawingArea<PistonBackend, Shift>){
        let cpu_loads = self.load_measurement[self.epoch % FPS as usize].done().unwrap();

        if self.data.len() < cpu_loads.len() {
            for _ in self.data.len()..cpu_loads.len() {
                self.data.push(VecDeque::from(vec![0f32; N_DATA_POINTS + 1]));
            }
        }

        for (core_load, target) in cpu_loads.into_iter().zip(self.data.iter_mut()) {
            if target.len() == N_DATA_POINTS + 1 {
                target.pop_front();
            }
            target.push_back(1.0 - core_load.idle);
        }

        let mut cc = ChartBuilder::on(&window)
            .margin(10)
            .caption("Real Time CPU Usage", ("sans-serif", 30))
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..1f32).unwrap();


        cc.configure_mesh()
            .x_label_formatter(&|x| format!("{}", -(LENGTH as f32) + (*x as f32 / FPS as f32)))
            .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
            .x_labels(15)
            .y_labels(5)
            .x_desc("Seconds")
            .y_desc("% Busy")
            .axis_desc_style(("sans-serif", 15))
            .draw().unwrap();

        for (idx, data) in (0..).zip(self.data.iter()) {
            cc.draw_series(LineSeries::new(
                (0..).zip(data.iter()).map(|(a, b)| (a, *b)),
                &Palette99::pick(idx),
            )).unwrap()
                .label(format!("CPU {}", idx))
                .legend(move |(x, y)| {
                    Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
                });
        }

        cc.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();

        self.load_measurement[self.epoch % FPS as usize] = self.sys.cpu_load().unwrap();
        self.epoch += 1;
    }
}