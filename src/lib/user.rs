use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new<T>(username: T, password: T) -> Self
    where
        T: ToString,
    {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn serialize_users(users: &Vec<Self>) -> Result<String> {
        let mut serialized_users = String::new();
        for user in users.iter() {
            serialized_users.push_str(&user.to_serialized_string()?);
            // add a newline \n between each user and at the end
            serialized_users.push('\n');
        }
        Ok(serialized_users)
    }

    pub fn create_random_users(number: usize) -> Vec<Self> {
        let mut users = Vec::new();
        for _ in 0..number {
            users.push(User {
                username: rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect(),
                password: rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect(),
            });
        }
        users
    }

    pub fn to_serialized_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_to_and_from_string_works() {
        let user = User::new("Spongebob", "HeyPatrick");
        let stringified_user = user.to_serialized_string().unwrap();
        assert_eq!(
            serde_json::from_str::<User>(&stringified_user).unwrap(),
            user
        );
    }

    #[test]
    fn deserialize_error_works() {
        let bad_string = "{\"username\":345,\"password\":\"HeyPatric\"}";
        assert!(serde_json::from_str::<User>(&bad_string).is_err())
    }
}
