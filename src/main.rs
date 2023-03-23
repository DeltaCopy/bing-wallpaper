use crate::dbuswallpaper::DbusWallpaper;
use crate::response::Response;
use std::path::Path;

mod client;
mod dbuswallpaper;
mod response;

fn main() {
    println!(":: Setting wallpaper from https://bing.com");
    process_image();
}

fn process_image() {
    let base_url = String::from("https://bing.com");
    let url = String::from("https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1");
    let http_client = client::build_http_client().unwrap();
    let dest_path = &String::from("/tmp/bing_wallpaper");
    let invalid_chars = vec![
        ':', '?', '!', '\'', ' ', '`', '¬', '@', '#', ',', '\'', '’', ';',
    ];

    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt
        .block_on(Response::get_response(&url, &http_client))
        .unwrap();

    if response.status == 200 {
        println!("{}",&response.text);
        let json_response = Response::parse_json(&response.text).unwrap();
        if json_response["images"].as_array().unwrap().len() > 0 {
            let image_url = json_response["images"][0]["url"].as_str().unwrap();
            let start_date = json_response["images"][0]["startdate"].as_str().unwrap();

            let copyright = json_response["images"][0]["copyright"].as_str().unwrap();

            println!(
                ":: Title = {}",
                json_response["images"][0]["title"].as_str().unwrap()
            );
            println!(":: Copyright = {}", copyright);
            if image_url.len() > 0 {
                let wallpaper_img = base_url + image_url;
                let mut title = String::new();

                for c in json_response["images"][0]["title"]
                    .as_str()
                    .unwrap()
                    .to_lowercase()
                    .replace(" ", "_")
                    .chars()
                {
                    if !invalid_chars.contains(&c) {
                        title += &c.to_string();
                    }
                }

                if title.len() > 0 {
                    if !Path::new(&dest_path).exists() {
                        match std::fs::create_dir(&dest_path) {
                            Err(e) => {
                                panic!(":: Couldn't create image directory {}, {}", dest_path, e)
                            }
                            Ok(_file) => {
                                println!(
                                    ":: Image directory created successfully = {:?}.",
                                    dest_path
                                );
                            }
                        };
                    }
                } else {
                    panic!("Failed to process image title.");
                }

                let dbus_wallpaper_conn = DbusWallpaper::new(
                    dest_path.to_owned() + "/" + &start_date + "_" + &title + ".jpg",
                );

                if !Path::new(&(dest_path.to_owned() + "/" + &start_date + "_" + &title + ".jpg"))
                    .exists()
                {
                    let image_buffer =
                        rt.block_on(Response::get_image(&wallpaper_img, &http_client));

                    match image_buffer {
                        Ok(img_buf) => {
                            println!(":: Size of image buffer = {}KiB", (img_buf.len() / 1024));
                            println!(":: Saving to disk.");

                            let img_save = Response::save_image(
                                dest_path.to_owned() + "/" + &start_date + "_" + &title,
                                &img_buf,
                            );
                            match img_save {
                                Ok(_) => dbus_wallpaper_conn.set_wallpaper(),

                                Err(e) => panic!("{}", e),
                            };
                        }
                        Err(e) => panic!("{:?}", e),
                    }
                } else {
                    println!(
                        ":: File = {} already exists on disk.",
                        dest_path.to_owned() + "/" + &start_date + "_" + &title + ".jpg"
                    );

                    dbus_wallpaper_conn.set_wallpaper();
                }
            }
        }
    } else {
        panic!("{}", response.text);
    }
}
