extern crate reqwest;

use reqwest::blocking::Client as ReqwestClient;

pub struct Client {
    reqwest_client: ReqwestClient,
    host_url: String,
    b_token: Option<String>
}

#[derive(Debug)]
pub enum Error {
    NotLoggedIn,
    RequestFailure,
    ReqwestError(reqwest::Error)
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

#[derive(Debug)]
pub enum MagData {
    Stripe(String),
    Track1(String),
    Track2(String),
    PanOtp((String, String)),
    PanShotp((String, String))
}

impl Client {
    pub fn new(host_url: Option<String>) -> Client {
        let url = if let Some(url) = host_url {
            url
        } else {
            String::from("https://bucks.shady.tel")
        };
        Client {
            reqwest_client: reqwest::blocking::Client::new(),
            host_url: url,
            b_token: None
        }
    }

    pub fn login(&mut self,  account_id: &str, password: &str) -> Result<(), Error> {
        let params = [("account_id", account_id), ("password", password), ("type", "password"), ("otp", "")];
        let resp = self.reqwest_client.post(self.host_url.clone() + "/api/login")
            .form(&params)
            .send()?;

        if !resp.status().is_success() {
            println!("Login request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            self.b_token = Some(resp.text()?);
            Ok(())
        }
    }

    pub fn capture(&self, amount: i32, auth_code: &str) -> Result<(), Error> {
        let params = [("amount", amount.to_string()), ("auth_code", auth_code.to_string())];

        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post(self.host_url.clone() + "/api/capture")
                .bearer_auth(token.clone())
                .form(&params)
                .send()?
        };

        if !resp.status().is_success() {
            println!("Capture request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            Ok(())
        }
    }

    pub fn authorize(&self, card: &MagData, amount: i32) -> Result<String, Error> {
        let mut params = Vec::new();
        match card {
            MagData::Stripe(magstripe) => { params.push(("magstripe", magstripe.clone())); },
            MagData::Track1(track) => { params.push(("track1", track.clone())); },
            MagData::Track2(track) => { params.push(("track2", track.clone())); },
            MagData::PanOtp((pan, otp)) => {
                params.push(("pan", pan.clone()));
                params.push(("otp", otp.clone()));
            },
            MagData::PanShotp((pan, shotp)) => {
                params.push(("pan", pan.clone()));
                params.push(("shotp", shotp.clone()));
            }
        }
        params.push(("amount", amount.to_string()));

        println!("{:?}", params);

        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post(self.host_url.clone() + "/api/authorize")
                .bearer_auth(token.clone())
                .form(&params)
                .send()?
        };

        if !resp.status().is_success() {
            println!("Authorize request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            Ok(resp.text()?)
        }
    }

    pub fn credit(&self, card: &MagData, amount: i32) -> Result<(), Error> {
        let mut params = Vec::new();
        match card {
            MagData::Stripe(magstripe) => { params.push(("magstripe", magstripe.clone())); },
            MagData::Track1(track) => { params.push(("track1", track.clone())); },
            MagData::Track2(track) => { params.push(("track2", track.clone())); },
            MagData::PanOtp((pan, otp)) => {
                params.push(("pan", pan.clone()));
                params.push(("otp", otp.clone()));
            },
            MagData::PanShotp((pan, shotp)) => {
                params.push(("pan", pan.clone()));
                params.push(("shotp", shotp.clone()));
            }
        }
        params.push(("amount", amount.to_string()));

        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post(self.host_url.clone() + "/api/credit")
            .bearer_auth(token.clone())
            .form(&params)
            .send()?
        };

        if !resp.status().is_success() {
            println!("Credit request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            Ok(())
        }
    }

    pub fn void(&self, auth_code: &str)-> Result<(), Error>  {
        if self.b_token.is_none() {
            return Err(Error::NotLoggedIn);
        }

        let params = [("auth_code", auth_code.to_string())];
        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post(self.host_url.clone() + "/api/void")
                .bearer_auth(token.clone())
                .form(&params)
                .send()?
        };

        if !resp.status().is_success() {
            println!("Void request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            Ok(())
        }
    }

    pub fn logout(&mut self) -> Result<(), Error> {
        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post(self.host_url.clone() + "/api/logout")
                .bearer_auth(token.clone())
                .send()?
        };

        self.b_token = None;

        if !resp.status().is_success() {
            println!("Logout request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            Ok(())
        }
    }
}
