use serde_json::Value;
use sqlx::{Error, QueryBuilder};
use std::{env, fs};
use zirv_db_sqlx::get_db_pool;

pub async fn seed_database() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = get_db_pool!();

    let current_dir = std::env::current_dir()?;
    let data_dir = current_dir.join("src").join("seeder").join("data");

    for folder in &[
        "default",
        env::var("ENV")
            .unwrap_or_else(|_| "development".to_string())
            .as_str(),
    ] {
        let dir_path = data_dir.join(folder);

        if !dir_path.exists() {
            println!("Directory does not exist: {}", dir_path.display());
            continue;
        }

        if !dir_path.is_dir() {
            return Err("data folder is not a directory".into());
        }

        for entry in fs::read_dir(&dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            println!("Processing file: {}", path.display());

            let table_name = path
                .file_stem()
                .and_then(|s| {
                    s.to_str().map(|s| match s.starts_with("1_") {
                        | true => s[2..].to_string(),
                        | false => s.to_string(),
                    })
                })
                .ok_or("invalid filename")?;

            let raw = fs::read_to_string(&path)?;
            let rows: Vec<serde_json::Map<String, Value>> = serde_json::from_str(&raw)?;
            if rows.is_empty() {
                continue;
            }

            let columns: Vec<String> = rows[0].keys().cloned().collect();

            let mut qb = QueryBuilder::new(format!("INSERT INTO {} ", table_name));
            qb.push("(");
            for (i, col) in columns.iter().enumerate() {
                qb.push(col);
                if i + 1 < columns.len() {
                    qb.push(", ");
                }
            }
            qb.push(") VALUES ");

            for (ri, row) in rows.iter().enumerate() {
                qb.push("(");
                for (ci, col) in columns.iter().enumerate() {
                    let val = row.get(col).unwrap_or(&Value::Null);
                    match val {
                        | Value::Null => {
                            // Bind NULL
                            qb.push_bind(None::<String>);
                        }
                        | Value::Bool(b) => {
                            qb.push_bind(*b);
                        }
                        | Value::Number(n) if n.is_i64() => {
                            qb.push_bind(n.as_i64().unwrap());
                        }
                        | Value::Number(n) if n.is_f64() => {
                            qb.push_bind(n.as_f64().unwrap());
                        }
                        | Value::Number(n) => {
                            qb.push_bind(n.to_string());
                        }
                        | Value::String(s) => {
                            qb.push_bind(s);
                        }
                        | other => {
                            qb.push_bind(other.to_string());
                        }
                    }

                    if ci + 1 < columns.len() {
                        qb.push(", ");
                    }
                }
                qb.push(")");

                if ri + 1 < rows.len() {
                    qb.push(", ");
                }
            }

            match qb.build().execute(pool).await {
                | Ok(_) => {}
                | Err(e) => {
                    if let Error::Database(db_err) = &e
                        && let Some(code) = db_err.code()
                        && code.starts_with("23")
                    {
                        return Ok(());
                    }
                    eprintln!("Skipping inserting into {}: {}", table_name, e);
                    return Ok(());
                }
            };
        }
    }

    Ok(())
}
