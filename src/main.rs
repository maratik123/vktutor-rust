use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::Version;

use vktutor_rust::main_window::MainWindow;

fn main() {
    let instance = Instance::new(None, Version::V1_2, &InstanceExtensions::none(), None).unwrap();
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    let main_window = MainWindow::default();
}
