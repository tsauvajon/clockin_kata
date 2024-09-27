use rocket::Error as RocketError;
use rocket::http::Status;
use rocket::local::asynchronous::{Client};
use crate::Error::NotFound;

#[macro_use] extern crate rocket;

#[get("/")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

async fn start_server() -> Result<Client, RocketError> {
    let server = rocket();

    let client = Client::tracked(server).await;
    client
}

#[derive(Debug)]
enum Error {
    NotFound,
}

async fn clock_in(client: Client) -> Result<(), Error> {
    // let response = client.get("/clock-in").dispatch().await;
    // if response.status() == Status::Ok {
    //     return Ok(());
    // }

    return Ok(());
    // Err(NotFound)

}

#[cfg(test)]
mod tests {
    use crate::{clock_in, start_server};


    #[tokio::test]
    async fn should_client_can_clock_in_server() {
        let client = start_server().await.unwrap();

        clock_in(client).await.unwrap();
    }

    #[tokio::test]
    async fn should_not_clock_in_server_when_not_present() {

    }
}