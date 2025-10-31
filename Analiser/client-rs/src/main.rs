use dotenvy::dotenv;
use reqwest::blocking::get;
use serde_json::Value;
use std::env;
use std::fs;
use chrono::Local;
use std::path::PathBuf;

fn main() {
    dotenv().ok();

    // Lê a URL da Realtime Database do .env
    let database_url: String = env::var("FIREBASE_DATABASE_URL")
        .expect("Defina FIREBASE_DATABASE_URL no .env");

    // Monta a URL final com .json
    let url = format!("{}/.json", database_url);
    println!("🔍 Testando conexão com Firebase DB em: {}", url);

    // Faz a requisição GET
    let response = get(&url).expect("❌ Falha ao conectar ao Firebase");
    println!("✅ Status HTTP: {}", response.status());

    let body = response.text().unwrap();

    // Faz o parse e formata o JSON
    let parsed: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
    let pretty = serde_json::to_string_pretty(&parsed).unwrap();

    // Caminho absoluto para a pasta /analiser/data (um nível acima do client-rs)
    let mut data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_dir.pop(); // sobe um diretório (de client-rs → analiser)
    data_dir.push("data");

    // Cria pasta /data caso não exista
    fs::create_dir_all(&data_dir).expect("❌ Falha ao criar pasta /data");

    // Gera nome de arquivo com timestamp
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = data_dir.join(format!("firebase_{}.json", timestamp));

    // Salva o JSON formatado
    fs::write(&filename, pretty).expect("❌ Falha ao salvar JSON formatado");

    println!("💾 JSON salvo com sucesso em {}", filename.display());
}
