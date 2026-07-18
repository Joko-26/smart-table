use rand::Rng;

pub struct Email(String);

impl Email {
    pub fn new(email_str: String) -> Result<self, InvalidEmail> {
        if !email_str.contain("@") {
            return Err(InvalidEmail);
        }
        Ok(self(email_str))
    }

    pub fn as_str() -> &str {
        &self.0
    }
}

pub struct Device {
    access_token: String,
    refresh_token: String,
}

impl Device {
    pub fn validate_access(other_token: String, address:String, mac_address: String) -> Result<Ok, InvalidToken> {
        if other_token == self.access_token{ Ok(()) } else { Err(InvalidToken)}
    }

    pub fn new_access_token() {
        let mut rng = rand::thread_rng();
        let new_token = "";
        let lenght = rnd.gen_range(0, 16);
        for n in 0..lenght {
            let char_type = rnd.gen_range(0, 2);
            if char_type == 0 {
                new_token.push_str(rnd.gen_range(0, 10))
            } else {
                new_token.push_str(rng.gen_range(b'A', b'Z') as char)
            }
        }
        self.access_token = new_token;
        new_access_token
    }

    pub fn new_refresh_token() {
        let mut rng = rand::thread_rng();
        let new_token = "";
        let lenght = rnd.gen_range(0, 16);
        for n in 0..lenght {
            let char_type = rnd.gen_range(0, 2);
            if char_type == 0 {
                new_token.push_str(rnd.gen_range(0, 10))
            } else {
                new_token.push_str(rng.gen_range(b'A', b'Z') as char)
            }
        }
        self.access_token = new_token;
        new_access_token
    }

    pub fn refresh_tokens(other_token: String, other_refresh_token: String) {
        if other_refresh_token == self.refresh_token {}
    }
}
