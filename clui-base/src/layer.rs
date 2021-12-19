#![allow(unused)]

use crate::{CluiColor, Point, Rect, Scalar, Size};
use slotmap::{new_key_type, HopSlotMap};

pub struct CluiLayer {
    windows: HopSlotMap<CluiWindowKey, CluiUiRect>,
    viewport: Size,
}

new_key_type! { pub struct CluiWindowKey; }

impl CluiLayer {
    pub fn update_viewport(&mut self, width: Scalar, height: Scalar) {
        self.viewport = Size { width, height }
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub(crate) fn new() -> CluiLayer {
        CluiLayer {
            windows: HopSlotMap::with_key(),
            viewport: Size {
                width: 800.0,
                height: 600.0,
            },
        }
    }

    pub fn add_default_window(&mut self) -> CluiWindowKey {
        self.windows.insert(CluiUiRect {
            rect: Rect {
                point: Point { x: 0.0, y: 0.0 },
                size: Size {
                    width: 0.0,
                    height: 0.0,
                },
            },
            //            screen_rect: CluiRect::default(),
            background_color: CluiColor::default(),
            z_index: 0,
            //          parent: 0,
        })
    }

    pub fn add_window(&mut self, window: CluiUiRect) -> CluiWindowKey {
        self.windows.insert(window)
    }

    pub fn remove_window(&mut self, key: CluiWindowKey) -> Option<CluiUiRect> {
        self.windows.remove(key)
    }

    pub fn get_mut_window_by_key(&mut self, key: CluiWindowKey) -> &mut CluiUiRect {
        self.windows.get_mut(key).unwrap()
    }

    pub fn get_window_by_key(&self, key: CluiWindowKey) -> &CluiUiRect {
        self.windows.get(key).unwrap()
    }
}

#[derive(Clone, Default, Copy)]
pub struct CluiUiRect {
    pub(crate) rect: Rect,
    //screen_rect: CluiRect,
    pub(crate) background_color: CluiColor,
    pub(crate) z_index: i32,
    //parent: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works2() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
