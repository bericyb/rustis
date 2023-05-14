use axum::extract::{Json, Path};
use serde_json::Value;
use tracing::info;

use crate::DB;

pub async fn read(Path(key): Path<String>) -> Json<Value> {
    info!(key, "Reading key:");
    let data = DB.get().get(&key).await;
    if let Some(data) = data {
        return Json(data.value().clone());
    }
    return Json(Value::Null);
}

pub async fn create(Path(key): Path<String>, Json(value): Json<Value>) -> Json<Value> {
    info!(key, "Setting key:");
    let data = DB.get().insert(key, value).await;
    if let Some(data) = data {
        return Json(data);
    } else {
        return Json(Value::Null);
    }
}

pub async fn update(Path(key): Path<String>, Json(value): Json<Value>) -> Json<Value> {
    info!(key, "Updating key:");
    let data = DB.get().update(key, value).await.unwrap();
    return Json(data.value().clone());
}

pub async fn delete(Path(key): Path<String>) -> Json<Value> {
    info!(key, "Deleting key:");
    let data = DB.get().delete(key).await;
    if let Some(data) = data {
        return Json(data.1);
    }
    return Json(Value::Null);
}
