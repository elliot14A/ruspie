use std::sync::Arc;

use super::{extract_ext_from_headers, get_table_source_fs, get_table_source_s3};
use crate::context::api_context::RuspieApiContext;
use axum::{extract, http::HeaderMap, response::IntoResponse, Extension};
use roapi::{api::bytes_to_json_resp, error::ApiErrResp};
use tokio::sync::Mutex;

pub async fn schema<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    extract::Path(table): extract::Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    if !context.table_exists(&table).await {
        let extension = extract_ext_from_headers(&headers);
        // let table_source = get_table_source_fs(&table, &extension);
        let table_source = match context.get_source() {
            &crate::context::api_context::Source::S3 => {
                get_table_source_s3(&table, &extension, &headers)
            }
            _ => get_table_source_fs(&table, &extension),
        };
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    let payload = context.table_schema_json_bytes(&table).await?;
    Ok(bytes_to_json_resp(payload))
}
