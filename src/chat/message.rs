use database::Message;
use log::{debug, error};

pub(crate) trait SocketSendable {
    fn room_name(&self) -> Option<String>;
}

impl SocketSendable for Message {
    fn room_name(&self) -> Option<String> {
        debug!("in room_name");
        match (self.to_user.clone(), self.to_group.clone()) {
            (None, None) => {
                error!(
                    "both `to_user` and `to_group` were null, from: {}",
                    self.from
                );
                None
            }
            (None, Some(others)) => {
                let room_name = Message::group_to_string(others);
                Some(room_name)
            }
            (Some(other), None) => {
                let (first, second) = (
                    self.from.clone().min(other.clone()),
                    self.from.clone().max(other.clone()),
                );
                Some(
                    first
                        .to_string()
                        .chars()
                        .zip(second.to_string().chars())
                        .flat_map(|(l, r)| vec![l, r])
                        .collect(),
                )
            }
            (Some(_), Some(_)) => {
                error!("both `to_user` and `to_group` are `Some`");
                None
            }
        }
    }
}
