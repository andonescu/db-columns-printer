use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct Config {
    exclude_tables: Vec<String>,
    exclude_columns: std::collections::HashMap<String, Vec<String>>,
}

fn main() -> Result<()> {
  
    let file = File::open("config.json").expect("Unable to open config file");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("Unable to parse config file");

    let url = "mysql://root:password@127.0.0.1:3306/TEST";

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;


    let tables: Vec<String> = conn.query("SHOW TABLES")?;

  

    for table in tables {

        // Skip excluded tables
        if config.exclude_tables.contains(&table) {
            continue;
        }

        println!("Table: {}", table);
        let query = format!("SHOW COLUMNS FROM {}", table);
        let columns: Vec<(String, String, String, String, Option<String>, String)> = conn.query(query)?;

        let mut index = 0;

        for column in columns {
            index += 1;

            if let Some(excluded_columns) = config.exclude_columns.get(&table.to_lowercase()) {
                if excluded_columns.contains(&column.0.to_lowercase()) {
                    continue;
                }
            }
           
            println!("{} - Column: {} - Type: {} - Null: {} - Key: {} - Default: {:?} - Extra: {}",
                index ,column.0, column.1, column.2, column.3, column.4, column.5);
        }

        println!();
    }

    Ok(())
}