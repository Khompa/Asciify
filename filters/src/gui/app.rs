use rfd::AsyncFileDialog;
use iced::{Task, Element, Theme};
use iced::widget::{button, Image};
use iced::widget::image::Handle;
use image::{DynamicImage, ImageReader};
use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum Message {
    OpenImage,
    ImageLoaded(Result<(DynamicImage, PathBuf), String>),
    SelectFilter,
    SaveImage,
}

pub struct FilterApp {
    image_handle: Option<Handle>,
    image: Option<DynamicImage>,
    file_path: Option<PathBuf>,
    selected_filter: String,
    status: String,
}

impl FilterApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                image_handle: None,
                image: None,
                file_path: None,
                selected_filter: String::new(),
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
                        match img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png) {
                            Ok(_) => {
                                self.image_handle = Some(Handle::from_bytes(bytes));
                                self.image = Some(img);
                                self.file_path = Some(path);
                                self.status = "Image loaded successfully".to_string();
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
            Message::SelectFilter => {
                Task::none()
            }
            Message::SaveImage => {
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        use iced::widget::{column, text};
        let mut content = column![
            button("Upload image").on_press(Message::OpenImage),
        ];

        if let Some(handle) = &self.image_handle {
            content = content.push(
                Image::new(handle.clone())
                .width(400)
                .height(300)
            );
        }
        content
            .spacing(10)
            .padding(20)
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

