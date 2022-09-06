use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
    .from("from <from@example.com>".parse().unwrap())
    .to("to <to@example.com>".parse().unwrap())
    .subject("Rustでメール送信！")
    .body(String::from("RustからSendGridのSMTPで送ってるよ！"))
    .unwrap();

    let creds = Credentials::new("apikey".to_string(), "SG.xxxxx".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.sendgrid.net")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("送信成功"),
        Err(e) => panic!("送信失敗: {:?}", e),
    }
}
