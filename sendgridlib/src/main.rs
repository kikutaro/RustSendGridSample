use sendgrid::v3::*;

fn main() {
    let p = Personalization::new(Email::new("to@example.com"));
    let message = Message::new(Email::new("from@example.com"))
        .set_subject("Rustでメール送信！")
        .add_content(
            Content::new()
                .set_content_type("text/plain")
                .set_value("sendgrid-rsを使ってWeb APIから送ってるよ！"),
        )
        .add_personalization(p);

    let api_key = "SG.xxxxx".to_string();
    let sender = Sender::new(api_key);
    let code = sender.send(&m);
    println!("{:?}", code);
}
