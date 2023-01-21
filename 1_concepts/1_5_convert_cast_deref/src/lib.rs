#![allow(unused)]
use std::ops::Deref;

use email_address::*;
use rand::Rng;

#[derive(PartialEq, Debug, Clone)]
pub struct EmailString(String);

impl From<&str> for EmailString {
    fn from(str: &str) -> Self {
        if !EmailAddress::is_valid(str) {
            panic!("invalid email address")
        } else {
            EmailString(str.to_string())
        }
    }
}

impl Into<String> for EmailString {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Random<T> {
    body: [T; 3],
}
impl<T> Random<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Random {
            body: [a, b, c],
        }
    }
}

impl <T>From<[T; 3]> for Random<T> {
    fn from(arr: [T; 3]) -> Self {
        Random { body: arr }
    }
}

impl <T: Copy> TryFrom<Vec<T>> for Random<T> {
    type Error = &'static str;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        if v.len() < 3 {
            Err("insufficient vector length")
        } else {
            Ok(Random { body: [v[0],v[1],v[2]] })
        }
    }
}

impl<T> Deref for Random<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let mut rng = rand::thread_rng();
        let i: usize = rng.gen_range(0..3);
        &self.body[i]
    }
}

impl<T> AsRef<T> for Random<T> {
    fn as_ref(&self) -> &T {
        let mut rng = rand::thread_rng();
        let i: usize = rng.gen_range(0..3);

        &self.body[i]
    }
}

mod oth_scope {
    use email_address::*;

    pub struct EmailString(String);

    impl TryFrom<&str> for EmailString {
        type Error = &'static str;
        fn try_from(str: &str) -> Result<Self, Self::Error> {
            if !EmailAddress::is_valid(str) {
                Err("unvalid email address")
            } else {
                Ok(EmailString(str.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_strings() {
        let email = EmailString::from("john@example.uk");

        let email_2: String = email.into();

        let email_3 = EmailString::try_from("john@example.uk").unwrap();

        assert_eq!(email_2, String::from("john@example.uk"));

        assert_eq!(email_3, EmailString(String::from("john@example.uk")));
    }

    #[test]
    fn test_to_as_ref() {
        let email = EmailString::from("john@example.uk");

        assert_eq!(email.as_ref(), "john@example.uk");
    }
    
}
