use crate::{Clui, CluiColor, CluiRect, CluiWidget};
use slotmap::{new_key_type, HopSlotMap};
use std::cell::RefCell;

pub struct CluiLayer {
    windows: HopSlotMap<CluiWindowKey, CluiUiRect>,
}

new_key_type! { struct CluiWindowKey; }

impl CluiLayer {
    pub fn update_viewport(width: f32, height: f32) {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn new() -> CluiLayer {
        CluiLayer {
            windows: HopSlotMap::with_key(),
        }
    }
}

struct CluiUiRect {
    rect: CluiRect,
    screen_rect: CluiRect,
    background_color: CluiColor,
    z_index: i32,
    parent: usize,
}
