use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    message::{header, MultiPart, SinglePart},
    SmtpTransport, Message, Transport,
};

fn main() {
    let html = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
    <html xmlns="http://www.w3.org/1999/xhtml">
      <head>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1" /><!--[if !mso]><!-->
        <meta http-equiv="X-UA-Compatible" content="IE=Edge" /><!--<![endif]-->
      </head>
      <body>
        <div style="display: flex; flex-direction: column; align-items: center;">
            <h2>RustからSendGridのSMTPで送ってるよ！</h2>
            <h4>HTMLパート</h4>
        </div>
      </body>
    </html>"#;

    let email = Message::builder()
    .from("from <from@example.com>".parse().unwrap())
    .to("to <to@example.com>".parse().unwrap())
    .subject("Rustでマルチパートメール送信！")
    .multipart(
        MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(String::from("テキストパート")),
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(String::from(html)),
            )
    )
    .unwrap();

    let creds = Credentials::new("apikey".to_string(), "SG.xxxxx".to_string());

    let mailer = SmtpTransport::relay("smtp.sendgrid.net")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("送信成功"),
        Err(e) => panic!("送信失敗: {:?}", e),
    }
}
