#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if validator::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::{assert_err, assert_ok};
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;

    #[test]
    fn empty_string_is_invalid() {
        let test_email = "".to_string();
        assert_err!(SubscriberEmail::parse(test_email));
    }

    #[test]
    fn email_missing_at_symbol_is_invalid() {
        let test_email = "gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(test_email));
    }

    #[test]
    fn email_missing_subject_is_invalid() {
        let test_email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(test_email));
    }

    #[test]
    fn valid_email_is_parsed_correctly() {
        let test_email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(test_email));
    }
} 
