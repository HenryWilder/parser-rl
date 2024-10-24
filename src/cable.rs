use crate::{Device, Plugin};

#[derive(Debug)]
struct Connection<'p> {
    device: &'p Device,
    plugin: &'p Plugin,
}

impl<'p> Connection<'p> {
    pub fn new(device: &'p Device, plugin: &'p Plugin) -> Self {
        Self { device, plugin }
    }
}

#[derive(Debug)]
pub struct Cable<'i, 'o> {
    input:  Connection<'i>,
    output: Connection<'o>,
}

impl<'i, 'o> Cable<'i, 'o> {
    pub fn new((in_device, in_plugin): (&'i Device, &'i Plugin), (out_device, out_plugin): (&'o Device, &'o Plugin)) -> Self {
        Self {
            input:  Connection::new( in_device,  in_plugin),
            output: Connection::new(out_device, out_plugin),
        }
    }
}
