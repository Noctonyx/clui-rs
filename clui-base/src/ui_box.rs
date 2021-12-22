use crate::{CluiColor, Rect, Scalar, Size};

pub enum MarginDef {
    Auto,
    Value(Scalar),
}

pub struct Margin {
    pub left: MarginDef,
    pub right: MarginDef,
    pub top: MarginDef,
    pub bottom: MarginDef,
}

pub struct Padding {
    pub left: Scalar,
    pub right: Scalar,
    pub top: Scalar,
    pub bottom: Scalar,
}

pub struct BorderMetrics {
    pub size: Scalar,
    pub color: CluiColor,
}

pub enum BorderDef {
    None,
    Solid(BorderMetrics),
}

pub struct Border {
    pub left: BorderDef,
    pub right: BorderDef,
    pub top: BorderDef,
    pub bottom: BorderDef,
}

pub struct BoxModel {
    pub margin: Margin,
    pub padding: Padding,
    pub content: Size,
    pub background_color: CluiColor,
    pub z_order: Scalar,
    //pub rect: Rect,
    //dirty: bool,
}
