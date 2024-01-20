use log::error;
use serde::Deserialize;

use crate::utils::UuidWrapper;

// #[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
// pub struct User {
//     pub id: UuidWrapper,
// }

#[derive(Deserialize)]
pub struct Message {
    pub from: UuidWrapper,
    pub to_user: Option<UuidWrapper>,
    pub to_group: Option<Vec<UuidWrapper>>,
    pub text: String,
}

impl Message {
    pub fn room_name(&self) -> Option<String> {
        match (self.to_user, self.to_group.clone()) {
            (None, None) => {
                error!(
                    "both `to_user` and `to_group` were null, from: {}",
                    self.from
                );
                None
            }
            (None, Some(others)) => {
                let mut others = others.clone();
                others.sort();
                let uuid_len = others[0].to_string().len();
                let mut room_name = String::with_capacity(uuid_len);
                for i in 0..uuid_len {
                    for user in &mut *others {
                        room_name += user.to_string().as_bytes()[i].to_string().as_str();
                    }
                }
                Some(room_name)
            }
            (Some(other), None) => {
                let (first, second) = (self.from.min(other), self.from.max(other));
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
