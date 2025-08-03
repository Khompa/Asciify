use rfd::FileDialog;
use iced::{Application, Task, Element, Theme, Settings};
use iced::widget::{button};
use image::{DynamicImage, ImageReader};
use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum Message {
    OpenImage,
    SelectFilter,
    SaveImage,
}

pub struct FilterApp {
    image: Option<DynamicImage>,
    file_path: Option<PathBuf>,
    selected_filter: String,
}

impl Application for FilterApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Self {
                image: None,
                file_path: None,
                selected_filter: String::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        "Filter App".to_string()
    }

    fn update(&mut self, message: Message) -> Result<(), Box<dyn std::error::Error>> {
        match message {
            Message::OpenImage => {               
                let path = FileDialog::new()
                    .add_filter("Images", &["jpg", "png", "jpeg"])
                    .set_title("Select and image file")
                    .pick_file()
                    .ok_or("No file selected")?;

                let img = ImageReader::open(&path)?.decode()?;
                self.image = Some(img);
                self.file_path = Some(path);
                Ok(())
            }
            Message::SelectFilter => {
                Ok(())
            }
            Message::SaveImage => {
                Ok(())
            }
        }
    }

    fn view(&self) -> Element<Message> {
        button("+").on_press(Message::OpenImage).into()
    }
}



