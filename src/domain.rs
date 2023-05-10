use unicode_segmentation::UnicodeSegmentation;
const MAX_SUSCRIBER_NAME_LENGHT: usize = 256;

pub struct NewSubscriber {
    pub email: String, 
    pub name: SubscriberName
}

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

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }
}


impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
