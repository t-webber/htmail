use std::collections::HashMap;
use std::{fs, io};

pub const PROFILE_DB: &str = "db/profiles.json";
pub const RECIPIENT_DB: &str = "db/recipients.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MailCredentials {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn read_profiles() -> HashMap<String, MailCredentials> {
    fs::File::open(PROFILE_DB).map_or(HashMap::new(), |file| {
        let reader = io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
    })
}

fn write_profiles(data: &HashMap<String, MailCredentials>) -> io::Result<()> {
    let file = fs::File::create(PROFILE_DB)?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer(writer, &data)?;
    Ok(())
}

pub fn read_recipients() -> HashMap<String, String> {
    fs::File::open(RECIPIENT_DB).map_or(HashMap::new(), |file| {
        let reader = io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| HashMap::new())
    })
}

fn write_recipients(data: &HashMap<String, String>) -> io::Result<()> {
    let file = fs::File::create(RECIPIENT_DB)?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer(writer, &data)?;
    Ok(())
}

pub fn create_profile(
    pseudonym: String,
    name: String,
    email: String,
    password: String,
) -> io::Result<()> {
    let mut data = read_profiles();
    data.insert(
        pseudonym,
        MailCredentials {
            name,
            email,
            password,
        },
    );
    write_profiles(&data)
}

#[allow(unused)]
pub fn delete_profile(key: &str) -> io::Result<()> {
    let mut data = read_profiles();
    data.remove(key);
    write_profiles(&data)
}

pub fn get_credentials(pseudonym: &str) -> Result<MailCredentials, String> {
    let data = read_profiles();
    data.get(pseudonym)
        .map(|x| MailCredentials {
            name: x.name.clone(),
            email: x.email.clone(),
            password: x.password.clone(),
        })
        .ok_or_else(|| "Profile not found in database. Consider adding it..".to_owned())
}

pub fn create_recipient(name: String, email: String) -> io::Result<()> {
    let mut data = read_recipients();
    data.insert(name, email);
    write_recipients(&data)
}

#[allow(unused)]
pub fn delete_recipient(key: &str) -> io::Result<()> {
    let mut data = read_recipients();
    data.remove(key);
    write_recipients(&data)
}
