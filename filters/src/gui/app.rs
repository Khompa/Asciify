use rfd::AsyncFileDialog;
use iced::{Task, Element, Theme};
use iced::widget::{button};
use image::{DynamicImage, ImageReader};
use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum Message {
    OpenImage,
    ImageLoaded(Result<DynamicImage, String>),
    SelectFilter,
    SaveImage,
}

pub struct FilterApp {
    image: Option<DynamicImage>,
    file_path: Option<PathBuf>,
    selected_filter: String,
    status: String,
}

impl FilterApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                image: None,
                file_path: None,
                selected_filter: String::new(),
                status: "Ready".to_string(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Filter App".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenImage => {               
                self.status = "Opening file dialog...".to_string();
                Task::perform(load_image_async(), Message::ImageLoaded)
            }
            Message::ImageLoaded(result) => {
                match result {
                    Ok(img) => {
                        self.image = Some(img);
                        self.status = "Image loaded successfully".to_string();
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
        column![
            text(&self.status),
            button("+").on_press(Message::OpenImage),
        ]
        .spacing(10)
        .padding(20)
        .into()        
    }

    pub fn theme(&self) -> Theme {
        Theme::default()
    }
}

async fn load_image_async() -> Result<DynamicImage, String> {
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

    Ok(img)
}

