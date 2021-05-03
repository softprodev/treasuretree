#![allow(unused)]
#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use anyhow::{anyhow, bail, Result};
use rocket::response::{content::Html, Responder, Content};
use rocket_contrib::{templates::Template, json::Json};
use rocket_contrib::serve::StaticFiles;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::fmt;
use treasure::Treasure;
use rocket::Data;
use std::fs::{self, File, DirEntry, Metadata};
use std::io::prelude::*;
use std::io::BufReader;
use rocket::http::{RawStr, Method, ContentType};
use std::time::SystemTime;

mod api;
mod crypto;
mod treasure_qrcode;
mod treasure;

#[get("/")]
fn root_page() -> Template {
    Template::render("index", json!({}))
}

#[get("/<page>")]
fn static_page(page: String) -> Template {
    Template::render(page, json!({}))
}

#[get("/recent", )]
fn recent_page() -> Result<Template> {

    fs::create_dir_all("treasure")?;

    // This nightmare expression collects DirEntrys for every
    // thing in the directory that is a file,
    // and extracting the modify time,
    // while also bubbling any possible errors.
    // It does the "collect Iter<Item = Result> into Result<Vec>" trick.
    let mut files = fs::read_dir("treasure")?
        // Get the file metadata
        .map(|dent: Result<DirEntry, _>| {
            dent.and_then(|dent| Ok((dent.metadata()?, dent)))
        })
        // Only keep entries that are files or errors
        .filter(|dent: &Result<(Metadata, DirEntry), _>| {
            dent.as_ref().map(|(meta, _)| meta.is_file()).unwrap_or(true)
        })
        // Keep modify time for sorting
        .map(|dent: Result<(Metadata, DirEntry), _> | {
            dent.and_then(|(meta, dent)| Ok((meta.modified()?, dent)))
        })
        // Collect iter of Result into Result<Vec>,
        // and return any error.
        .collect::<Result<Vec<_>, _>>()?;

    files.sort_by_key(|&(time, _)| time);

    #[derive(Serialize)]
    struct Treasure {
        public_key: String,
        image_url: String,
        date_time: String,
    }

    let treasures = files.into_iter().take(10).map(|(time, dent)| {
        let public_key = dent.file_name().into_string().expect("utf-8");
        let image_url = format!("treasure-images/{}", public_key);
        let date_time = chrono::DateTime::<chrono::Local>::from(time);
        let date_time = date_time.to_rfc2822();
        Treasure {
            public_key,
            image_url,
            date_time,
        }
    }).collect();

    #[derive(Serialize)]
    struct TemplateData {
        treasures: Vec<Treasure>,
    }

    let data = TemplateData {
        treasures,
    };

    Ok(Template::render("recent", data))
}

/// Return an html page displaying a treasure
///
/// `public_key` is bech32 encoded.
///
/// The page includes an `img` tag with the url of the treasure image,
/// and displays the private (public) key of the treasure.
///
/// Remember to percent-decode the rawstr.
///
/// Load the template from templates/treasure/template.html.tera.
#[get("/treasure/<public_key>")]
fn treasure_page(public_key: &RawStr) -> Result<Template> {
    panic!()
}

/// A treasure's image.
///
/// The `public_key` is bech32 encoded.
///
/// Need to set the mime/type.
/// For now set to image/jpeg.
#[get("/treasure-images/<public_key>")]
fn treasure_image(public_key: &RawStr) -> Result<Content<Vec<u8>>> {
    let public_key = public_key.percent_decode()?;
    let public_key = crypto::decode_public_key(&public_key)?;
    let public_key = crypto::encode_public_key(&public_key)?;

    let path = format!("data/treasure/{}", public_key);
    let file = BufReader::new(File::open(path)?);
    let record: api::PlantRequest = serde_json::from_reader(file)?;
    let encoded_image = record.image;
    let decoded_image = base64::decode(&encoded_image)?;

    // TODO: Correct content type
    Ok(Content(ContentType::JPEG, decoded_image))
}

fn main() {
    let css_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/css");
    let js_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/js");
    let images_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static/images");
    let wasm_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/wasm/pkg");
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/css", StaticFiles::from(css_dir))
        .mount("/js", StaticFiles::from(js_dir))
        .mount("/images", StaticFiles::from(images_dir))
        .mount("/wasm/pkg", StaticFiles::from(wasm_dir))
        .mount("/", routes![
            root_page,
            static_page,
            recent_page,
            treasure_page,
            treasure_image,
            api::create_treasure_key,
            api::plant_treasure_with_key,
            api::claim_treasure_with_key,
        ])
        .launch();
}
