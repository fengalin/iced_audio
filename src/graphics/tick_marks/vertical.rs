//! `iced` renderer for tick marks

use super::Cache;
use crate::core::Normal;
use crate::native::tick_marks;
use crate::style::tick_marks::{Appearance, Placement, Shape};

use iced::widget::canvas::{Path, Stroke};
use iced::{Color, Point, Rectangle};
use iced_renderer::geometry::Frame;

#[allow(clippy::too_many_arguments)]
fn draw_vertical_lines(
    frame: &mut Frame,
    tick_marks: &[Normal],
    bounds_y: f32,
    bounds_height: f32,
    x: f32,
    width: f32,
    length: f32,
    color: Color,
    inverse: bool,
) {
    let start_y = bounds_y - (width / 2.0);

    if inverse {
        for tick_mark in tick_marks {
            let y = start_y + tick_mark.scale(bounds_height);

            frame.stroke(
                &Path::line(Point { x, y }, Point { x, y: y + width }),
                Stroke {
                    style: color.into(),
                    width: length,
                    ..Default::default()
                },
            );
        }
    } else {
        for tick_mark in tick_marks {
            let y = start_y + tick_mark.scale_inv(bounds_height);

            frame.stroke(
                &Path::line(Point { x, y }, Point { x, y: y + width }),
                Stroke {
                    style: color.into(),
                    width: length,
                    ..Default::default()
                },
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_vertical_circles(
    frame: &mut Frame,
    tick_marks: &[Normal],
    bounds_y: f32,
    bounds_height: f32,
    x: f32,
    diameter: f32,
    color: Color,
    inverse: bool,
) {
    let radius = diameter / 2.0;
    let start_y = bounds_y - radius;

    if inverse {
        for tick_mark in tick_marks {
            let y = start_y + tick_mark.scale(bounds_height);

            frame.stroke(
                &Path::circle(Point { x, y }, radius),
                Stroke {
                    style: color.into(),
                    ..Default::default()
                },
            );
        }
    } else {
        for tick_mark in tick_marks {
            let y = start_y + tick_mark.scale_inv(bounds_height);

            frame.stroke(
                &Path::circle(Point { x, y }, radius),
                Stroke {
                    style: color.into(),
                    ..Default::default()
                },
            );
        }
    }
}

#[inline]
fn draw_vertical_left_aligned_tier(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                draw_vertical_lines(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    *width,
                    *length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                draw_vertical_circles(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    *diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_left_aligned(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Appearance,
    inverse: bool,
) {
    draw_vertical_left_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_vertical_left_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_vertical_left_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_vertical_right_aligned_tier(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                draw_vertical_lines(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x - (*length),
                    *width,
                    *length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                draw_vertical_circles(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x - (*diameter),
                    *diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_right_aligned(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Appearance,
    inverse: bool,
) {
    draw_vertical_right_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_vertical_right_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_vertical_right_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_vertical_center_aligned_tier(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    fill_length: bool,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                let (x, length) = if fill_length {
                    (bounds.x + (*length), bounds.width - ((*length) * 2.0))
                } else {
                    (x - (*length / 2.0), *length)
                };

                draw_vertical_lines(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    *width,
                    length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                let (x, diameter) = if fill_length {
                    (bounds.x + (*diameter), bounds.width - ((*diameter) * 2.0))
                } else {
                    (x - (*diameter / 2.0), *diameter)
                };

                draw_vertical_circles(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_center_aligned(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Appearance,
    fill_length: bool,
    inverse: bool,
) {
    draw_vertical_center_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        inverse,
    );
    draw_vertical_center_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        inverse,
    );
    draw_vertical_center_aligned_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        inverse,
    );
}

#[inline]
#[allow(clippy::too_many_arguments)]
fn draw_vertical_center_aligned_split_tier(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => (),
            Shape::Line {
                length,
                width,
                color,
            } => {
                let (left_x, length) = if fill_length {
                    let length = *length + ((bounds.width + gap) / 2.0);
                    (x - length - (gap / 2.0), length)
                } else {
                    (x - *length - (gap / 2.0), *length)
                };

                let right_x = x + (gap / 2.0);

                draw_vertical_lines(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    left_x,
                    *width,
                    length,
                    *color,
                    inverse,
                );
                draw_vertical_lines(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    right_x,
                    *width,
                    length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                let (left_x, diameter) = if fill_length {
                    (
                        bounds.x - *diameter,
                        *diameter + ((bounds.width + gap) / 2.0),
                    )
                } else {
                    (x - *diameter - (gap / 2.0), *diameter)
                };

                let right_x = x + (gap / 2.0);

                draw_vertical_circles(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    left_x,
                    diameter,
                    *color,
                    inverse,
                );
                draw_vertical_circles(
                    frame,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    right_x,
                    diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_vertical_center_aligned_split(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Appearance,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    draw_vertical_center_aligned_split_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        gap,
        inverse,
    );
    draw_vertical_center_aligned_split_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        gap,
        inverse,
    );
    draw_vertical_center_aligned_split_tier(
        frame,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        gap,
        inverse,
    );
}

/// Draws tick marks on a vertical axis.
///
/// * bounds - The bounds of the widget to place the tick marks in/outside of.
/// * tick_marks - The group of tick marks.
/// * style - The tick marks style.
/// * placement - The placement of the tick marks relative to the bounds.
/// * inverse - Whether to inverse the positions of the tick marks (true) or
/// not (false).
pub fn draw_vertical_tick_marks<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    bounds: &Rectangle,
    tick_marks: &tick_marks::Group,
    style: &Appearance,
    placement: &Placement,
    inverse: bool,
    cache: &Cache,
) {
    cache.draw_cached_linear(
        renderer,
        *bounds,
        tick_marks,
        *style,
        *placement,
        inverse,
        |frame| match placement {
            Placement::BothSides { offset, inside } => {
                let bounds = offset.offset_rect(bounds);

                if *inside {
                    draw_vertical_left_aligned(
                        frame, &bounds, bounds.x, tick_marks, style, inverse,
                    );
                    draw_vertical_right_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        tick_marks,
                        style,
                        inverse,
                    );
                } else {
                    draw_vertical_right_aligned(
                        frame, &bounds, bounds.x, tick_marks, style, inverse,
                    );
                    draw_vertical_left_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        tick_marks,
                        style,
                        inverse,
                    );
                }
            }
            Placement::LeftOrTop { offset, inside } => {
                let bounds = offset.offset_rect(bounds);

                if *inside {
                    draw_vertical_left_aligned(
                        frame, &bounds, bounds.x, tick_marks, style, inverse,
                    );
                } else {
                    draw_vertical_right_aligned(
                        frame, &bounds, bounds.x, tick_marks, style, inverse,
                    );
                }
            }
            Placement::RightOrBottom { offset, inside } => {
                let bounds = offset.offset_rect(bounds);

                if *inside {
                    draw_vertical_right_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        tick_marks,
                        style,
                        inverse,
                    );
                } else {
                    draw_vertical_left_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        tick_marks,
                        style,
                        inverse,
                    );
                }
            }
            Placement::Center {
                offset,
                fill_length,
            } => {
                let bounds = offset.offset_rect(bounds);

                draw_vertical_center_aligned(
                    frame,
                    &bounds,
                    bounds.center_x(),
                    tick_marks,
                    style,
                    *fill_length,
                    inverse,
                );
            }
            Placement::CenterSplit {
                offset,
                fill_length,
                gap,
            } => {
                let bounds = offset.offset_rect(bounds);

                draw_vertical_center_aligned_split(
                    frame,
                    &bounds,
                    bounds.center_x(),
                    tick_marks,
                    style,
                    *fill_length,
                    *gap,
                    inverse,
                );
            }
        },
    );
}
