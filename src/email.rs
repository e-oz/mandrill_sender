use rustc_serialize::json::{self, Json};
use reqwest::Client;
use std::io::Read;
use std::default::Default;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Recipient
{
  pub email: String,
  pub name: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
#[allow(dead_code)]
pub struct Attachment
{
  pub _type: String,
  pub name: String,
  pub content: String,
}

#[derive(Default, RustcDecodable, RustcEncodable, Clone)]
pub struct Email
{
  pub html: Option<String>,
  pub text: Option<String>,
  pub subject: Option<String>,
  pub from_email: Option<String>,
  pub from_name: Option<String>,
  pub to: Option<Vec<Recipient>>,
  pub important: bool,
  pub bcc_address: Option<String>,
  pub tags: Option<Vec<String>>,
  pub global_merge_vars: Option<Vec<TemplateVar>>,
  pub google_analytics_domains: Option<Vec<String>>,
  pub google_analytics_campaign: Option<String>,
  pub attachments: Option<Vec<Attachment>>,
}

impl Email {
  pub fn new(from_email: &str, to: &str, subject: &str, message: &str) -> Email {
    let mut m: Email = Default::default();
    m.from_email = Some(from_email.clone().to_owned());
    m.html = Some(message.clone().to_owned());
    m.subject = Some(subject.clone().to_owned());
    m.to = Some(vec![Recipient{email: to.clone().to_owned(), name: None}]);
    m
  }
}

#[derive(Clone)]
pub struct Sender
{
  key: String,
  api_url: String,
}

#[derive(RustcEncodable)]
struct MandrillMessage
{
  key: String,
  message: Email
}

#[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
pub struct TemplateVar
{
  pub name: String,
  pub content: String
}

#[derive(RustcEncodable)]
struct MandrillTemplateMessage
{
  key: String,
  template_name: String,
  template_content: Vec<TemplateVar>,
  message: Email,
}

impl Sender {
  pub fn new(key: String) -> Sender {
    Sender { api_url: "https://mandrillapp.com/api/1.0".to_owned(), key: key }
  }

  pub fn send(&self, email: &Email) -> bool {
    let client = Client::new().unwrap();
    let msg = MandrillMessage { key: self.key.clone().to_owned(), message: email.clone() };
    let body = json::encode(&msg).unwrap_or("".to_owned()).replace("\"_type\"", "\"type\"");
    match client.post(&format!("{}/messages/send.json", self.api_url)).body(body).send() {
      Ok(mut res) => {
        if res.status().is_success() {
          let mut response_body = &mut "".to_owned();
          match res.read_to_string(response_body) {
            Ok(_) => {
              match Json::from_str(response_body) {
                Ok(j) => {
                  match j.as_object() {
                    Some(r) => {
                      match r.get::<String>(&"status".to_owned()) {
                        Some(ref s) => {
                          match s.as_string() {
                            Some(status) => {
                              status != "error"
                            },
                            None => false
                          }
                        },
                        None => true
                      }
                    },
                    None => {
                      match j.as_array() {
                        Some(a) => {
                          match a.get(0) {
                            Some(j) => {
                              match j.as_object() {
                                Some(r) => {
                                  match r.get::<String>(&"status".to_owned()) {
                                    Some(ref s) => {
                                      match s.as_string() {
                                        Some(status) => {
                                          status != "error"
                                        },
                                        None => false
                                      }
                                    },
                                    None => true
                                  }
                                },
                                None => true
                              }
                            },
                            None => true
                          }
                        },
                        None => true
                      }
                    }
                  }
                },
                Err(_) => true
              }
            },
            Err(_) => false
          }
        } else {
          let mut response_body = &mut "".to_owned();
          match res.read_to_string(response_body) {
            Ok(_) => (), Err(_) => ()
          }
          println!("mandrill error: {:?} \n\n {}", res.headers(), response_body);
          false
        }
      },
      Err(_) => {
        false
      }
    }
  }

  pub fn send_template(&self, template_slug: &str, to: &str, template_vars: Vec<TemplateVar>, from_email: Option<String>, from_name: Option<String>, subject: Option<String>) -> bool {
    let client = Client::new().unwrap();
    let mut email: Email = Default::default();
    email.to = Some(vec![Recipient{email: to.clone().to_owned(), name: None}]);
    email.global_merge_vars = Some(template_vars.clone());
    if let Some(f_email) = from_email {
      email.from_email = Some(f_email);
    }
    if let Some(f_name) = from_name {
      email.from_name = Some(f_name);
    }
    if let Some(subj) = subject {
      email.subject = Some(subj);
    }
    let msg = MandrillTemplateMessage {
      key: self.key.clone().to_owned(),
      template_name: template_slug.clone().to_owned(),
      template_content: template_vars,
      message: email,
    };
    let body = json::encode(&msg).unwrap_or("".to_owned());
    match client.post(&format!("{}/messages/send-template.json", self.api_url)).body(body).send() {
      Ok(mut res) => {
        if res.status().is_success() {
          let mut response_body = &mut "".to_owned();
          match res.read_to_string(response_body) {
            Ok(_) => {
              match Json::from_str(response_body) {
                Ok(j) => {
                  match j.as_object() {
                    Some(r) => {
                      match r.get::<String>(&"status".to_owned()) {
                        Some(ref s) => {
                          match s.as_string() {
                            Some(status) => {
                              status != "error"
                            },
                            None => false
                          }
                        },
                        None => true
                      }
                    },
                    None => {
                      match j.as_array() {
                        Some(a) => {
                          match a.get(0) {
                            Some(j) => {
                              match j.as_object() {
                                Some(r) => {
                                  match r.get::<String>(&"status".to_owned()) {
                                    Some(ref s) => {
                                      match s.as_string() {
                                        Some(status) => {
                                          status != "error"
                                        },
                                        None => false
                                      }
                                    },
                                    None => true
                                  }
                                },
                                None => true
                              }
                            },
                            None => true
                          }
                        },
                        None => true
                      }
                    }
                  }
                },
                Err(_) => true
              }
            },
            Err(_) => false
          }
        } else {
          let mut response_body = &mut "".to_owned();
          match res.read_to_string(response_body) {
            Ok(_) => (), Err(_) => ()
          }
          println!("mandrill error: {:?} \n\n {}", res.headers(), response_body);
          false
        }
      },
      Err(_) => {
        false
      }
    }
  }
}

