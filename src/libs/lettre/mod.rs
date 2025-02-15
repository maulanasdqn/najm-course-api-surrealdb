use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn send_email(
	to: &str,
	subject: &str,
	body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let sender_email = env::var("SMTP_EMAIL")?.to_string();
	let sender_name = env::var("SMTP_NAME")?.to_string();
	let sender_password = env::var("SMTP_PASSWORD")?.to_string();
	let recipient_email = to;

	let email = Message::builder()
		.from(Mailbox::new(
			Some(sender_name.replace("-", " ")),
			sender_email.parse()?,
		))
		.to(recipient_email.parse()?)
		.subject(subject)
		.body(body.to_string())?;

	let smtp_credentials =
		Credentials::new(sender_email, sender_password.replace("-", " "));

	let mailer = SmtpTransport::relay("smtp.gmail.com")?
		.credentials(smtp_credentials)
		.build();

	match mailer.send(&email) {
		Ok(_) => {
			println!("Email sent successfully to {}", to);
			Ok(())
		}
		Err(e) => {
			println!("Failed to send email: {}", e);
			Err(Box::new(e))
		}
	}
}
