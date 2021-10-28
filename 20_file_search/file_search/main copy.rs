use iced::{
    button, Align, Button, Column, Element, Sandbox, Settings, Text,
};
use std::fs;

fn serching() {
    let paths = fs::read_dir("./").unwrap();  
    println!("{:#?}", paths);
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}


pub fn main() -> iced::Result{
    FileSearch::run(Settings::default())
}

#[derive(Debug, Default)]
struct FileSearch {
    open_burron: button::State,
    close_button: button::State,
    file_num: u32,
}


#[derive(Debug, Clone, Copy)]
enum Message{
    OpenPressed,
    ClosePressed,
}

impl Sandbox for FileSearch {
    type Message = Message;

    fn new() -> Self{
        Self::default()
    }

    fn title(&self) -> String{
        String::from("File serch")
    }

    fn update(&mut self, message: Message){
        match message{
            Message::OpenPressed => {
                self.file_num += 10;
            }
            Message::ClosePressed => {
                self.file_num = 0;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.open_burron, Text::new("Open"))
                    .on_press(Message::OpenPressed),
            )
            .push(Text::new(self.file_num.to_string()).size(50))
            .push(
                Button::new(&mut self.close_button, Text::new("Close"))
                    .on_press(Message::ClosePressed),
            )
            .into()
    }

}
