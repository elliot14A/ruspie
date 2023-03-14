use std::sync::Arc;

use crate::{api::get_max_limit, context::api_context::RuspieApiContext};
use axum::{body::Bytes, extract, http::HeaderMap, response::IntoResponse, Extension};
use columnq::encoding;

use super::{extract_ext_from_headers, get_limit, get_table_source_fs};
use roapi::{
    api::{encode_record_batches, encode_type_from_hdr},
    error::ApiErrResp,
};
use tokio::sync::Mutex;

pub async fn sql<H: RuspieApiContext>(
    Extension(ctx): extract::Extension<Arc<Mutex<H>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiErrResp> {
    let mut context = ctx.lock().await;
    let mut query = std::str::from_utf8(&body)
        .map_err(ApiErrResp::read_query)?
        .to_string();
    let idx = query
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .position(|&x| x.to_lowercase() == "from")
        .unwrap();
    let table_name = query.split(" ").collect::<Vec<&str>>()[idx + 1];
    if !context.table_exists(table_name).await {
        let extension = extract_ext_from_headers(&headers);
        let table_source = get_table_source_fs(table_name, &extension);
        if let Err(e) = context.conf_table(&table_source).await {
            return Err(ApiErrResp::load_table(e));
        }
    }
    if query.contains("limit") || query.contains("LIMIT") {
        let vec_q = query.split(" ").collect::<Vec<&str>>();
        let i = vec_q
            .iter()
            .position(|&x| x == "limit" || x == "LIMIT")
            .unwrap();
        let mut limit = vec_q[i + 1].parse::<i64>().unwrap();
        if limit > get_max_limit() {
            limit = get_limit();
        }
        query = query.replace(vec_q[i + 1], limit.to_string().as_str())
    } else {
        let limit = get_limit().to_string();
        query = query + " limit " + limit.as_str();
    }
    let encode_type = encode_type_from_hdr(headers, encoding::ContentType::default());
    let batches = context.query_sql_ruspie(query.as_str()).await?;

    encode_record_batches(encode_type, &batches)
}
