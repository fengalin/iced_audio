//! Style for the [`XYPad`] widget
//!
//! [`XYPad`]: ../native/xy_pad/struct.XYPad.html

use iced_style::Color;

use crate::style::default_colors;

/// The appearance of an [`XYPad`].
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`HandleShape`]: enum.HandleShape.html
#[derive(Debug, Clone)]
pub struct Appearance {
    /// the width of the horizontal and vertical rail lines
    pub rail_width: f32,
    /// color of the horizontal rail line
    pub h_rail_color: Color,
    /// color of the vertical rail line
    pub v_rail_color: Color,
    /// the [`HandleShape`] of the handle
    ///
    /// [`HandleShape`]: enum.HandleShape.html
    pub handle: HandleShape,
    /// the color of the background square
    pub back_color: Color,
    /// the width of the border of the background square
    pub border_width: f32,
    /// the color of the border of the background square
    pub border_color: Color,
    /// the width of the center line markings
    pub center_line_width: f32,
    /// the color of the center line markings
    pub center_line_color: Color,
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance {
            rail_width: 2.0,
            h_rail_color: default_colors::XY_PAD_RAIL,
            v_rail_color: default_colors::XY_PAD_RAIL,
            handle: HandleShape::Circle(Default::default()),
            back_color: default_colors::LIGHT_BACK,
            border_width: 1.0,
            border_color: default_colors::BORDER,
            center_line_width: 1.0,
            center_line_color: default_colors::XY_PAD_CENTER_LINE,
        }
    }
}

/// The shape of the handle for the [`Style`] of an [`XYPad`]
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`Style`]: struct.Style.html
#[derive(Debug, Clone)]
pub enum HandleShape {
    /// a circular handle
    Circle(HandleCircle),
    /// a square handle
    Square(HandleSquare),
}

/// a circular handle style for the [`Style`] of an [`XYPad`]
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`Style`]: struct.Style.html
#[derive(Debug, Clone)]
pub struct HandleCircle {
    /// the color of the circle
    pub color: Color,
    /// the diameter of the circle
    pub diameter: f32,
    /// the width of the border of the circle
    pub border_width: f32,
    /// the color of the border of the circle
    pub border_color: Color,
}

impl Default for HandleCircle {
    fn default() -> Self {
        HandleCircle {
            color: default_colors::LIGHT_BACK,
            diameter: 11.0,
            border_width: 2.0,
            border_color: default_colors::BORDER,
        }
    }
}

/// a square handle style for the [`Style`] of an [`XYPad`]
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
/// [`Style`]: struct.Style.html
#[derive(Debug, Clone)]
pub struct HandleSquare {
    /// the color of the square
    pub color: Color,
    /// the size of the square
    pub size: u16,
    /// the width of the border of the square
    pub border_width: f32,
    /// the radius of the corners of the square
    pub border_radius: f32,
    /// the color of the border of the square
    pub border_color: Color,
}

/// A set of rules that dictate the style of an [`XYPad`].
///
/// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of an active [`XYPad`].
    ///
    /// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of a hovered [`XYPad`].
    ///
    /// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
    fn hovered(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of an [`XYPad`] that is being dragged.
    ///
    /// [`XYPad`]: ../../native/xy_pad/struct.XYPad.html
    fn dragging(&self, style: &Self::Style) -> Appearance;
}
