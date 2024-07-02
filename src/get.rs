use crate::data::Data;

use google_sheets4::{
    api::ValueRange,
    hyper::{client::HttpConnector, Client},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    oauth2::{
        authenticator::Authenticator, read_service_account_key, ServiceAccountAuthenticator,
        ServiceAccountKey,
    },
    Sheets,
};
use serde_json::value::Value;

type GetResult<T> = std::result::Result<T, String>;

pub async fn get_data(sheet_id: &str) -> GetResult<Vec<Data>> {
    let client = http_client();
    let auth = auth(client.clone()).await?;
    let hub = Sheets::new(client.clone(), auth);
    let data = read(&hub, sheet_id).await?;
    
    Ok(format(data))
}

fn http_client() -> Client<HttpsConnector<HttpConnector>> {
    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();

    Client::builder().build(connector)
}

async fn auth(
    client: Client<HttpsConnector<HttpConnector>>,
) -> GetResult<Authenticator<HttpsConnector<HttpConnector>>> {
    let secret: ServiceAccountKey = read_service_account_key("spreadsheet_priv_key.json")
        .await
        .map_err(|e| format!("{}", e))?;

    ServiceAccountAuthenticator::with_client(secret, client)
        .build()
        .await
        .map_err(|e| format!("{}", e))
}

async fn read(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &str,
) -> GetResult<ValueRange> {
    hub.spreadsheets()
        .values_get(sheet_id, "A2:D201")
        .doit()
        .await
        .map(|x| x.1)
        .map_err(|e| format!("{}", e))
}

fn format(
    spreadsheet: ValueRange
) -> Vec<Data> {
    spreadsheet
        .values
        .unwrap()
        .into_iter()
        .flat_map(|row| extract(row))
        .collect()
}

fn extract(row: Vec<Value>) -> Option<Data> {
    let proverb = row.get(0)
        .map(|v| v.as_str().unwrap())
        .map(|v| v.to_string())?;

    let speaker = row.get(1)
        .map(|v| v.as_str().unwrap())
        .map(|v| v.to_string())?;

    let date = row.get(2)
        .map(|v| v.as_str().unwrap())
        .map(|v| v.to_string())?;

    let rarity = row.get(3)
        .map(|v| v.as_str().unwrap())
        .map(|v| v.to_string())?;

    Some(Data::new(proverb, speaker, date, rarity))
}
