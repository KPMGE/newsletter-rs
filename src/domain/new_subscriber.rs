use super::SubscriberName;

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: String, 
    pub name: SubscriberName
}
