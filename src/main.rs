use error_chain::error_chain;
use std::io::{copy, Write};
use std::fs::File;
use std::thread::Builder;
use std::{env, fs};
use serde_json::{Value, Error, Map};
use serde::{Serializer, Serialize, Deserialize, Deserializer};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReqJSON {
    pub provenance: String,
    pub collection: Vec<Collections>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collections {
    #[serde(alias = "tokenId")]
    pub token_id: u32,
    #[serde(alias = "image")]
    pub image: String
}


#[tokio::main]
async fn main() -> Result<(), ()> {


    let response_json: serde_json::Value = reqwest::get("https://ipfs.io/ipfs/Qme57kZ2VuVzcj5sC3tVHFgyyEgBTmAnyTK45YVNxKf6hi").await.unwrap().json().await.unwrap();

    let parse_json: ReqJSON = serde_json::from_value(response_json).unwrap();

    let arr_images = parse_json.collection.iter().map(|collect| collect.image.split("/ipfs/*").map(std::string::ToString::to_string)
        .collect()).collect::<Vec<String>>();

    let mut links = vec![];

    for mut image in arr_images {
        let image = image.split_off(16);
        links.push(image);
    }

    // println!("{:?}", links);

    for link in links {
    let object_path = link;
    let target = format!("https://ipfs.io/{}", object_path);
    let response = reqwest::get(&target).await.unwrap();

    let mut dest = {

        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");


        println!("file to download: '{}'", fname);

        let object_prefix = &object_path[..object_path.rfind('/').unwrap()];
        let object_name = &object_path[object_path.rfind('/').unwrap()+1..];
        let output_dir = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap().to_string(), object_prefix);
        fs::create_dir_all(output_dir.clone()).unwrap();

        println!("will be located under: '{}'", output_dir.clone());

        let output_fname = format!("{}/{}", output_dir, object_name);
        println!("Creating the file {}", output_fname);

        File::create(output_fname).unwrap()

    };
    let content =  response.bytes().await.unwrap();

    let mut pos = 0;
    while pos < content.len() {
        let bytes_written = dest.write(&content[pos..]).unwrap();
        pos += bytes_written;
    }
    }
    Ok(())
}

