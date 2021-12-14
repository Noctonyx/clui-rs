use crate::layer::CluiLayer;
use slotmap::{new_key_type, DefaultKey, SlotMap};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Copy, Clone)]
struct CluiExtent {
    width: f32,
    height: f32,
}

#[derive(Copy, Clone)]
struct CluiOffset {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
struct CluiRect {
    offset: CluiOffset,
    extent: CluiExtent,
}

struct Clui<'a> {
    get_elapsed_time_callback: Option<Box<dyn Fn() -> f64 + 'a>>,
    log_message_handler: Option<Box<dyn FnMut(&str) -> bool + 'a>>,
    file_read_handler: Option<Box<dyn FnMut(&str) -> Vec<u8> + 'a>>,

    layers: SlotMap<CluiLayerKey, CluiLayer>,
}

new_key_type! { struct CluiLayerKey; }

impl<'a> Clui<'a> {
    pub fn set_elapsed_time_handler<F>(&mut self, func: F)
    where
        F: 'a + Fn() -> f64,
    {
        self.get_elapsed_time_callback = Some(Box::new(func));
    }

    pub fn set_log_message_handler<F>(&mut self, func: F)
    where
        F: FnMut(&str) -> bool + 'a,
    {
        self.log_message_handler = Some(Box::new(func));
    }

    pub fn set_file_read_handler<F>(&mut self, func: F)
    where
        F: FnMut(&str) -> Vec<u8> + 'a,
    {
        self.file_read_handler = Some(Box::new(func));
    }

    pub fn get_render_data() -> CluiDrawList {
        todo!()
    }

    pub fn create_layer(&mut self) -> CluiLayerKey {
        self.layers.insert(CluiLayer::new())
    }
}

mod layer;

struct CluiWidget {}

#[derive(Copy, Clone)]
struct CluiColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Copy, Clone)]
enum CluiPositioning {
    Relative,
    Absolute,
}

#[derive(Copy, Clone)]
struct CluiVertex {
    position: [f32; 2],
    color: [f32; 4],
}

#[derive(Clone)]
struct CluiDrawList {
    draw_sets: Vec<CluiDrawSet>,
}

#[derive(Clone)]
struct CluiDrawSet {
    viewport: CluiRect,
    scissor: CluiRect,
    draws: Vec<CluiDraw>,
    vertices: Vec<CluiVertex>,
    indices: Vec<u32>,
}

#[derive(Clone)]
struct CluiDraw {
    index_offset: u32,
    index_count: u32,
    vertex_offset: u32,
}
