use iced::{alignment, keyboard, Point};
use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::text::Text;
use iced_native::widget::Widget;
use iced_native::{event, text, Clipboard, Shell};
use iced_native::{Color, Element, Length, Rectangle, Size};

#[allow(missing_debug_implementations)]
pub struct MyText<'a, Message, Renderer: text::Renderer> {
    content: String,
    size: Option<u16>,
    color: Option<Color>,
    font: Renderer::Font,
    width: Length,
    height: Length,
    on_change: Box<dyn Fn(String) -> Message + 'a>,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
}

impl<'a, Message, Renderer: text::Renderer> MyText<'a, Message, Renderer> {
    /// Create a new fragment of [`Text`] with the given contents.
    pub fn new<F>(content: &str, on_change: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        MyText {
            content: String::from(content),
            size: None,
            color: None,
            font: Default::default(),
            width: Length::Fill,
            height: Length::Units(20),
            on_change: Box::new(on_change),
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
        }
    }

    /// Sets the size of the [`Text`].
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the [`Color`] of the [`Text`].
    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the [`Font`] of the [`Text`].
    ///
    /// [`Font`]: crate::text::Renderer::Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = font.into();
        self
    }

    /// Sets the width of the [`Text`] boundaries.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Text`] boundaries.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the [`alignment::Horizontal`] of the [`Text`].
    pub fn horizontal_alignment(mut self, alignment: alignment::Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`alignment::Vertical`] of the [`Text`].
    pub fn vertical_alignment(mut self, alignment: alignment::Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    pub fn content(mut self, new_content: String) -> Self {
        self.content = new_content;
        self
    }
}

// pub fn my_text(content: String) -> MyText<Renderer>
//     where T: ToString
// {
//     MyText::new(content)
// }

impl<'a, Message, Renderer> Widget<Message, Renderer> for MyText<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: text::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or(renderer.default_size());

        let bounds = limits.max();

        let (width, height) = renderer.measure(&self.content, size, self.font.clone(), bounds);

        let size = limits.resolve(Size::new(width, height));

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw(
            renderer,
            style,
            layout,
            &self.content.to_string(),
            self.font.clone(),
            self.size,
            self.color,
            self.horizontal_alignment,
            self.vertical_alignment,
        );
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        update(
            event,
            layout,
            cursor_position,
            renderer,
            shell,
            &mut self.content,
            self.size,
            &self.font,
            self.on_change.as_ref(),
        )
    }
}

fn update<'a, Message, Renderer>(
    _event: iced_native::Event,
    layout: Layout,
    _cursor_position: Point,
    renderer: &Renderer,
    shell: &mut Shell<Message>,
    content: &mut String,
    size: Option<u16>,
    font: &Renderer::Font,
    on_change: &dyn Fn(String) -> Message,
) -> event::Status
where
    Message: Clone,
    Renderer: text::Renderer,
{
    let text = content.to_string();
    let size = size.unwrap_or(renderer.default_size());

    let text_width = renderer.measure_width(&text, size, font.clone());

    let bounds = layout.bounds();

    // match event {
    //     iced_native::Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
    //         content.insert_str(
    //             content.len(),
    //             format!("{} ", key_code_to_key(&key_code)).as_str(),
    //         );
    //         let message = (on_change)(content.clone());
    //         shell.publish(message);
    //         return event::Status::Captured;
    //     }
    //     _ => {}
    // }
    if text_width > bounds.width {
        let (_oldest_key, splitted_content) = content.split_once(" ").unwrap();
        *content = splitted_content.to_string();
        let message = (on_change)(content.clone());
        shell.publish(message);
        return event::Status::Captured;
    }
    event::Status::Ignored
}

impl<'a, Message, Renderer> From<MyText<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + text::Renderer,
{
    fn from(my_text: MyText<'a, Message, Renderer>) -> Self {
        Self::new(my_text)
    }
}
pub fn draw<Renderer>(
    renderer: &mut Renderer,
    style: &renderer::Style,
    layout: Layout<'_>,
    content: &String,
    font: Renderer::Font,
    size: Option<u16>,
    color: Option<Color>,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
) where
    Renderer: text::Renderer,
{
    let bounds = layout.bounds();

    let x = match horizontal_alignment {
        alignment::Horizontal::Left => bounds.x,
        alignment::Horizontal::Center => bounds.center_x(),
        alignment::Horizontal::Right => bounds.x + bounds.width,
    };

    let y = match vertical_alignment {
        alignment::Vertical::Top => bounds.y,
        alignment::Vertical::Center => bounds.center_y(),
        alignment::Vertical::Bottom => bounds.y + bounds.height,
    };

    renderer.fill_text(Text {
        content: content.as_str(),
        size: f32::from(size.unwrap_or(renderer.default_size())),
        bounds: Rectangle { x, y, ..bounds },
        color: color.unwrap_or(style.text_color),
        font,
        horizontal_alignment,
        vertical_alignment,
    });
}
