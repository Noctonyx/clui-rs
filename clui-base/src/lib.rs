#![allow(unused)]

pub use self::rect::Rect;

pub mod rect;

use crate::layer::CluiLayer;
use slotmap::{new_key_type, SlotMap};

pub type Scalar = f32;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Point {
    x: Scalar,
    y: Scalar,
}

impl Point {
    pub fn from_xy(x: Scalar, y: Scalar) -> Point {
        Point { x, y }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Size {
    width: Scalar,
    height: Scalar,
}

struct Clui<'a> {
    get_elapsed_time_callback: Option<Box<dyn Fn() -> f64 + 'a>>,
    log_message_handler: Option<Box<dyn FnMut(&str) -> bool + 'a>>,
    file_read_handler: Option<Box<dyn FnMut(&str) -> Vec<u8> + 'a>>,

    layers: SlotMap<CluiLayerKey, CluiLayer>,
}

new_key_type! { pub struct CluiLayerKey; }

impl<'a> Clui<'a> {
    pub fn new() -> Clui<'a> {
        Clui {
            get_elapsed_time_callback: None,
            log_message_handler: None,
            file_read_handler: None,
            layers: SlotMap::with_key(),
        }
    }

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

    pub fn get_layer_by_key(&mut self, key: CluiLayerKey) -> Option<&mut CluiLayer> {
        self.layers.get_mut(key)
    }
}

mod layer;

struct CluiWidget {}

#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Debug)]
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
    viewport: Size,
    scissor: Rect,
    draws: Vec<DrawInstruction>,
    vertices: Vec<CluiVertex>,
    indices: Vec<u32>,
}

#[derive(Clone)]
struct DrawInstruction {
    index_offset: u32,
    index_count: u32,
    vertex_offset: u32,
}

#[cfg(test)]
mod tests {
    use crate::layer::CluiUiRect;
    use crate::{Clui, CluiColor, Rect};

    #[test]
    fn layer_basics() {
        let mut clui = crate::Clui::new();
        {
            let layer_key = clui.create_layer();
            let layer = clui.get_layer_by_key(layer_key).unwrap();

            layer.update_viewport(800.0, 600.0);

            let rect = Rect::from_values(10.0, 11.0, 100.0, 110.0);
            let wk = layer.add_window(CluiUiRect {
                rect,
                background_color: Default::default(),
                z_index: 0,
            });
            let w = layer.get_window_by_key(wk);

            assert_eq!(w.rect, Rect::from_values(10.0, 11.0, 100.0, 110.0));
            assert_eq!(
                w.background_color,
                CluiColor {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0
                }
            );

            layer.remove_window(wk);
        }
    }
}
