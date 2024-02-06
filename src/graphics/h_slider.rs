//! Display an interactive horizontal slider that controls a [`Param`]
//!
//! [`Param`]: ../core/param/trait.Param.html

use crate::core::{ModulationRange, Normal};
use crate::graphics::{text_marks, tick_marks};
use crate::native::h_slider;

#[cfg(feature = "image")]
use iced::advanced::image;
use iced::advanced::renderer::Quad;
use iced::advanced::{self, mouse};
use iced::{Background, Color, Rectangle};

pub use crate::style::h_slider::{
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

/// A horizontal slider GUI widget that controls a [`Param`]
///
/// an [`HSlider`] will try to fill the horizontal space of its container.
///
/// [`Param`]: ../../core/param/trait.Param.html
/// [`HSlider`]: struct.HSlider.html
pub type HSlider<'a, Message, Theme> =
    h_slider::HSlider<'a, Message, iced::Renderer<Theme>>;

impl<Theme> h_slider::Renderer for iced::Renderer<Theme>
where
    Self::Theme: StyleSheet,
{
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor: mouse::Cursor,
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
        let is_mouse_over =
            cursor.position().map_or(false, |pos| bounds.contains(pos));

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
            Appearance::Texture(_style) => {
                #[cfg(feature = "image")]
                draw_texture_style(
                    self,
                    normal,
                    &bounds,
                    _style,
                    &value_markers,
                    tick_marks_cache,
                    text_marks_cache,
                );

                #[cfg(not(feature = "image"))]
                panic!("Build with 'image' feature for texture style support");
            }
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
    renderer: &mut iced::Renderer<Theme>,
    mark_bounds: &Rectangle,
    mod_bounds: &Rectangle,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    draw_tick_marks(
        renderer,
        mark_bounds,
        value_markers.tick_marks,
        &value_markers.tick_marks_style,
        tick_marks_cache,
    );
    draw_text_marks(
        renderer,
        mark_bounds,
        value_markers.text_marks,
        &value_markers.text_marks_style,
        text_marks_cache,
    );
    draw_mod_range(
        renderer,
        mod_bounds,
        value_markers.mod_range_1,
        &value_markers.mod_range_style_1,
    );
    draw_mod_range(
        renderer,
        mod_bounds,
        value_markers.mod_range_2,
        &value_markers.mod_range_style_2,
    );
}

fn draw_tick_marks<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    bounds: &Rectangle,
    tick_marks: Option<&tick_marks::Group>,
    tick_marks_style: &Option<TickMarksAppearance>,
    tick_marks_cache: &tick_marks::Cache,
) {
    let Some(tick_marks) = tick_marks else { return };
    let Some(style) = tick_marks_style else {
        return;
    };

    tick_marks::draw_horizontal_tick_marks(
        renderer,
        bounds,
        tick_marks,
        &style.style,
        &style.placement,
        false,
        tick_marks_cache,
    );
}

fn draw_text_marks<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    value_bounds: &Rectangle,
    text_marks: Option<&text_marks::Group>,
    text_marks_style: &Option<TextMarksAppearance>,
    text_marks_cache: &text_marks::Cache,
) {
    let Some(text_marks) = text_marks else { return };
    let Some(style) = text_marks_style else {
        return;
    };

    text_marks::draw_horizontal_text_marks(
        renderer,
        value_bounds,
        text_marks,
        &style.style,
        &style.placement,
        false,
        text_marks_cache,
    );
}

fn draw_mod_range<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    bounds: &Rectangle,
    mod_range: Option<&ModulationRange>,
    style: &Option<ModRangeAppearance>,
) {
    let Some(mod_range) = mod_range else { return };
    let Some(style) = style else { return };

    let (y, height) = match style.placement {
        ModRangePlacement::Center { height, offset } => {
            (bounds.y + offset + ((bounds.height - height) / 2.0), height)
        }
        ModRangePlacement::CenterFilled { edge_padding } => (
            bounds.y + edge_padding,
            bounds.height - (edge_padding * 2.0),
        ),
        ModRangePlacement::Top { height, offset } => {
            (bounds.y + offset - height, height)
        }
        ModRangePlacement::Bottom { height, offset } => {
            (bounds.y + bounds.height + offset, height)
        }
    };

    if let Some(back_color) = style.back_color {
        advanced::Renderer::fill_quad(
            renderer,
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y,
                    width: bounds.width,
                    height,
                },
                border_radius: [style.back_border_radius; 4].into(),
                border_width: style.back_border_width,
                border_color: style.back_border_color,
            },
            Background::Color(back_color),
        );
    }

    if mod_range.filled_visible
        && (mod_range.start.as_f32() != mod_range.end.as_f32())
    {
        let (start, end, color) =
            if mod_range.start.as_f32() < mod_range.end.as_f32() {
                (
                    mod_range.start.as_f32(),
                    mod_range.end.as_f32(),
                    style.filled_color,
                )
            } else {
                (
                    mod_range.end.as_f32(),
                    mod_range.start.as_f32(),
                    style.filled_inverse_color,
                )
            };

        let start_offset = bounds.width * start;
        let filled_width = (bounds.width * end) - start_offset;

        advanced::Renderer::fill_quad(
            renderer,
            Quad {
                bounds: Rectangle {
                    x: bounds.x + start_offset,
                    y,
                    width: filled_width,
                    height,
                },
                border_radius: [style.back_border_radius; 4].into(),
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(color),
        );
    }
}

#[cfg(feature = "image")]
fn draw_texture_style<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: TextureAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let value_bounds = Rectangle {
        x: (bounds.x + (f32::from(style.handle_width) / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - f32::from(style.handle_width),
        height: bounds.height,
    };

    draw_value_markers(
        renderer,
        &value_bounds,
        &value_bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    draw_classic_rail(renderer, bounds, &style.rail);

    image::Renderer::draw(
        renderer,
        style.image_handle,
        Rectangle {
            x: (value_bounds.x
                + style.image_bounds.x
                + normal.scale(value_bounds.width))
            .round(),
            y: (bounds.center_y() + style.image_bounds.y).round(),
            width: style.image_bounds.width,
            height: style.image_bounds.height,
        },
    );
}

fn draw_classic_style<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: &ClassicAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let handle_width = f32::from(style.handle.width);

    let value_bounds = Rectangle {
        x: (bounds.x + (handle_width / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - handle_width,
        height: bounds.height,
    };

    draw_value_markers(
        renderer,
        &value_bounds,
        &value_bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    draw_classic_rail(renderer, bounds, &style.rail);

    let handle_border_radius = style.handle.border_radius;
    let handle_offset = normal.scale(value_bounds.width).round();
    let notch_width = style.handle.notch_width;

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: bounds.y,
                width: handle_width,
                height: bounds.height,
            },
            border_radius: [handle_border_radius; 4].into(),
            border_width: style.handle.border_width,
            border_color: style.handle.border_color,
        },
        Background::Color(style.handle.color),
    );

    if style.handle.notch_width != 0.0 {
        advanced::Renderer::fill_quad(
            renderer,
            Quad {
                bounds: Rectangle {
                    x: (bounds.x + handle_offset + (handle_width / 2.0)
                        - (notch_width / 2.0))
                        .round(),
                    y: bounds.y,
                    width: notch_width,
                    height: bounds.height,
                },
                border_radius: [0.0; 4].into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(style.handle.notch_color),
        );
    }
}

fn draw_rect_style<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let handle_width = f32::from(style.handle_width);

    let value_bounds = Rectangle {
        x: (bounds.x + (handle_width / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - handle_width,
        height: bounds.height,
    };

    draw_value_markers(
        renderer,
        &value_bounds,
        bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4].into(),
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        },
        Background::Color(style.back_color),
    );

    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    let handle_offset = normal
        .scale(value_bounds.width - twice_border_width)
        .round();

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: handle_offset + twice_border_width
                    - style.handle_filled_gap,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4].into(),
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(style.filled_color),
    );

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: bounds.y,
                width: handle_width + twice_border_width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4].into(),
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(style.handle_color),
    );
}

fn draw_rect_bipolar_style<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    normal: Normal,
    bounds: &Rectangle,
    style: &RectBipolarAppearance,
    value_markers: &ValueMarkers<'_>,
    tick_marks_cache: &tick_marks::Cache,
    text_marks_cache: &text_marks::Cache,
) {
    let handle_width = f32::from(style.handle_width);

    let value_bounds = Rectangle {
        x: (bounds.x + (handle_width / 2.0)).round(),
        y: bounds.y,
        width: bounds.width - handle_width,
        height: bounds.height,
    };

    draw_value_markers(
        renderer,
        &value_bounds,
        bounds,
        value_markers,
        tick_marks_cache,
        text_marks_cache,
    );

    let border_width = style.back_border_width;
    let twice_border_width = border_width * 2.0;

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4].into(),
            border_width: style.back_border_width,
            border_color: style.back_border_color,
        },
        Background::Color(style.back_color),
    );

    let handle_offset = normal
        .scale(value_bounds.width - twice_border_width)
        .round();

    let handle_color = if normal.as_f32() > 0.499 && normal.as_f32() < 0.501 {
        style.handle_center_color
    } else if normal.as_f32() < 0.5 {
        let filled_rect_offset =
            handle_offset + handle_width + style.handle_filled_gap;

        advanced::Renderer::fill_quad(
            renderer,
            Quad {
                bounds: Rectangle {
                    x: bounds.x + filled_rect_offset,
                    y: bounds.y,
                    width: ((bounds.width / 2.0) - filled_rect_offset
                        + twice_border_width)
                        .round(),
                    height: bounds.height,
                },
                border_radius: [style.back_border_radius; 4].into(),
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(style.left_filled_color),
        );

        style.handle_left_color
    } else {
        let filled_rect_offset = (bounds.width / 2.0).round() - border_width;
        advanced::Renderer::fill_quad(
            renderer,
            Quad {
                bounds: Rectangle {
                    x: bounds.x + filled_rect_offset,
                    y: bounds.y,
                    width: handle_offset - filled_rect_offset
                        + twice_border_width
                        - style.handle_filled_gap,
                    height: bounds.height,
                },
                border_radius: [style.back_border_radius; 4].into(),
                border_width: style.back_border_width,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(style.right_filled_color),
        );

        style.handle_right_color
    };

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset,
                y: bounds.y,
                width: handle_width + twice_border_width,
                height: bounds.height,
            },
            border_radius: [style.back_border_radius; 4].into(),
            border_width: style.back_border_width,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(handle_color),
    );
}

fn draw_classic_rail<Theme>(
    renderer: &mut iced::Renderer<Theme>,
    bounds: &Rectangle,
    style: &ClassicRail,
) {
    let (top_width, bottom_width) = style.rail_widths;
    let (top_color, bottom_color) = style.rail_colors;

    let full_width = top_width + bottom_width;

    let x = bounds.x + style.rail_padding;
    let width = bounds.width - (style.rail_padding * 2.0);

    let start_y = (bounds.y + ((bounds.height - full_width) / 2.0)).round();

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x,
                y: start_y,
                width,
                height: top_width,
            },
            border_radius: [0.0; 4].into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(top_color),
    );

    advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds: Rectangle {
                x,
                y: start_y + top_width,
                width,
                height: bottom_width,
            },
            border_radius: [0.0; 4].into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(bottom_color),
    );
}
