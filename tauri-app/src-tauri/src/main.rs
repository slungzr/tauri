// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use redis::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use std::fs::File;
use std::io::Write;

struct RedisState(Mutex<Option<(Client, Option<String>)>>);

#[derive(Debug, Serialize, Deserialize)]
struct RedisConfig {
    host: String,
    port: u16,
    password: Option<String>,
}

#[derive(Debug, Serialize)]
struct RedisResponse {
    success: bool,
    error: Option<String>,
    data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct KeyData {
    key: String,
    success: bool,
    error: Option<String>,
    data: Option<serde_json::Value>,
}

// 添加新的辅助函数用于创建 Redis 连接
fn create_redis_connection(config: &RedisConfig) -> Result<Client, String> {
    let url = if let Some(password) = &config.password {
        format!(
            "redis://:{}@{}:{}/",
            password,
            config.host,
            config.port
        )
    } else {
        format!(
            "redis://{}:{}/",
            config.host,
            config.port
        )
    };

    Client::open(url).map_err(|e| e.to_string())
}

// 添加新的辅助函数用于获取认证的连接
fn get_authenticated_connection(
    client: &Client,
    password: Option<&String>,
) -> Result<redis::Connection, String> {
    let mut con = client.get_connection().map_err(|e| e.to_string())?;
    
    // 如果连接 URL 中没有包含密码，则手动进行认证
    if let Some(pwd) = password {
        redis::cmd("AUTH")
            .arg(pwd)
            .query::<()>(&mut con)
            .map_err(|e| e.to_string())?;
    }

    // 测试连接
    redis::cmd("PING")
        .query::<String>(&mut con)
        .map_err(|e| e.to_string())?;

    Ok(con)
}

#[tauri::command]
async fn connect_redis(
    state: State<'_, RedisState>,
    config: RedisConfig,
) -> Result<RedisResponse, String> {
    match create_redis_connection(&config) {
        Ok(client) => {
            // 测试连接
            let _ = get_authenticated_connection(&client, config.password.as_ref())?;

            // 保存客户端连接和密码
            let mut lock = state.0.lock().unwrap();
            *lock = Some((client, config.password));
            
            Ok(RedisResponse {
                success: true,
                error: None,
                data: None,
            })
        }
        Err(e) => Ok(RedisResponse {
            success: false,
            error: Some(e.to_string()),
            data: None,
        }),
    }
}

#[tauri::command]
async fn search_keys(
    state: State<'_, RedisState>,
    pattern: String,
) -> Result<RedisResponse, String> {
    let lock = state.0.lock().unwrap();
    let (client, password) = lock.as_ref().ok_or("未连接到Redis")?;
    
    let mut con = get_authenticated_connection(client, password.as_ref())?;

    let mut keys = Vec::new();
    let mut cursor: u64 = 0;
    
    loop {
        let (new_cursor, scan_keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg(&pattern)
            .arg("COUNT")
            .arg(10000)
            .query(&mut con)
            .map_err(|e| e.to_string())?;
            
        keys.extend(scan_keys);
        
        if new_cursor == 0 {
            break;
        }
        cursor = new_cursor;
    }

    Ok(RedisResponse {
        success: true,
        error: None,
        data: Some(serde_json::json!(keys)),
    })
}

#[tauri::command]
async fn get_keys_data(
    state: State<'_, RedisState>,
    keys: Vec<String>,
) -> Result<RedisResponse, String> {
    let lock = state.0.lock().unwrap();
    let (client, password) = lock.as_ref().ok_or("未连接到Redis")?;
    
    let mut con = get_authenticated_connection(client, password.as_ref())?;

    let mut results = Vec::new();
    
    for key in keys {
        let result = match get_key_data_internal(&mut con, &key) {
            Ok(data) => KeyData {
                key,
                success: true,
                error: None,
                data: Some(data),
            },
            Err(e) => KeyData {
                key,
                success: false,
                error: Some(e),
                data: None,
            },
        };
        results.push(result);
    }

    Ok(RedisResponse {
        success: true,
        error: None,
        data: Some(serde_json::json!(results)),
    })
}

// 将原来的get_key_data内部逻辑抽取为独立函数
fn get_key_data_internal(con: &mut redis::Connection, key: &str) -> Result<serde_json::Value, String> {
    // 获取键类型
    let key_type: String = redis::cmd("TYPE")
        .arg(key)
        .query(con)
        .map_err(|e| e.to_string())?;

    // 获取TTL
    let ttl: i64 = redis::cmd("TTL")
        .arg(key)
        .query(con)
        .map_err(|e| e.to_string())?;

    // 根据类型获取值
    let value: serde_json::Value = match key_type.as_str() {
        "string" => {
            let val: String = redis::cmd("GET")
                .arg(key)
                .query(con)
                .map_err(|e| e.to_string())?;
            serde_json::json!(val)
        }
        "hash" => {
            let val: std::collections::HashMap<String, String> = redis::cmd("HGETALL")
                .arg(key)
                .query(con)
                .map_err(|e| e.to_string())?;
            serde_json::json!(val)
        }
        "list" => {
            let val: Vec<String> = redis::cmd("LRANGE")
                .arg(key)
                .arg(0)
                .arg(-1)
                .query(con)
                .map_err(|e| e.to_string())?;
            serde_json::json!(val)
        }
        "set" => {
            let val: Vec<String> = redis::cmd("SMEMBERS")
                .arg(key)
                .query(con)
                .map_err(|e| e.to_string())?;
            serde_json::json!(val)
        }
        "zset" => {
            let val: Vec<(String, f64)> = redis::cmd("ZRANGE")
                .arg(key)
                .arg(0)
                .arg(-1)
                .arg("WITHSCORES")
                .query(con)
                .map_err(|e| e.to_string())?;
            serde_json::json!(val)
        }
        _ => serde_json::json!(null),
    };

    Ok(serde_json::json!({
        "type": key_type,
        "ttl": ttl,
        "value": value
    }))
}

#[tauri::command]
async fn get_key_data(
    state: State<'_, RedisState>,
    key: String,
) -> Result<RedisResponse, String> {
    let lock = state.0.lock().unwrap();
    let (client, password) = lock.as_ref().ok_or("未连接到Redis")?;
    
    let mut con = get_authenticated_connection(client, password.as_ref())?;

    match get_key_data_internal(&mut con, &key) {
        Ok(data) => Ok(RedisResponse {
            success: true,
            error: None,
            data: Some(data),
        }),
        Err(e) => Ok(RedisResponse {
            success: false,
            error: Some(e),
            data: None,
        }),
    }
}

#[tauri::command]
async fn delete_key(
    state: State<'_, RedisState>,
    key: String,
) -> Result<RedisResponse, String> {
    let lock = state.0.lock().unwrap();
    let (client, password) = lock.as_ref().ok_or("未连接到Redis")?;
    
    let mut con = get_authenticated_connection(client, password.as_ref())?;

    redis::cmd("DEL")
        .arg(&key)
        .query::<i32>(&mut con)
        .map_err(|e| e.to_string())?;

    Ok(RedisResponse {
        success: true,
        error: None,
        data: None,
    })
}

#[tauri::command]
async fn write_text_file(file_path: String, content: String) -> Result<RedisResponse, String> {
    match File::create(&file_path) {
        Ok(mut file) => {
            match file.write_all(content.as_bytes()) {
                Ok(_) => Ok(RedisResponse {
                    success: true,
                    error: None,
                    data: None,
                }),
                Err(e) => Ok(RedisResponse {
                    success: false,
                    error: Some(format!("写入文件失败: {}", e)),
                    data: None,
                }),
            }
        },
        Err(e) => Ok(RedisResponse {
            success: false,
            error: Some(format!("创建文件失败: {}", e)),
            data: None,
        }),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(RedisState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            connect_redis,
            search_keys,
            get_key_data,
            get_keys_data,
            delete_key,
            write_text_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
