use std::sync::{Arc, Mutex};
use crate::Error::NotFound;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::{Error as RocketError, State};

#[macro_use]
extern crate rocket;








#[get("/")]
async fn hello(server: &State<Arc<Mutex<ServerState>>>) -> &'static str {
    let mut server = server.lock().unwrap(); // Accès exclusif à l'état du serveur
    server.clocked_in = true;
    "Hello, world!"
}

#[launch]
fn rocket(serverState: Arc<Mutex<ServerState>>) -> _ {
    rocket::build().manage(server).mount("/", routes![hello])
}

#[derive(Default)]
struct ServerState {
    clocked_in: bool,
}

async fn start_server(server_state: Arc<Mutex<ServerState>>) -> Result<Client, RocketError> {
    let server = rocket(server_state);

    let client = Client::tracked(server).await;
    client
}

#[derive(Debug)]
enum Error {
    NotFound,
}

async fn clock_in(client: Client) -> Result<(), Error> {
    let response = client.get("/").dispatch().await;
    if response.status() == Status::Ok {
        return Ok(());
    }

    return Ok(());
    // Err(NotFound)
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::{clock_in, start_server, ServerState};

    #[tokio::test]
    async fn should_client_can_clock_in_server() {
        let server = Arc::new(Mutex::new(ServerState::default())); // État partagé entre les requêtes
        let client = start_server(server.clone()).await.unwrap();

        // assert_eq!(server.clocked_in, false);

        // apres avoir clock in
        assert_eq!(server.lock().unwrap().clocked_in, true);

        // clock_in(client).await.unwrap();
    }

    #[tokio::test]
    async fn should_not_clock_in_server_when_not_present() {}
}
