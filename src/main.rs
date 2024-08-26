mod data;
mod model;
mod training;

use burn::backend::{Autodiff, Wgpu};

fn main() {
    let device = burn::backend::wgpu::WgpuDevice::default();

    training::run::<Autodiff<Wgpu>>(device);
}
