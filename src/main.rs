use rocket::http::Status;
use rocket::{fs::NamedFile, get};
use rocket::{launch, routes};
use std::path::{Path, PathBuf};

#[get("/")]
async fn index() -> Result<NamedFile, Status> {
    NamedFile::open(Path::new("client/index.html"))
        .await
        .map_err(|_| Status::NotFound)
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, Status> {
    NamedFile::open(Path::new("client/").join(file))
        .await
        .map_err(|_| Status::NotFound)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
