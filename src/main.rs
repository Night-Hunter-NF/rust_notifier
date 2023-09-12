use windows_notifier::{
    tags::{image::Image, text::Text},
    Toast,
};

#[tokio::main]
async fn main() {
    let mut toast = Toast::new().unwrap();
    toast.title("Hello, world!").unwrap();

    toast.add_text(Text::new("Jill Bender")).unwrap();
    toast.add_text(Text::new("Check out where we camped last weekend! It was incredible, wish you could have come on the backpacking trip!")).unwrap();

    toast
        .add_image(
            Image::new("https://unsplash.it/64?image=1027")
                .set_placement(windows_notifier::tags::image::Placement::AppLogoOverride)
                .set_hint_crop(windows_notifier::tags::image::Crop::Circle),
        )
        .unwrap();

    toast
        .add_image(
            Image::new("https://unsplash.it/360/180?image=1043")
                .set_placement(windows_notifier::tags::image::Placement::Hero),
        )
        .unwrap();

    toast
        .add_input(windows_notifier::tags::input::Input::new_text(
            "textbox",
            Some("reply"),
        ))
        .unwrap();

    toast
        .add_action(
            windows_notifier::tags::action::Action::new(
                "Send".into(),
                "action=reply&amp;threadId=92187".into(),
            )
            .image_uri("".to_string())
            .hint_input_id("textbox".to_string())
            .activation_type(windows_notifier::tags::action::ActivationType::Background),
        )
        .unwrap();

    toast.show().unwrap();
}
