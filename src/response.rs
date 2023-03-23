use http::StatusCode;
use image::io::Reader as ImageReader;
use image::{ImageError, ImageFormat, ImageResult};
use isahc::AsyncReadResponseExt;
use isahc::HttpClient;
use serde_json;
use std::io::Cursor;

pub struct Response {
    pub status: StatusCode,
    pub text: String,
    pub content_type: String,
}

impl Response {
    pub async fn get_response(
        url: &String,
        httpclient: &HttpClient,
    ) -> Result<Response, isahc::Error> {
        let mut response = httpclient.get_async(url).await?;

        let api_response = Response {
            status: response.status(),
            text: response.text().await?,
            content_type: format!("{:?}", response.headers().get("content-type").unwrap())
                .replace('"', ""),
        };

        Ok(api_response)
    }

    pub fn parse_json(content: &String) -> serde_json::Result<serde_json::Value> {
        let json: serde_json::Value = serde_json::from_str(&content)?;

        Ok(json)
    }

    pub async fn get_image(
        url: &String,
        http_client: &HttpClient,
    ) -> Result<Vec<u8>, isahc::Error> {
        let mut img_buffer = vec![];
        http_client
            .get_async(url)
            .await?
            .copy_to(&mut img_buffer)
            .await?;

        Ok(img_buffer)
    }

    pub fn save_image(filename: String, bytes: &Vec<u8>) -> Result<ImageResult<()>, ImageError> {
        let image = image::load_from_memory(&bytes);
        let mut filename_with_format = String::new();
        match image {
            Ok(_) => &image,
            Err(e) => panic!("{}", e),
        };

        let reader = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap();

        match reader.format() {
            Some(ImageFormat::Jpeg) => filename_with_format = filename + ".jpg",
            None => panic!("Failed to guess image format"),
            Some(format) => println!("Image format not supported = {:?}", format),
        }
        let img_save = image
            .unwrap()
            .save_with_format(filename_with_format, reader.format().unwrap());

        match img_save {
            Ok(_) => Ok(img_save),
            Err(e) => Err(e),
        }
    }
}
