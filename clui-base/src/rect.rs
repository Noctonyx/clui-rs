use crate::{Point, Scalar, Size};
use std::cmp::min;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Rect {
    pub point: Point,
    pub size: Size,
}

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Rect {
    pub fn from_values(x: Scalar, y: Scalar, width: Scalar, height: Scalar) -> Self {
        Self {
            point: Point { x, y },
            size: Size { width, height },
        }
    }

    pub fn from_pos_and_size(point: Point, size: Size) -> Self {
        Self { point, size }
    }

    pub fn from_corners(a: Point, b: Point) -> Self {
        Self {
            point: Point {
                x: Scalar::min(a.x, b.x),
                y: Scalar::min(a.y, b.y),
            },
            size: Size {
                width: Scalar::abs(a.x - b.x),
                height: Scalar::abs(a.y - b.y),
            },
        }
    }

    pub fn from_rects(r1: Self, r2: Self) {}

    pub fn move_to(&self, point: Point) -> Self {
        Rect { point, ..*self }
    }

    pub fn center(self) -> Point {
        Point {
            x: self.point.x + self.size.width / 2.0,
            y: self.point.y + self.size.height / 2.0,
        }
    }

    pub fn contains_point(self, p: Point) -> bool {
        let br = self.bottom_right();

        if p.x < self.point.x {
            return false;
        }
        if p.x > br.x {
            return false;
        }
        if p.y < self.point.y {
            return false;
        }
        if p.y > br.y {
            return false;
        }

        true
    }

    pub fn bottom_right(self) -> Point {
        Point {
            x: self.point.x + self.size.width,
            y: self.point.y + self.size.height,
        }
    }

    pub fn top(&self) -> Scalar {
        self.point.y
    }

    pub fn bottom(&self) -> Scalar {
        self.point.y + self.size.height
    }

    pub fn left(&self) -> Scalar {
        self.point.x
    }

    pub fn right(&self) -> Scalar {
        self.point.x + self.size.width
    }

    pub fn height(&self) -> Scalar {
        self.size.height
    }

    pub fn width(&self) -> Scalar {
        self.size.width
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Rect, Size};

    #[test]
    fn has_rect_create() {
        let r = Rect::from_values(1.0, 2.0, 3.0, 4.0);
        assert_eq!(r.point, Point { x: 1.0, y: 2.0 });
        assert_eq!(
            r.size,
            Size {
                width: 3.0,
                height: 4.0
            }
        );
    }

    #[test]
    fn edge_positions() {
        let r = Rect::from_corners(Point { x: 1.0, y: 2.0 }, Point { x: 11.0, y: 20.0 });
        assert_eq!(r.width(), 10.0);
        assert_eq!(r.height(), 18.0);
        assert_eq!(r.top(), 2.0);
        assert_eq!(r.bottom(), 20.0);
        assert_eq!(r.left(), 1.0);
        assert_eq!(r.right(), 11.0);
        assert_eq!(r.bottom_right(), Point { x: 11.0, y: 20.0 });
    }
}
