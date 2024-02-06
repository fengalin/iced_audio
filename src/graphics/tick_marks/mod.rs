//! Structs for constructing a group of tick marks.

use iced::widget::canvas;
use iced::{Point, Rectangle, Size};
use iced_renderer::geometry::{self, Frame};

use std::cell::RefCell;

pub use crate::native::tick_marks::*;
pub use crate::style::tick_marks::*;

mod horizontal;
mod radial;
mod vertical;

pub use horizontal::*;
pub use radial::*;
pub use vertical::*;

struct CacheData {
    // FIXME fengalin: the fieds probably don't need to be pub
    pub cache: geometry::Cache,

    pub bounds: Rectangle,
    pub tick_marks_hash: u64,
    pub style: Appearance,
    pub placement: Placement,
    pub inverse: bool,

    pub center: Point,
    pub radius: f32,
    pub start_angle: f32,
    pub angle_span: f32,
    pub inside: bool,
    size: Size,
}

impl Default for CacheData {
    fn default() -> Self {
        Self {
            cache: geometry::Cache::default(),

            bounds: Rectangle::default(),
            tick_marks_hash: 0,
            style: Appearance::default(),
            placement: Placement::default(),
            inverse: false,

            center: Point::default(),
            radius: 0.0,
            start_angle: 0.0,
            angle_span: 0.0,
            inside: false,
            size: Size::ZERO,
        }
    }
}

impl std::fmt::Debug for CacheData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// A cache for tick mark primitives.
#[derive(Debug, Default)]
pub struct Cache {
    data: RefCell<CacheData>,
}

impl Cache {
    /// Cache and retrieve linear tick marks.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_cached_linear<F: FnOnce(&mut Frame), Theme>(
        &self,
        renderer: &mut iced::Renderer<Theme>,
        bounds: Rectangle,
        tick_marks: &Group,
        style: Appearance,
        placement: Placement,
        inverse: bool,
        builder: F,
    ) {
        let mut data = self.data.borrow_mut();

        if !(data.bounds == bounds
            && data.tick_marks_hash == tick_marks.hashed()
            && data.style == style
            && data.placement == placement
            && data.inverse == inverse)
        {
            data.bounds = bounds;
            data.tick_marks_hash = tick_marks.hashed();
            data.style = style;
            data.placement = placement;
            data.inverse = inverse;
            data.size = bounds.size();

            data.cache.clear();
        }

        canvas::Renderer::draw(
            renderer,
            vec![data.cache.draw(renderer, data.size, builder)],
        );
    }

    /// Cache and retrieve radial tick marks.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_cached_radial<F: FnOnce(&mut Frame), Theme>(
        &self,
        renderer: &mut iced::Renderer<Theme>,
        center: Point,
        radius: f32,
        start_angle: f32,
        angle_span: f32,
        inside: bool,
        tick_marks: &Group,
        style: Appearance,
        inverse: bool,
        builder: F,
    ) {
        let mut data = self.data.borrow_mut();

        if !(data.center == center
            && data.radius == radius
            && data.start_angle == start_angle
            && data.angle_span == angle_span
            && data.inside == inside
            && data.tick_marks_hash == tick_marks.hashed()
            && data.style == style
            && data.inverse == inverse)
        {
            data.center = center;
            data.radius = radius;
            data.start_angle = start_angle;
            data.angle_span = angle_span;
            data.inside = inside;
            data.tick_marks_hash = tick_marks.hashed();
            data.style = style;
            data.inverse = inverse;

            let frame_radius = if inside {
                radius
            } else {
                radius + max_length(&style)
            };

            let frame_size = frame_radius * 2.0;

            data.size = Size::new(frame_size, frame_size);

            data.cache.clear();
        }

        canvas::Renderer::draw(
            renderer,
            vec![data.cache.draw(renderer, data.size, builder)],
        );
    }
}

// FIXME fengalin duplicate from super::radial.rs
fn max_length(style: &Appearance) -> f32 {
    let length_1 = match style.tier_1 {
        Shape::None => 0.0,
        Shape::Line { length, .. } => length,
        Shape::Circle { diameter, .. } => diameter,
    };

    let length_2 = match style.tier_1 {
        Shape::None => 0.0,
        Shape::Line { length, .. } => length,
        Shape::Circle { diameter, .. } => diameter,
    };

    let length_3 = match style.tier_1 {
        Shape::None => 0.0,
        Shape::Line { length, .. } => length,
        Shape::Circle { diameter, .. } => diameter,
    };

    length_1.max(length_2).max(length_3)
}
