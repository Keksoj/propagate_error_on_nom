mod lib;

use lib::user::User;

fn main() -> anyhow::Result<()> {

    let user = User::new("Spongebob", "HeyPatrick");

    println!("{}", user.to_serialized_string().unwrap());

    let random_users = User::create_random_users(5);
    println!("Some random users:\n{}", User::serialize_users(&random_users)?);

    let bad_string = "{\"username\":345,\"password\":\"HeyPatrick\"}";

    println!(
        "Trying to deserialize this:{}\nyields this error: {:?}",
        bad_string,
        serde_json::from_str::<User>(&bad_string)
    );

    Ok(())
}
