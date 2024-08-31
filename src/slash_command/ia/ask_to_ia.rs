
use reqwest::Client;

use crate::prelude::*;

/// ❓| Converse com a Marie!
#[poise::command(slash_command, prefix_command)]
pub async fn ask_anything(
    ctx: Context<'_>,
    #[description = "Pergunte algo para Marie!"] ask: String,
) -> Result<()> {
    ctx.say("pensando...").await.unwrap();
    let request_body = serde_json::json!({
        "model": "lmstudio-community/Meta-Llama-3.1-8B-Instruct-GGUF",
        "messages": [
            { "role": "user", "content": ask }
        ],
        "temperature": 0.7,
        "max_tokens": -1,
        "stream": true
    });
    let url = "http://localhost:1234/v1/chat/completions";
    let client = Client::new();
    // Envia a requisição POST
    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?;

    // Lê o corpo da resposta como texto
    let response_text = response.text().await?;
    // Inicializa uma string para armazenar o conteúdo concatenado
    let mut concatenated_content = String::new();

    // Processa cada linha da string
    for line in response_text.lines() {
        // Remove o prefixo "data: " se estiver presente
        let trimmed_line = line.trim_start_matches("data: ");

        // Converte a linha para um objeto JSON
        match serde_json::from_str::<Value>(trimmed_line) {
            Ok(json_value) => {
                // Acessa o campo "content"
                if let Some(choices) = json_value["choices"].as_array() {
                    if let Some(choice) = choices.get(0) {
                        if let Some(content) = choice["delta"]["content"].as_str() {
                            concatenated_content.push_str(content);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Erro ao parsear JSON: {}", e),
        }
    }
    let mut guard = ctx.data().ia_response.lock().unwrap();
    *guard = concatenated_content;
    Ok(())
}