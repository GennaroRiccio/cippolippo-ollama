use clearscreen;
use std::error::Error;
use dialoguer::{theme::ColorfulTheme, Input};
use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationContext, GenerationResponseStream,
    },
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;
use owo_colors::{DynColors, OwoColorize};
use fancy::printcoln;
use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
struct Data {
    config: Config,
}
#[derive(Deserialize)]
struct Config {
    model: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    clearscreen::clear().expect("failed to clear screen");
    const COLORS: &str = r#"
     _______                   __    _
    / ____(_)___  ____  ____  / /   (_)___  ____  ____
   / /   / / __ \/ __ \/ __ \/ /   / / __ \/ __ \/ __ \
  / /___/ / /_/ / /_/ / /_/ / /___/ / /_/ / /_/ / /_/ /
  \____/_/ .___/ .___/\____/_____/_/ .___/ .___/\____/
        /_/   /_/                 /_/   /_/

     "#;
    let colors: [DynColors; 6] = ["#B80A41", "#4E4BA8", "#6EB122", "#DAAC06", "#00938A", "#E23838",]
        .map(|color| color.parse().unwrap());

    for line in COLORS.split_inclusive('\n') {
        for (text, color) in line.split('|').zip(colors.iter().copied()) {
            print!("{}", text.color(color).bold());
        }
    }
   let title = format!("Ollama-Copilot (C) 2024 by Gennaro Riccio v{}",env!("CARGO_PKG_VERSION"));
   printcoln!("[green]{}",title);

    let filename = "config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    printcoln!("[blue] Modello Usato: {}",data.config.model);

    loop{
        let prompt: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Dimmi:")
            .interact_text()
            .unwrap();

        if prompt == "exit" {
            printcoln!("[green][bold]Tanto Ciao!.[bold]");
            break;
        }
        let mut stdout = stdout();
        let ollama = Ollama::default();
        let context: Option<GenerationContext> = None;
        let model = data.config.model.to_string();
        println!();
        let mut request = GenerationRequest::new(model, prompt);
        if let Some(context) = context.clone() {
            request = request.context(context);
        }
        let mut stream: GenerationResponseStream = ollama.generate_stream(request).await?;
        while let Some(Ok(res)) = stream.next().await {
            for ele in res {
                stdout.write_all(ele.response.as_bytes()).await?;
                stdout.flush().await?;

                if let Some(final_data) = ele.context {
                    _ = Some(final_data);
                }
            }
        }
        println!("\n");
    }
    Ok(())
}
