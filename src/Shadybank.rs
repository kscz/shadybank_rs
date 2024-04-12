extern crate reqwest;

use reqwest::blocking::Client as ReqwestClient;

pub struct Client {
    reqwest_client: ReqwestClient,
    b_token: Option<String>
}

#[derive(Debug)]
pub enum Error {
    NotLoggedIn,
    RequestFailure,
    ReqwestError(reqwest::Error),
    Unknown
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

pub enum TrackData {
    MagStripe(String),
    Track1(String),
    Track2(String)
}

impl Client {
    pub fn new() -> Client {
        Client {
            reqwest_client: reqwest::blocking::Client::new(),
            b_token: None
        }
    }

    pub fn login(&mut self,  account_id: &str, password: &str) -> Result<(), Error> {
        let params = [("account_id", account_id.clone()), ("password", password.clone()), ("type", "password")];
        let resp = self.reqwest_client.post("https://bucks.shady.tel/api/login")
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
            Some(token) => self.reqwest_client.post("https://bucks.shady.tel/api/capture")
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

    pub fn authorize(&self, magstripe: &str, amount: i32) -> Result<String, Error> {
        let params = [("magstripe", magstripe.clone()), ("amount", &amount.to_string())];

        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post("https://bucks.shady.tel/api/authorize")
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

    pub fn credit(&self, magstripe: &str, amount: i32) -> Result<(), Error> {
        let params = [("magstripe", magstripe.clone()), ("amount", &amount.to_string())];
        let resp = match &self.b_token {
            None => return Err(Error::NotLoggedIn),
            Some(token) => self.reqwest_client.post("https://bucks.shady.tel/api/credit")
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
            Some(token) => self.reqwest_client.post("https://bucks.shady.tel/api/void")
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
            Some(token) => self.reqwest_client.post("https://bucks.shady.tel/api/logout")
                .bearer_auth(token.clone())
                .send()?
        };

        if !resp.status().is_success() {
            println!("Logout request not successful: {:?}", resp);
            println!("{:?}", resp.text()?);
            Err(Error::RequestFailure)
        } else {
            self.b_token = None;
            Ok(())
        }
    }
}
