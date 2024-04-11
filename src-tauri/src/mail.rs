use crate::db;
use lettre::message;
use lettre::transport::smtp::authentication;
use lettre::transport::smtp::response;
use lettre::Transport;

fn now() -> String {
    let now = chrono::Local::now();
    now.format("%H:%M:%S").to_string()
}

fn is_valid_email(email: &str) -> Result<(), String> {
    if email.contains('@')
        && email.contains('.')
        && email.len() > 4
        && email.chars().filter(|ch| *ch == '@').count() == 1
        && email.chars().last().unwrap_or('.').is_ascii_alphabetic()
    {
        Ok(())
    } else {
        Err(format!("Email {email} is invalid"))
    }
}

fn build_mailer(from: &str) -> Result<lettre::SmtpTransport, String> {
    let read_credentials = db::get_credentials(from)?;
    let smtp_creds =
        authentication::Credentials::new(read_credentials.email.clone(), read_credentials.password);

    Ok(lettre::SmtpTransport::relay("smtp.gmail.com")
        .map_err(|err| format!("Could not connect to gmail: {err}"))?
        .credentials(smtp_creds)
        .build())
}

fn build_email(
    from: &str,
    from_email: &str,
    to_emails: Vec<&str>,
    subject: &str,
    body: &str,
) -> Result<lettre::Message, String> {
    let mut email_msg = lettre::Message::builder();

    email_msg = email_msg.from(
        format!("{from} <{from_email}>")
            .parse::<message::Mailbox>()
            .map_err(|err| format!("Invalid from: {err}"))?,
    );

    for email in to_emails {
        email_msg = email_msg.to(email
            .parse::<message::Mailbox>()
            .map_err(|err| format!("Invalid to: {err}"))?);
    }

    let subject_part = format!("[{}] {subject}", now());

    let body_part = message::SinglePart::html(body.to_owned());

    email_msg
        .subject(subject_part)
        .multipart(message::MultiPart::mixed().singlepart(body_part))
        .map_err(|err| format!("Could not build email: {err}"))
}

pub fn send(from: &str, to: &str, subject: &str, body: &str) -> Result<response::Response, String> {
    eprintln!("Checking email {from} == ({subject}) ==> {to}\n[[[ {body} ]]]");

    let profile = db::get_credentials(from)?;
    is_valid_email(&profile.email)?;

    let to_emails = to
        .split(',')
        .flat_map(|elt| elt.split(';'))
        .map(str::trim)
        .collect::<Vec<&str>>();

    for email in &to_emails {
        is_valid_email(email)?;
    }

    (!body.is_empty())
        .then_some(())
        .ok_or_else(|| "Body found empty, but required".to_owned())?;

    eprintln!("Building email");

    let email = build_email(from, &profile.email, to_emails, subject, body)?;
    let mailer = build_mailer(from)?;

    eprintln!("Sending email");

    let result: Result<response::Response, String> = mailer
        .send(&email)
        .map_err(|err| format!("Could not send email: {err:?}"));

    eprintln!("Email sent");

    result
}
