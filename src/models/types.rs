use chrono::{DateTime, Utc};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::{string, time::SystemTime};
pub struct Email(String);

impl Email {
    pub fn new(s: String) -> Result<Self, String> {
        if !s.contains('@') {
            return Err("invalid email".to_string());
        }

        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Device {
    access_token: String,
    refresh_token: String,
}

impl Device {
    pub fn validate_access(&self, other_token: String) -> Result<bool, String> {
        if other_token == self.access_token {
            Ok((true))
        } else {
            Err("invalid token".to_string())
        }
    }

    pub fn new_access_token(&mut self, mac_adress: String) -> String {
        let mut new_token = String::new();
        let current_time = SystemTime::now();
        let dt: DateTime<Utc> = current_time.clone().into();
        let mut current_date_timed = dt.format("%d-%b-%Y %H:%M:%S %P %z");
        new_token.push_str(&format!("{}|{}|", mac_adress, current_date_timed));
        for n in 0..11 {
            let char_type = rand::random_bool(0.5 / 1.0);
            if char_type {
                new_token.push_str(&rand::random_range(0..10).to_string())
            } else {
                new_token.push(rand::random_range(b'A'..=b'Z') as char);
            }
        }
        self.access_token = new_token.to_string();
        new_token
    }

    pub fn new_refresh_token(mut self, mac_adress: String) -> String {
        let mut new_token = String::new();
        let current_time = SystemTime::now();
        let dt: DateTime<Utc> = current_time.clone().into();
        let mut current_date_timed = dt.format("%d-%b-%Y %H:%M:%S %P %z");
        new_token.push_str(&format!("{}|{}|", mac_adress, current_date_timed));
        let lenght = rand::random_range(0..21);
        for n in 0..lenght {
            let char_type = rand::random_bool(0.5 / 1.0);
            if char_type {
                new_token.push_str(&rand::random_range(0..10).to_string())
            } else {
                new_token.push(rand::random_range(b'A'..=b'Z') as char);
            }
        }
        self.refresh_token = new_token.to_string();
        new_token
    }

    pub fn refresh_tokens(
        mut self,
        other_token: String,
        other_refresh_token: String,
        mac_adress: String,
    ) -> Result<(String, String), String> {
        if other_refresh_token == self.refresh_token {
            Ok((
                self.new_access_token(mac_adress.clone()),
                self.new_refresh_token(mac_adress.clone()),
            ))
        } else {
            Err("invalid token".to_string())
        }
    }
}

pub struct Password {
    pub password_hash: String,
}

impl Password {
    pub fn new(new_pw: String) -> Self {
        let hash = Sha256::digest(new_pw.as_bytes());
        let hash = hex::encode(hash);
        Self {
            password_hash: (hash),
        }
    }

    pub fn verify(&self, pw: String) -> bool {
        let hash = Sha256::digest(pw.as_bytes());
        let hash = hex::encode(hash);
        hash == self.password_hash
    }
}
