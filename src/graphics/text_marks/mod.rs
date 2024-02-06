//! Structs for constructing a group of text marks.

use iced::widget::canvas;
use iced::{Point, Rectangle, Size};
use iced_renderer::geometry::{self, Frame};

use std::cell::RefCell;

pub use crate::native::text_marks::*;
pub use crate::style::text_marks::*;

mod horizontal;
mod radial;
mod vertical;

pub use horizontal::*;
pub use radial::*;
pub use vertical::*;

#[derive(Default)]
struct CacheData {
    pub cache: geometry::Cache,

    pub bounds: Rectangle,
    pub text_marks_hash: u64,
    pub style: Appearance,
    pub placement: Placement,
    pub inverse: bool,

    pub center: Point,
    pub radius: f32,
    pub start_angle: f32,
    pub angle_span: f32,
}

impl std::fmt::Debug for CacheData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// A cache for text mark primitives.
#[derive(Debug, Default)]
pub struct Cache {
    data: RefCell<CacheData>,
}

impl Cache {
    /// Cache and retrieve linear text marks.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_cached_linear<F: FnOnce(&mut Frame), Theme>(
        &self,
        renderer: &mut iced::Renderer<Theme>,
        bounds: Rectangle,
        text_marks: &Group,
        style: Appearance,
        placement: Placement,
        inverse: bool,
        builder: F,
    ) {
        let mut data = self.data.borrow_mut();

        if !(data.bounds == bounds
            && data.text_marks_hash == text_marks.hashed()
            && data.style == style
            && data.placement == placement
            && data.inverse == inverse)
        {
            data.bounds = bounds;
            data.text_marks_hash = text_marks.hashed();
            data.style = style;
            data.placement = placement;
            data.inverse = inverse;

            data.cache.clear();
        }

        canvas::Renderer::draw(
            renderer,
            vec![data.cache.draw(renderer, bounds.size(), builder)],
        );
    }

    /// Cache and retrieve radial text marks.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_cached_radial<F: FnOnce(&mut Frame), Theme>(
        &self,
        renderer: &mut iced::Renderer<Theme>,
        center: Point,
        radius: f32,
        start_angle: f32,
        angle_span: f32,
        text_marks: &Group,
        style: Appearance,
        inverse: bool,
        builder: F,
    ) {
        let mut data = self.data.borrow_mut();

        if !(data.center == center
            && data.radius == radius
            && data.start_angle == start_angle
            && data.angle_span == angle_span
            && data.text_marks_hash == text_marks.hashed()
            && data.style == style
            && data.inverse == inverse)
        {
            data.center = center;
            data.radius = radius;
            data.start_angle = start_angle;
            data.angle_span = angle_span;
            data.text_marks_hash = text_marks.hashed();
            data.style = style;
            data.inverse = inverse;

            data.cache.clear();
        }

        let diameter = 2.0f32 * data.radius;
        canvas::Renderer::draw(
            renderer,
            vec![data.cache.draw(
                renderer,
                Size::new(diameter, diameter),
                builder,
            )],
        );
    }
}
