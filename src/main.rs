use vulkano::device::physical::{PhysicalDevice, QueueFamily};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::Version;

use vktutor_rust::main_window::MainWindow;

fn main() {
    let instance = Instance::new(None, Version::V1_2, &InstanceExtensions::none(), None).unwrap();
    let physical: PhysicalDevice = PhysicalDevice::enumerate(&instance).next().unwrap();
    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }
    let main_window = MainWindow::default();
}
