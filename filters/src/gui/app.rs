use iced::widget::image::Handle;
use iced::widget::{Image, button, responsive, slider, text_input};
use iced::{Element, Length, Task, Theme, alignment};
use image::{DynamicImage, ImageReader};
use rfd::AsyncFileDialog;
use std::path::PathBuf;
#[path = "../filters/ascii.rs"]
mod ascii;

#[derive(Debug, Clone)]
pub enum Message {
    OpenImage,
    ImageLoaded(Result<(DynamicImage, PathBuf), String>),
    Asciify,
    AsciiGenerated((Vec<(u8, u8, u8)>, Vec<char>, usize)),
    AsciiInput(String),
    StepSize(u8),
}

pub struct FilterApp {
    image_handle: Option<Handle>,
    image: Option<DynamicImage>,
    file_path: Option<PathBuf>,
    ascii_data: Option<(Vec<(u8, u8, u8)>, Vec<char>, usize)>,
    ascii_input: String,
    step_size: u8,
    status: String,
}

impl FilterApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                image_handle: None,
                image: None,
                file_path: None,
                ascii_data: None,
                ascii_input: String::new(),
                step_size: 8,
                status: "Ready".to_string(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenImage => {
                self.status = "Opening file dialog...".to_string();
                Task::perform(load_image_async(), Message::ImageLoaded)
            }
            Message::ImageLoaded(result) => {
                match result {
                    Ok((img, path)) => {
                        let mut bytes = Vec::new();
                        match img.write_to(
                            &mut std::io::Cursor::new(&mut bytes),
                            image::ImageFormat::Png,
                        ) {
                            Ok(_) => {
                                self.image_handle = Some(Handle::from_bytes(bytes));
                                self.image = Some(img);
                                self.file_path = Some(path);
                                self.status = "Image loaded successfully".to_string();
                                self.ascii_data = None; // reset ascii version when a new image is loaded
                            }
                            Err(e) => {
                                self.status = format!("Failed to encode image: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        self.status = format!("Error: {}", e);
                    }
                }
                Task::none()
            }
            Message::AsciiGenerated(data) => {
                self.ascii_data = Some(data);
                self.status = "Ready".to_string();
                Task::none()
            }
            Message::AsciiInput(input) => {
                self.ascii_input = input;

                if self.ascii_data.is_some() {
                    if let Some(img) = &self.image {
                        let img_clone = img.clone();
                        let chars = self.ascii_input.clone();
                        let step = self.step_size as usize;

                        return Task::perform(
                            async move { ascii::img_to_ascii(img_clone, &chars, step) },
                            Message::AsciiGenerated,
                        );
                    }
                }
                Task::none()
            }
            Message::StepSize(input) => {
                self.step_size = input;

                if self.ascii_data.is_some() {
                    if let Some(img) = &self.image {
                        let img_clone = img.clone();
                        let chars = self.ascii_input.clone();
                        let step = self.step_size as usize;

                        return Task::perform(
                            async move { ascii::img_to_ascii(img_clone, &chars, step) },
                            Message::AsciiGenerated,
                        );
                    }
                }
                Task::none()
            }
            Message::Asciify => {
                if let Some(img) = &self.image {
                    self.status = "Generating ASCII...".to_string();
                    let img_clone = img.clone();
                    let chars = self.ascii_input.clone();
                    let step = self.step_size as usize;

                    return Task::perform(
                        async move { ascii::img_to_ascii(img_clone, &chars, step) },
                        Message::AsciiGenerated,
                    );
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        use iced::widget::{column, row, text};

        let action_button = if self.image.is_some() {
            button("Change Image").on_press(Message::OpenImage)
        } else {
            button("Upload Image").on_press(Message::OpenImage)
        };

        // Initial content at start
        let mut content = column![];

        if let Some(handle) = &self.image_handle {
            if let Some(ascii_data) = &self.ascii_data {
                let mut ascii_string = String::new();
                let (_, ascii_chars, cols) = ascii_data;

                let rows = ascii_chars.len() / cols;

                for (i, &character) in ascii_chars.iter().enumerate() {
                    ascii_string.push(character);

                    if (i + 1) % cols == 0 {
                        ascii_string.push('\n');
                    }
                }

                content = content.push(responsive(move |size| {
                    let max_height = size.height / rows as f32;
                    let max_width = (size.width / *cols as f32) / 0.6;
                    let optimal_size = max_height.min(max_width);

                    text(ascii_string.clone())
                        .font(iced::Font::MONOSPACE)
                        .size(optimal_size)
                        .line_height(1.0)
                        .width(Length::Fill)
                        .center()
                        .into()
                }));
            } else {
                content = content.push(Image::new(handle.clone()).width(400).height(300));
                content = content.push(row![
                    action_button,
                    button("Asciify").on_press(Message::Asciify)
                ]);
            }
            content = content.push(row![
                text_input(
                    "Enter ASCII characters. If left empty, a default set will be used.",
                    &self.ascii_input,
                )
                .on_input(Message::AsciiInput),
                slider(2..=20, self.step_size, Message::StepSize)
            ]);
        } else {
            let welcome_logo = r#"   _____                .__.__  _____       
  /  _  \   ______ ____ |__|__|/ ____\__.__.
 /  /_\  \ /  ___// ___\|  |  \   __<   |  |
/    |    \\___ \\  \___|  |  ||  |  \___  |
\____|__  /____  >\___  >__|__||__|  / ____|
        \/     \/     \/             \/     "#;

            content = content.push(
                text(welcome_logo)
                    .font(iced::Font::MONOSPACE)
                    .size(20)
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center),
            );
            content = content.push(action_button);
        }

        content
            .spacing(10)
            .padding(20)
            .align_x(iced::Alignment::Center)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::default()
    }
}

async fn load_image_async() -> Result<(DynamicImage, PathBuf), String> {
    let file_handle = AsyncFileDialog::new()
        .add_filter("Images", &["jpg", "png", "jpeg"])
        .set_title("Select an image file")
        .pick_file()
        .await
        .ok_or("No file selected")?;

    let path = file_handle.path();
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open file: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    Ok((img, path.to_path_buf()))
}
