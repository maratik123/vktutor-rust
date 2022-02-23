use vulkano::device::physical::{PhysicalDevice, QueueFamily};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::Version;

use vktutor_rust::main_window::MainWindow;

fn main() {
    let instance = Instance::new(None, Version::V1_2, &InstanceExtensions::none(), None).unwrap();
    let physical: PhysicalDevice = PhysicalDevice::enumerate(&instance).next().unwrap();
    let mut queue_families = physical.queue_families();
    let queue_family = queue_families.find(|&q| q.supports_graphics()).unwrap();
    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }
    let (device, mut queues) = Device::new(
        physical,
        &Features::none(),
        &DeviceExtensions::none(),
        [(queue_family, 0.5)].iter().cloned(),
    )
    .unwrap();
    let queue = queues.next().unwrap();
    let main_window = MainWindow::default();
}
