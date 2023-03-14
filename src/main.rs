use reqwest;
use reqwest::header::{ACCEPT,AUTHORIZATION,CONTENT_TYPE};
use std::env;
use serde::{Deserialize,Serialize};




#[derive(Deserialize,Serialize,Debug)]
struct ExternalUrls{
    sportify: String
}
#[derive(Deserialize,Serialize,Debug)]
struct Artist{
    name: String,
    external_urls: ExternalUrls
}

#[derive(Deserialize,Serialize,Debug)]
struct Album{
    name: String,
    artis: Vec<Artist>,
    external: ExternalUrls
}
#[derive(Deserialize,Serialize,Debug)]
struct Track{
    name: String,
    href: String,
    album: Album,
    popularity: u32,
    external_urls: ExternalUrls
}
#[tokio::main]
async fn main() {
   
}
