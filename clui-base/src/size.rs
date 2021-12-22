use crate::Scalar;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Size {
    pub width: Scalar,
    pub height: Scalar,
}

impl Size {
    pub fn from_wh(w: Scalar, h: Scalar) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn grow(&self, w: Scalar, h: Scalar) -> Self {
        Self::from_wh(self.width + w, self.height + h)
    }

    /// Return a new Size shrunk by the specified width and height
    ///
    pub fn shrink(&self, width: Scalar, height: Scalar) -> Self {
        Self::from_wh(self.width - width, self.height - height)
    }

    pub fn with_height(&self, h: Scalar) -> Self {
        Self { height: h, ..*self }
    }

    pub fn with_width(&self, w: Scalar) -> Self {
        Self { width: w, ..*self }
    }
}

#[cfg(test)]
mod tests {
    use crate::Size;

    #[test]
    fn size_with_height() {
        let size1 = Size::from_wh(10.0, 20.0);
        let size2 = size1.with_height(35.0);
        assert_eq!(size1.width, size2.width);
        assert_eq!(size2.height, 35.0);
    }

    #[test]
    fn size_with_width() {
        let size1 = Size::from_wh(10.0, 20.0);
        let size2 = size1.with_width(35.0);
        assert_eq!(size1.height, size2.height);
        assert_eq!(size2.width, 35.0);
    }

    #[test]
    fn size_grow() {
        let size1 = Size::from_wh(10.0, 20.0);
        let size2 = size1.grow(1.0, 2.0);
        assert_eq!(size2.height, 22.0);
        assert_eq!(size2.width, 11.0);
    }

    #[test]
    fn size_shrink() {
        let size1 = Size::from_wh(10.0, 20.0);
        let size2 = size1.shrink(1.0, 2.0);
        assert_eq!(size2.height, 18.0);
        assert_eq!(size2.width, 9.0);
    }
}
