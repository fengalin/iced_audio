use super::Cache;
use crate::native::text_marks;
use crate::style::text_marks::{Align, Appearance, Placement};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::Text;
use iced::widget::text::LineHeight;
use iced::{Point, Rectangle};
use iced_renderer::geometry::Frame;

fn draw_aligned(
    frame: &mut Frame,
    bounds: &Rectangle,
    x: f32,
    text_marks: &text_marks::Group,
    style: &Appearance,
    inverse: bool,
    align: Horizontal,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_height = f32::from(style.bounds_height);

    if inverse {
        for text_mark in &text_marks.group {
            frame.fill_text(Text {
                content: text_mark.1.clone(),
                size: text_size,
                position: Point {
                    x,
                    y: (bounds.y + (text_mark.0.scale(bounds.height))).round(),
                },
                line_height: LineHeight::Relative(text_bounds_height),
                color,
                font,
                horizontal_alignment: align,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            });
        }
    } else {
        for text_mark in &text_marks.group {
            frame.fill_text(Text {
                content: text_mark.1.clone(),
                size: text_size,
                position: Point {
                    x,
                    y: (bounds.y + (text_mark.0.scale_inv(bounds.height)))
                        .round(),
                },
                line_height: LineHeight::Relative(text_bounds_height),
                color,
                font,
                horizontal_alignment: align,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            });
        }
    }
}

/// Draws text marks on a vertical axis.
///
/// * bounds - The bounds of the widget to place the text marks in/outside of.
/// * text_marks - The group of text marks.
/// * style - The text marks style.
/// * placement - The placement of the text marks relative to the bounds.
/// * inverse - Whether to inverse the positions of the text marks (true) or
/// not (false).
pub fn draw_vertical_text_marks<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    bounds: &Rectangle,
    text_marks: &text_marks::Group,
    style: &Appearance,
    placement: &Placement,
    inverse: bool,
    cache: &Cache,
) {
    cache.draw_cached_linear(
        renderer,
        *bounds,
        text_marks,
        *style,
        *placement,
        inverse,
        |frame| match placement {
            Placement::BothSides { inside, offset } => {
                let bounds = offset.offset_rect(bounds);

                if *inside {
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Left,
                    );
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Right,
                    );
                } else {
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Right,
                    );
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Left,
                    );
                }
            }
            Placement::LeftOrTop { inside, offset } => {
                let bounds = offset.offset_rect(bounds);

                if *inside {
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Left,
                    );
                } else {
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Right,
                    );
                }
            }
            Placement::RightOrBottom { inside, offset } => {
                let bounds = offset.offset_rect(bounds);

                if *inside {
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Right,
                    );
                } else {
                    draw_aligned(
                        frame,
                        &bounds,
                        bounds.x + bounds.width,
                        text_marks,
                        style,
                        inverse,
                        Horizontal::Left,
                    );
                }
            }
            Placement::Center { align, offset } => {
                let bounds = offset.offset_rect(bounds);

                match align {
                    Align::Start => {
                        draw_aligned(
                            frame,
                            &bounds,
                            bounds.center_x(),
                            text_marks,
                            style,
                            inverse,
                            Horizontal::Left,
                        );
                    }
                    Align::End => {
                        draw_aligned(
                            frame,
                            &bounds,
                            bounds.center_x(),
                            text_marks,
                            style,
                            inverse,
                            Horizontal::Right,
                        );
                    }
                    Align::Center => {
                        draw_aligned(
                            frame,
                            &bounds,
                            bounds.center_x(),
                            text_marks,
                            style,
                            inverse,
                            Horizontal::Center,
                        );
                    }
                }
            }
        },
    );
}
