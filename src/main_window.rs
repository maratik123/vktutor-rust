use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::instance::Instance;

pub struct MainWindow<'a> {
    instance: &'a Arc<Instance>,
    physical_device: PhysicalDevice<'a>,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl<'a> MainWindow<'a> {
    pub fn new(instance: &'a Arc<Instance>) -> MainWindow<'a> {
        let physical_device: PhysicalDevice = PhysicalDevice::enumerate(&instance).next().unwrap();
        let mut queue_families = physical_device.queue_families();
        let queue_family = queue_families.find(|&q| q.supports_graphics()).unwrap();
        for family in physical_device.queue_families() {
            println!(
                "Found a queue family with {:?} queue(s)",
                family.queues_count()
            );
        }
        let (device, mut queues) = Device::new(
            physical_device,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();
        let queue = queues.next().unwrap();
        MainWindow {
            instance,
            physical_device,
            device: device,
            queue: queue,
        }
    }
}
