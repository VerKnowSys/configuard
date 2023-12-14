use configuard::*;
use rocket::{catch, catchers, request::Request, routes};


#[catch(500)]
fn internal_error() -> &'static str {
    "Internal Error."
}


#[catch(404)]
fn not_found(_req: &Request) -> String {
    new_decoy()
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = config();
    validate_config(&config);

    let server_uuid = config.uuid;
    rocket::build()
        .mount(
            &format!("/{server_uuid}/wireguard/workstation/"),
            routes![workstations::new],
        )
        .register("/", catchers![internal_error, not_found])
        .launch()
        .await?;
    Ok(())
}
