use std::{fs, path::PathBuf};

use clap::Parser;
use dotenv::dotenv;
use rocket::{
    data::{Data, ToByteUnit},
    http::uri::Absolute,
    serde::json::Json,
    tokio::fs::File,
};

use crate::{
    error::Error,
    jwt::Token,
    paste::PasteId,
    user::{LoginForm, PassWordChangeForm, RegUserForm, User, UserToken},
};

mod error;
mod jwt;
mod paste;
mod user;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, value_name = "HOST", default_value = "http://127.0.0.1")]
    pub host: String,
    #[arg(short, long, value_name = "LENGTH", default_value = "64")]
    pub id_length: usize,
    #[arg(short, long, value_name = "DIR", default_value = "upload")]
    pub upload: PathBuf,
}

#[rocket::get("/")]
fn help() -> &'static str {
    "USAGE
      POST /
          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<format>/<id>?<uid>
          retrieves the content for the paste with id `<id>` (optionally for user
          `<uid>`) formatted as <format>

      GET /<id>?<uid>
          retrieves the content for the paste with id `<id>` (optionally for user
          `<uid>`)

      DELETE /<id>?<uid>
          delete paste with id `<id>` (optionally for user `<uid>`)


      GET /register
          generate login creds

      POST /change-password
          change password

      POST /login
          login (after login pastes can not be deleted without login)"
}

#[rocket::post("/", data = "<paste>")]
async fn upload(paste: Data<'_>, token: Option<Token<UserToken>>) -> Result<String, Error> {
    let args = Args::parse();
    let owner = if let Some(token) = token {
        Some(User::from(token.decode()?).username)
    } else {
        None
    };
    let id = PasteId::new(args.id_length);
    paste
        .open(128.kibibytes())
        .into_file(id.file_path(&owner))
        .await?;
    let host =
        Absolute::parse(args.host.as_str()).unwrap_or(Absolute::parse("http://127.0.0.1").unwrap());
    Ok(rocket::uri!(host, retrieve(id, owner)).to_string())
}

#[rocket::delete("/<id>?<my>")]
async fn drop(id: PasteId, my: Option<bool>, token: Option<Token<UserToken>>) -> Result<(), Error> {
    let my = my.unwrap_or(false);
    let owner = if my {
        if let Some(token) = token {
            Some(User::from(token.decode()?).username)
        } else {
            None
        }
    } else {
        None
    };
    Ok(fs::remove_file(id.file_path(&owner))?)
}

#[rocket::get("/<id>?<owner>")]
async fn retrieve(id: PasteId, owner: Option<String>) -> Option<File> {
    File::open(id.file_path(&owner)).await.ok()
}

#[rocket::post("/register", data = "<form_data>")]
async fn register(form_data: Json<RegUserForm>) -> Result<(), Error> {
    let Json(data) = form_data;
    User::register(data)?;
    Ok(())
}

#[rocket::post("/change-password", data = "<form_data>")]
async fn change_password(
    token: Token<UserToken>,
    form_data: Json<PassWordChangeForm>,
) -> Result<(), Error> {
    let mut user = User::from(token.decode()?);
    let Json(data) = form_data;
    user.change_password(data)?;
    Ok(())
}

#[rocket::post("/login", data = "<form_data>")]
async fn login(form_data: Json<LoginForm>) -> Result<Json<Token<UserToken>>, Error> {
    let Json(data) = form_data;
    let user_token = UserToken::from(User::login(data)?);
    let token = Token::encode(user_token)?;
    Ok(Json(token))
}

#[rocket::launch]
fn launcher() -> _ {
    // load env
    dotenv().ok();
    // parse args and ensure upload dir existence
    let arg = Args::parse();
    let mut upload_dir = arg.upload;
    upload_dir.push(User::anonymous().username);
    if !upload_dir.exists() {
        let _ = fs::create_dir_all(upload_dir);
    }
    // launch rocket
    rocket::build().mount(
        "/",
        rocket::routes![
            help,
            upload,
            retrieve,
            drop,
            register,
            login,
            change_password,
        ],
    )
}
