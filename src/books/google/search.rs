use super::ApiConfig;
use reqwest::get;
use serde_json::Value;

fn get_url(isbn: &str, params: &ApiConfig) -> String {
    format!(
        "{base}volumes/?projection=full&key={key}&q=isbn:{isbn}",
        base = params.base_uri,
        key = params.api_key,
        isbn = isbn
    )
}

pub async fn search_isbn(isbn: &str, params: &ApiConfig) -> Option<String> {
    let url: String = get_url(isbn, params);

    match get(&url).await {
        Ok(response) => {
            println!("Search: {:?}", response.status());
            match response.status().as_u16() {
                200 => match response.json::<Value>().await {
                    Ok(data) => get_first_result(data),
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

fn get_first_result(data: Value) -> Option<String> {
    data.get("items")
        .and_then(|items| items.as_array())
        .and_then(|list| {
            list.get(0).map(|item| {
                item.get("id")
                    .and_then(|value| value.as_str())
                    .map(|value| value.to_string())
                    .unwrap_or_default()
            })
        })
}
