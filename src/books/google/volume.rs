use super::{ApiConfig, Volume};
use reqwest::get;
use serde_json::Value;

fn get_url(volume_id: &str, params: &ApiConfig) -> String {
    format!(
        "{base}volumes/{id}?key={key}",
        base = params.base_uri,
        key = params.api_key,
        id = volume_id
    )
}

pub async fn get_volume(volume_id: &str, params: &ApiConfig) -> Option<Volume> {
    let url: String = get_url(volume_id, params);
    match get(&url).await {
        Ok(response) => {
            println!("Volume: {:?}", response.status());
            match response.status().as_u16() {
                200 => match response.json::<Value>().await {
                    Ok(data) => create_volume(data),
                    Err(_) => None,
                },
                _ => None,
            }
        }
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

fn create_volume(data: Value) -> Option<Volume> {
    let mut volume = Volume::default();
    let _ = data
        .get("volumeInfo")
        .and_then(|info| info.as_object())
        .map(|info| {
            for (k, vs) in info.iter() {
                match k.as_ref() {
                    "title" => volume.title = as_string(vs),
                    "description" => volume.description = as_string(vs),
                    "publisher" => volume.publisher = as_string(vs),
                    "publishedDate" => volume.published_date = as_string(vs),
                    "language" => volume.language = as_string(vs),
                    "pageCount" => volume.pages = as_i64(vs),
                    "imageLinks" => {
                        volume.image = vs
                            .get("thumbnail")
                            .map(|thumb| as_string(thumb))
                            .unwrap_or_default()
                    }
                    "authors" => {
                        volume.authors = vs
                            .as_array()
                            .map(|list| list.iter().map(|item| as_string(item)).collect())
                            .unwrap_or_default()
                    }
                    _ => (),
                };
            }
        });
    Some(volume)
}

fn as_string(v: &Value) -> String {
    v.as_str().map_or(String::new(), |v| v.to_string())
}

fn as_i64(v: &Value) -> i64 {
    v.as_i64().map_or(0, |v| v)
}
