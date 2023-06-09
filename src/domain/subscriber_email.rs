#[derive(Debug)]
pub struct SubscriberEmail(pub String);

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

impl AsMut<str> for SubscriberEmail {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::empty_shrinker;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            empty_shrinker()
        }
    }

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

    #[quickcheck_macros::quickcheck]
    fn valid_email_is_parsed_correctly(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
