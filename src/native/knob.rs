//! Display an interactive rotating knob that controls a [`NormalParam`]
//!
//! [`NormalParam`]: ../core/normal_param/struct.NormalParam.html

use std::fmt::Debug;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::Widget;
use iced::advanced::{Clipboard, Shell};
use iced::{event, keyboard, touch, Element, Event, Length, Rectangle, Size};
// Need mouse via iced_core because Click is not re-exported by iced
use iced_core::mouse;

use crate::core::{ModulationRange, Normal, NormalParam};
use crate::native::{text_marks, tick_marks, SliderStatus};
use crate::style::knob::StyleSheet;

static DEFAULT_SIZE: f32 = 30.0;
static DEFAULT_SCALAR: f32 = 0.00385;
static DEFAULT_WHEEL_SCALAR: f32 = 0.01;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.02;

/// A rotating knob GUI widget that controls a [`NormalParam`]
///
/// [`NormalParam`]: ../../core/normal_param/struct.NormalParam.html
#[allow(missing_debug_implementations)]
pub struct Knob<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Renderer::Theme: StyleSheet,
{
    normal_param: NormalParam,
    size: Length,
    on_change: Box<dyn 'a + Fn(Normal) -> Message>,
    on_grab: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    on_release: Option<Box<dyn 'a + FnMut() -> Option<Message>>>,
    scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,
    modifier_keys: keyboard::Modifiers,
    bipolar_center: Option<Normal>,
    style: <Renderer::Theme as StyleSheet>::Style,
    tick_marks: Option<&'a tick_marks::Group>,
    text_marks: Option<&'a text_marks::Group>,
    mod_range_1: Option<&'a ModulationRange>,
    mod_range_2: Option<&'a ModulationRange>,
}

impl<'a, Message, Renderer> Knob<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`Knob`].
    ///
    /// It expects:
    ///   * the [`NormalParam`] of the [`Knob`]
    ///   * a function that will be called when the [`Knob`] is turned.
    ///
    /// [`NormalParam`]: struct.NormalParam.html
    /// [`Knob`]: struct.Knob.html
    pub fn new<F>(normal_param: NormalParam, on_change: F) -> Self
    where
        F: 'a + Fn(Normal) -> Message,
    {
        Knob {
            normal_param,
            size: Length::Fixed(DEFAULT_SIZE),
            on_change: Box::new(on_change),
            on_grab: None,
            on_release: None,
            scalar: DEFAULT_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,
            modifier_keys: keyboard::Modifiers::CTRL,
            bipolar_center: None,
            style: Default::default(),
            tick_marks: None,
            text_marks: None,
            mod_range_1: None,
            mod_range_2: None,
        }
    }

    /// Sets the grab message of the [`Knob`].
    /// This is called when the mouse grabs from the knob.
    ///
    /// Typically, the user's interaction with the knob starts when this message is produced.
    /// This is useful for some environments so that external changes, such as automation,
    /// don't interfer with user's changes.
    pub fn on_grab(
        mut self,
        on_grab: impl 'a + FnMut() -> Option<Message>,
    ) -> Self {
        self.on_grab = Some(Box::new(on_grab));
        self
    }

    /// Sets the release message of the [`Knob`].
    /// This is called when the mouse is released from the knob.
    ///
    /// Typically, the user's interaction with the knob is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the knob's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(
        mut self,
        on_release: impl 'a + FnMut() -> Option<Message>,
    ) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Sets the diameter of the [`Knob`]. The default size is
    /// `Length::from(Length::Fixed(31))`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn size(mut self, size: Length) -> Self {
        self.size = size;
        self
    }

    /// Sets the style of the [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per `y`
    /// pixel movement of the mouse.
    ///
    /// The default value is `0.00385`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn scalar(mut self, scalar: f32) -> Self {
        self.scalar = scalar;
        self
    }

    /// Sets how much the [`Normal`] value will change for the [`Knob`] per line scrolled
    /// by the mouse wheel.
    ///
    /// This can be set to `0.0` to disable the scroll wheel from moving the parameter.
    ///
    /// The default value is `0.01`
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`Normal`]: ../../core/struct.Normal.html
    pub fn wheel_scalar(mut self, wheel_scalar: f32) -> Self {
        self.wheel_scalar = wheel_scalar;
        self
    }

    /// Sets the modifier keys of the [`Knob`].
    ///
    /// The default modifier key is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_keys(mut self, modifier_keys: keyboard::Modifiers) -> Self {
        self.modifier_keys = modifier_keys;
        self
    }

    /// Sets the scalar to use when the user drags the knobs while holding down
    /// the modifier key. This is multiplied to the value set by
    /// `Knob::scalar()` (which the default is `0.00385`).
    ///
    /// For example, a `modifier_scalar` of `0.5` will cause the knob to turn
    /// half as fast when the modifier key is down.
    ///
    /// The default `modifier_scalar` is `0.02`, and the default modifier key
    /// is `Ctrl`.
    ///
    /// [`Knob`]: struct.Knob.html
    pub fn modifier_scalar(mut self, scalar: f32) -> Self {
        self.modifier_scalar = scalar;
        self
    }

    /// Sets the tick marks to display. Note your [`StyleSheet`] must
    /// also implement `tick_marks_style(&self) -> Option<tick_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    pub fn tick_marks(mut self, tick_marks: &'a tick_marks::Group) -> Self {
        self.tick_marks = Some(tick_marks);
        self
    }

    /// Sets the text marks to display. Note your [`StyleSheet`] must
    /// also implement `text_marks_style(&self) -> Option<text_marks::Style>` for
    /// them to display (which the default style does).
    ///
    /// [`StyleSheet`]: ../../style/knob/trait.StyleSheet.html
    pub fn text_marks(mut self, text_marks: &'a text_marks::Group) -> Self {
        self.text_marks = Some(text_marks);
        self
    }

    /// Sets a [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range(mut self, mod_range: &'a ModulationRange) -> Self {
        self.mod_range_1 = Some(mod_range);
        self
    }

    /// Sets a second [`ModulationRange`] to display. Note your [`StyleSheet`] must
    /// also implement `mod_range_style_2(&self) -> Option<ModRangeStyle>` for
    /// them to display.
    ///
    /// [`ModulationRange`]: ../../core/struct.ModulationRange.html
    /// [`StyleSheet`]: ../../style/v_slider/trait.StyleSheet.html
    pub fn mod_range_2(mut self, mod_range: &'a ModulationRange) -> Self {
        self.mod_range_1 = Some(mod_range);
        self
    }

    /// Sets the value to be considered the center of the [`Knob`]. Only has
    /// an effect when using [`ArcBipolarStyle`].
    ///
    /// [`Knob`]: struct.Knob.html
    /// [`ArcBipolarStyle`]: ../../style/knob/struct.ArcBipolarStyle.html
    pub fn bipolar_center(mut self, bipolar_center: Normal) -> Self {
        self.bipolar_center = Some(bipolar_center);
        self
    }

    fn move_virtual_slider(
        &mut self,
        state: &mut State,
        mut normal_delta: f32,
    ) -> SliderStatus {
        if normal_delta.abs() < f32::EPSILON {
            return SliderStatus::Unchanged;
        }

        if state.pressed_modifiers.contains(self.modifier_keys) {
            normal_delta *= self.modifier_scalar;
        }

        self.normal_param
            .value
            .set_clipped(state.continuous_normal - normal_delta);
        state.continuous_normal = self.normal_param.value.as_f32();

        SliderStatus::Moved
    }

    fn maybe_fire_on_grab(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) =
            self.on_grab.as_mut().and_then(|on_grab| on_grab())
        {
            shell.publish(message);
        }
    }

    fn fire_on_change(&self, shell: &mut Shell<'_, Message>) {
        shell.publish((self.on_change)(self.normal_param.value));
    }

    fn maybe_fire_on_release(&mut self, shell: &mut Shell<'_, Message>) {
        if let Some(message) =
            self.on_release.as_mut().and_then(|on_release| on_release())
        {
            shell.publish(message);
        }
    }
}

/// The local state of a [`Knob`].
///
/// [`Knob`]: struct.Knob.html
#[derive(Debug)]
struct State {
    dragging_status: Option<SliderStatus>,
    prev_drag_y: f32,
    prev_normal: Normal,
    continuous_normal: f32,
    pressed_modifiers: keyboard::Modifiers,
    last_click: Option<mouse::Click>,
    tick_marks_cache: crate::graphics::tick_marks::Cache,
    text_marks_cache: crate::graphics::text_marks::Cache,
}

impl State {
    /// Creates a new [`Knob`] state.
    ///
    /// It expects:
    /// * current [`Normal`] value for the [`Knob`]
    ///
    /// [`Normal`]: ../../core/normal/struct.Normal.html
    /// [`Knob`]: struct.Knob.html
    fn new(normal: Normal) -> Self {
        Self {
            dragging_status: None,
            prev_drag_y: 0.0,
            prev_normal: normal,
            continuous_normal: normal.as_f32(),
            pressed_modifiers: Default::default(),
            last_click: None,
            tick_marks_cache: Default::default(),
            text_marks_cache: Default::default(),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Knob<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.normal_param.value))
    }

    fn width(&self) -> Length {
        self.size
    }

    fn height(&self) -> Length {
        self.size
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.size).height(self.size);

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = state.state.downcast_mut::<State>();

        // Update state after a discontinuity
        if state.dragging_status.is_none()
            && state.prev_normal != self.normal_param.value
        {
            state.prev_normal = self.normal_param.value;
            state.continuous_normal = self.normal_param.value.as_f32();
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.dragging_status.is_none() {
                    return event::Status::Ignored;
                }
                let Some(position) = cursor.position() else {
                    return event::Status::Ignored;
                };

                let normal_delta =
                    (position.y - state.prev_drag_y) * self.scalar;

                state.prev_drag_y = position.y;

                if self.move_virtual_slider(state, normal_delta).was_moved() {
                    self.fire_on_change(shell);

                    state
                        .dragging_status
                        .as_mut()
                        .expect("dragging_status taken")
                        .moved();
                }

                return event::Status::Captured;
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if self.wheel_scalar == 0.0 {
                    return event::Status::Ignored;
                }

                if !cursor
                    .position()
                    .map_or(false, |pos| layout.bounds().contains(pos))
                {
                    return event::Status::Ignored;
                }

                let lines = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y,
                    mouse::ScrollDelta::Pixels { y, .. } => {
                        if y > 0.0 {
                            1.0
                        } else if y < 0.0 {
                            -1.0
                        } else {
                            0.0
                        }
                    }
                };

                if lines == 0.0 {
                    return event::Status::Ignored;
                }

                let normal_delta = -lines * self.wheel_scalar;

                if self.move_virtual_slider(state, normal_delta).was_moved() {
                    if state.dragging_status.is_none() {
                        self.maybe_fire_on_grab(shell);
                    }

                    self.fire_on_change(shell);

                    if let Some(slider_status) = state.dragging_status.as_mut()
                    {
                        // Widget was grabbed => keep it grabbed
                        slider_status.moved();
                    } else {
                        self.maybe_fire_on_release(shell);
                    }
                }

                return event::Status::Captured;
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let Some(position) = cursor.position() else {
                    return event::Status::Ignored;
                };
                if !layout.bounds().contains(position) {
                    return event::Status::Ignored;
                }

                let click = mouse::Click::new(position, state.last_click);

                match click.kind() {
                    mouse::click::Kind::Single => {
                        self.maybe_fire_on_grab(shell);

                        state.dragging_status = Some(Default::default());
                        state.prev_drag_y = position.y;
                    }
                    _ => {
                        // Reset to default

                        let prev_dragging_status = state.dragging_status.take();

                        if self.normal_param.value != self.normal_param.default
                        {
                            if prev_dragging_status.is_none() {
                                self.maybe_fire_on_grab(shell);
                            }

                            self.normal_param.value = self.normal_param.default;

                            self.fire_on_change(shell);

                            self.maybe_fire_on_release(shell);
                        } else if prev_dragging_status.is_some() {
                            self.maybe_fire_on_release(shell);
                        }
                    }
                }

                state.last_click = Some(click);

                return event::Status::Captured;
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if let Some(slider_status) = state.dragging_status.take() {
                    if self.on_grab.is_some() || slider_status.was_moved() {
                        // maybe fire on release if `on_grab` is defined
                        // so as to terminate the action, regardless of the actual user movement.
                        self.maybe_fire_on_release(shell);
                    }

                    return event::Status::Captured;
                }
            }
            Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::KeyReleased { modifiers, .. } => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                keyboard::Event::ModifiersChanged(modifiers) => {
                    state.pressed_modifiers = modifiers;

                    return event::Status::Captured;
                }
                _ => {}
            },
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = state.state.downcast_ref::<State>();
        renderer.draw(
            layout.bounds(),
            cursor,
            self.normal_param.value,
            self.bipolar_center,
            state.dragging_status.is_some(),
            self.mod_range_1,
            self.mod_range_2,
            self.tick_marks,
            self.text_marks,
            theme,
            &self.style,
            &state.tick_marks_cache,
            &state.text_marks_cache,
        )
    }
}

/// The renderer of a [`Knob`].
///
/// Your renderer will need to implement this trait before being
/// able to use a [`Knob`] in your user interface.
///
/// [`Knob`]: struct.Knob.html
pub trait Renderer: renderer::Renderer
where
    Self::Theme: StyleSheet,
{
    /// Draws a [`Knob`].
    ///
    /// It receives:
    ///   * the bounds of the [`Knob`]
    ///   * the current cursor position
    ///   * the current normal of the [`Knob`]
    ///   * optionally, a custom bipolar center value
    ///   * whether the knob is currently being dragged
    ///   * any tick marks to display
    ///   * any text marks to display
    ///   * the style of the [`Knob`]
    ///
    /// [`Knob`]: struct.Knob.html
    #[allow(clippy::too_many_arguments)]
    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor: mouse::Cursor,
        normal: Normal,
        bipolar_center: Option<Normal>,
        dragging_status: bool,
        mod_range_1: Option<&ModulationRange>,
        mod_range_2: Option<&ModulationRange>,
        tick_marks: Option<&tick_marks::Group>,
        text_marks: Option<&text_marks::Group>,
        style_sheet: &dyn StyleSheet<
            Style = <Self::Theme as StyleSheet>::Style,
        >,
        style: &<Self::Theme as StyleSheet>::Style,
        tick_marks_cache: &crate::tick_marks::Cache,
        text_marks_cache: &crate::text_marks::Cache,
    );
}

impl<'a, Message, Renderer> From<Knob<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + self::Renderer,
    Renderer::Theme: 'a + StyleSheet,
{
    fn from(
        knob: Knob<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(knob)
    }
}
