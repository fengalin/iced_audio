//! Display an interactive vertical slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::{ModulationRange, Normal};
use crate::graphics::{text_marks, tick_marks};
use crate::native::v_slider;

use iced::advanced::renderer::Quad;
use iced::{Background, Color, Point, Rectangle};

pub use crate::style::v_slider::{
    Appearance, ClassicAppearance, ClassicHandle, ClassicRail,
    ModRangeAppearance, ModRangePlacement, RectAppearance,
    RectBipolarAppearance, StyleSheet, TextMarksAppearance, TextureAppearance,
    TickMarksAppearance,
};

struct ValueMarkers<'a> {
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
    tick_marks_style: Option<TickMarksAppearance>,
    text_marks_style: Option<TextMarksAppearance>,
    mod_range_style_1: Option<ModRangeAppearance>,
    mod_range_style_2: Option<ModRangeAppearance>,
}

/// A vertical slider GUI widget that controls a [`Param`]
///
/// a [`VSlider`] will try to fill the vertical space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`VSlider`]: struct.VSlider.html
pub type VSlider<'a, Message, Theme> =
    v_slider::VSlider<'a, Message, iced::Renderer<Theme>>;

impl<Theme> v_slider::Renderer for iced::Renderer<Theme>
where
    Self::Theme: StyleSheet,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        mod_range_1: Option<&ModulationRange>,
        mod_range_2: Option<&ModulationRange>,
        tick_marks: Option<&tick_marks::Group>,
        text_marks: Option<&text_marks::Group>,
        style_sheet: &dyn StyleSheet<
            Style = <Self::Theme as StyleSheet>::Style,
        >,
        style: &<Self::Theme as StyleSheet>::Style,
        tick_marks_cache: &tick_marks::Cache,
        text_marks_cache: &text_marks::Cache,
    ) {
        let is_mouse_over = bounds.contains(cursor_position);

        let appearance = if is_dragging {
            style_sheet.dragging(style)
        } else if is_mouse_over {
            style_sheet.hovered(style)
        } else {
            style_sheet.active(style)
        };

        let bounds = Rectangle {
            x: bounds.x.round(),
            y: bounds.y.round(),
            width: bounds.width.round(),
            height: bounds.height.round(),
        };

        let value_markers = ValueMarkers {
            tick_marks,
            text_marks,
            mod_range_1,
            mod_range_2,
            tick_marks_style: style_sheet.tick_marks_appearance(style),
            text_marks_style: style_sheet.text_marks_appearance(style),
            mod_range_style_1: style_sheet.mod_range_appearance(style),
            mod_range_style_2: style_sheet.mod_range_appearance_2(style),
        };

        match appearance {
            Appearance::Texture(style) => draw_texture_style(
                self,
                normal,
                &bounds,
                style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::Classic(style) => draw_classic_style(
                self,
                normal,
                &bounds,
                &style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::Rect(style) => draw_rect_style(
                self,
                normal,
                &bounds,
                &style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
            Appearance::RectBipolar(style) => draw_rect_bipolar_style(
                self,
                normal,
                &bounds,
                &style,
                &value_markers,
                tick_marks_cache,
                text_marks_cache,
            ),
        }
    }
}

fn draw_value_markers<Theme>(
    renderer: &iced::Renderer<Theme>,
    mark_bounds: &Rectangle,
    mod_bounds: &Rectangle,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    (
        draw_tick_marks(
            renderer,
            mark_bounds,
            value_markers.tick_marks,
            &value_markers.tick_marks_style,
            tick_marks_cache,
        ),
        draw_text_marks(
            renderer,
            mark_bounds,
            value_markers.text_marks,
            &value_markers.text_marks_style,
            text_marks_cache,
        ),
        draw_mod_range(
            renderer,
            mod_bounds,
            value_markers.mod_range_1,
            &value_markers.mod_range_style_1,
        ),
        draw_mod_range(
            renderer,
            mod_bounds,
            value_markers.mod_range_2,
            &value_markers.mod_range_style_2,
        ),
    )
}

fn draw_tick_marks<Theme>(
    renderer: &iced::Renderer<Theme>,
    bounds: &Rectangle,
    tick_marks: Option<&tick_marks::Group>,
    tick_marks_style: &Option<TickMarksAppearance>,
    tick_marks_cache: &tick_marks::Cache,
) {
    let Some(tick_marks) = tick_marks else { return };
    let Some(style) = tick_marks_style else {
        return;
    };

    tick_marks::draw_vertical_tick_marks(
        renderer,
        bounds,
        tick_marks,
        &style.style,
        &style.placement,
        false,
        tick_marks_cache,
    )
}

fn draw_text_marks<Theme>(
    renderer: &iced::Renderer<Theme>,
    bounds: &Rectangle,
    text_marks: Option<&text_marks::Group>,
    text_marks_style: &Option<TextMarksAppearance>,
    text_marks_cache: &text_marks::Cache,
) {
    let Some(text_marks) = text_marks else { return };
    let Some(style) = text_marks_style else {
        return;
    };

    text_marks::draw_vertical_text_marks(
        renderer,
        bounds,
        text_marks,
        &style.style,
        &style.placement,
        false,
        text_marks_cache,
    )
}

fn draw_mod_range<Theme>(
    renderer: &iced::Renderer<Theme>,
    bounds: &Rectangle,
    mod_range: Option<&ModulationRange>,
    style: &Option<ModRangeAppearance>,
) {
    let Some(mod_range) = mod_range else { return };
    let Some(style) = style else { return };

    let (x, width) = match style.placement {
        ModRangePlacement::Center { width, offset } => {
            (bounds.x + offset + ((bounds.width - width) / 2.0), width)
        }
        ModRangePlacement::CenterFilled { edge_padding } => {
            (bounds.x + edge_padding, bounds.width - (edge_padding * 2.0))
        }
        ModRangePlacement::Left { width, offset } => {
            (bounds.x + offset - width, width)
        }
        ModRangePlacement::Right { width, offset } => {
            (bounds.x + bounds.width + offset, width)
        }
    };

    let Some(back_color) = style.back_color else {
        return;
    };

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x,
                y: bounds.y,
                width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4],
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        },
        Background::Color(back_color),
    );

    if mod_range.filled_visible
        && (mod_range.start.as_f32() != mod_range.end.as_f32())
    {
        let (start, end, color) =
            if mod_range.start.as_f32() > mod_range.end.as_f32() {
                (
                    mod_range.start.as_f32_inv(),
                    mod_range.end.as_f32_inv(),
                    style.filled_color,
                )
            } else {
                (
                    mod_range.end.as_f32_inv(),
                    mod_range.start.as_f32_inv(),
                    style.filled_inverse_color,
                )
            };

        let start_offset = bounds.height * start;
        let filled_height = (bounds.height * end) - start_offset;

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x,
                    y: bounds.y + start_offset,
                    width,
                    height: filled_height,
                },
                border_radius: [style.back_border_radius; 4],
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(color),
        );
    }
}

fn draw_texture_style<Theme>(
    renderer: &iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: TextureAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (f32::from(style.handle_height) / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - f32::from(style.handle_height),
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        renderer,
        &value_bounds,
        &value_bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let (left_rail, right_rail) =
        draw_classic_rail(renderer, bounds, &style.rail);

    // FIXME fengalin
    let handle = Primitive::Image {
        handle: style.image_handle,
        bounds: Rectangle {
            x: (bounds.center_x() + style.image_bounds.x).round(),
            y: (value_bounds.y
                + style.image_bounds.y
                + normal.scale_inv(value_bounds.height))
            .round(),
            width: style.image_bounds.width,
            height: style.image_bounds.height,
        },
    };
}

fn draw_classic_style<Theme>(
    renderer: &iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: &ClassicAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let handle_height = f32::from(style.handle.height);

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        renderer,
        &value_bounds,
        &value_bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let (left_rail, right_rail) =
        draw_classic_rail(renderer, bounds, &style.rail);

    let handle_border_radius = style.handle.border_radius;
    let handle_offset = normal.scale_inv(value_bounds.height).round();
    let notch_width = style.handle.notch_width;

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + handle_offset,
                width: bounds.width,
                height: handle_height,
            },
            border_radius: [handle_border_radius; 4],
            border_width: style.handle.border_width,
            border_color: style.handle.border_color,
        },
        Background::Color(style.handle.color),
    );

    if style.handle.notch_width != 0.0 {
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: (bounds.y + handle_offset + (handle_height / 2.0)
                        - (notch_width / 2.0))
                        .round(),
                    width: bounds.width,
                    height: notch_width,
                },
                border_radius: [0.0; 4],
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(style.handle.notch_color),
        );
    }
}

fn draw_rect_style<Theme>(
    renderer: &iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let handle_height = f32::from(style.handle_height);

    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
    };

    let (tick_marks, text_marks, mod_range_1, mod_range_2) = draw_value_markers(
        renderer,
        &value_bounds,
        bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4],
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        },
        Background::Color(style.back_color),
    );

    let handle_offset = normal
        .scale_inv(value_bounds.height - twice_border_width)
        .round();

    let filled_offset = handle_offset + handle_height + style.handle_filled_gap;
    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + filled_offset,
                width: bounds.width,
                height: bounds.height - filled_offset,
            },
            border_radius: [style.back_border_radius; 4],
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(style.filled_color),
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + handle_offset,
                width: bounds.width,
                height: handle_height + twice_border_width,
            },
            border_radius: [style.back_border_radius; 4],
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(style.handle_color),
    );
}

fn draw_rect_bipolar_style<Theme>(
    renderer: &iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectBipolarAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let handle_height = f32::from(style.handle_height);

    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let value_bounds = Rectangle {
        x: bounds.x,
        y: (bounds.y + (handle_height / 2.0)).round(),
        width: bounds.width,
        height: bounds.height - handle_height,
    };

    draw_value_markers(
        renderer,
        &value_bounds,
        bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4],
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        },
        Background::Color(style.back_color),
    );

    let handle_offset = normal
        .scale_inv(value_bounds.height - twice_border_width)
        .round();

    let (handle_color, filled_rect) = if normal.as_f32() > 0.499
        && normal.as_f32() < 0.501
    {
        style.handle_center_color
    } else if normal.as_f32() > 0.5 {
        let filled_rect_offset =
            handle_offset + handle_height + style.handle_filled_gap;

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + filled_rect_offset,
                    width: bounds.width,
                    height: ((bounds.height / 2.0) - filled_rect_offset
                        + twice_border_width),
                },
                border_radius: [style.back_border_radius; 4],
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(style.top_filled_color),
        );

        style.handle_top_color
    } else {
        let filled_rect_offset = (bounds.height / 2.0).round() - border_width;

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + filled_rect_offset,
                    width: bounds.width,
                    height: handle_offset - filled_rect_offset
                        + twice_border_width,
                },
                border_radius: [style.back_border_radius; 4],
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(style.bottom_filled_color),
        );

        style.handle_bottom_color
    };

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y + handle_offset,
                width: bounds.width,
                height: handle_height + twice_border_width,
            },
            border_radius: [style.back_border_radius; 4],
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(handle_color),
    );
}

fn draw_classic_rail<Theme>(
    renderer: &iced::Renderer<Theme>,
    bounds: &Rectangle,
    style: &ClassicRail,
) {
    let (left_width, right_width) = style.rail_widths;
    let (left_color, right_color) = style.rail_colors;

    let left_width = left_width;
    let right_width = right_width;

    let full_width = left_width + right_width;

    let start_x = (bounds.x + ((bounds.width - full_width) / 2.0)).round();

    let y = bounds.y + style.rail_padding;
    let height = bounds.height - (style.rail_padding * 2.0);

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: start_x,
                y,
                width: left_width,
                height,
            },
            border_radius: [0.0; 4],
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(left_color),
    );

    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x: start_x + left_width,
                y,
                width: right_width,
                height,
            },
            border_radius: [0.0; 4],
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(right_color),
    );
}
