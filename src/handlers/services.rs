use axum::{Json, extract};
use crate::human::Human;

pub struct Service {}

impl Service {

    pub async fn get_stuff() -> Json<Human> {
        let env_test = std::env::var("VARIABEL").expect("MAILCOACH_API_TOKEN must be set.");
        println!("{:?}", env_test);

        Json(Human{name:String::from("abbe"), age: 2})
    }

    pub async fn post_stuff(extract::Json(payload): extract::Json<Human>)  {
        println!("{:?}", payload);
    }
}
