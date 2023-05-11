use unicode_segmentation::UnicodeSegmentation;
const MAX_SUSCRIBER_NAME_LENGHT: usize = 256;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > MAX_SUSCRIBER_NAME_LENGHT;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '}', '{'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl ToString for SubscriberName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl AsMut<str> for SubscriberName {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valiid() {
        let test_name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(test_name));
    }

    #[test]
    fn a_grapheme_longer_than_256_is_invaliid() {
        let test_name = "a".repeat(257);
        assert_err!(SubscriberName::parse(test_name));
    }

    #[test]
    fn whitespace_only_names_are_invalid() {
        let test_name = " ".repeat(10);
        assert_err!(SubscriberName::parse(test_name));
    }

    #[test]
    fn empty_string_is_invalid() {
        let test_name = "".to_string();
        assert_err!(SubscriberName::parse(test_name));
    }

    #[test]
    fn name_containing_invalid_characters_are_invalid() {
        for c in &['/', '(', ')', '"', '<', '>', '\\', '}', '{'] {
            let name = c.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn valid_name_is_parsed_properly() {
        let valid_name = "Kevin".to_string();
        assert_ok!(SubscriberName::parse(valid_name));
    }
}
