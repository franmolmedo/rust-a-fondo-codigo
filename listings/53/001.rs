use atlas_client::{Client, UserId};

let id = UserId::parse("usr_42")?;
let user = client.user(id).await?;
println!("{}", user.display_name());
