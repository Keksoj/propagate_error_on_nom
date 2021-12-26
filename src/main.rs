mod lib;

use lib::user::User;

fn main() -> anyhow::Result<()> {
    let user = User::new("Spongebob", "HeyPatrick");

    println!("{}", user.to_serialized_string().unwrap());

    let random_users = User::create_random_users(5);
    println!(
        "Some random users:\n{}",
        User::serialize_users(&random_users)?
    );

    let bad_string = "{\"username\":345,\"password\":\"HeyPatrick\"}";

    println!(
        "Trying to deserialize this:{}\nwith serde_json, yields this error: {:?}\n",
        bad_string,
        serde_json::from_str::<User>(&bad_string)
    );

    println!(
        "Trying to parse this:{}\nwith our nom-combinated parser that wraps serde_json, should propagate the same error:\n{:?}",
        bad_string,
        lib::parse_several_structs::<User>(&bad_string)
    );

    Ok(())
}
