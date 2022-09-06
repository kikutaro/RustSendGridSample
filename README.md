# RustSendGridSample
RustからSendGridを使ってメール送信するサンプルコード

## SMTP(lettre)による送信
### テキストメール

```sh
cd smtp
cargo run --bin textonly
```

### マルチパートメール

```sh
cd smtp
cargo run --bin multipart
```

## Web API(sendgrid-rs)による送信

```sh
cd sendgridlib
cargo run
```
