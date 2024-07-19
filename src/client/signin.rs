use super::openiap::{Envelope, SigninRequest};

#[allow(dead_code)]
impl SigninRequest {
    pub fn with_userpass(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            ping: false,
            ..Default::default()
        }
    }
    pub fn with_jwt(jwt: &str) -> Self {
        Self {
            jwt: jwt.to_string(),
            ..Default::default()
        }
    }
}

impl SigninRequest {
    pub fn to_envelope(&self) -> Envelope {
        let any_message = prost_types::Any {
            type_url: "type.googleapis.com/openiap.SigninRequest".to_string(),
            value: {
                let mut buf = Vec::new();
                prost::Message::encode(self, &mut buf).unwrap();
                buf
            },
        };
        Envelope {
            command: "signin".into(),
            data: Some(any_message.clone()),
            ..Default::default() 
        }
    }
}

#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin + Default + Clone + PartialEq>() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_type() {
        is_normal::<SigninRequest>();
    }
}