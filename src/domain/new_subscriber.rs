use super::{SubscriberName, SubscriberEmail};

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail, 
    pub name: SubscriberName
}
