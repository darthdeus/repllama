use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::process::{Command, Stdio};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                Command::new("stdbuf")
                    .current_dir("../llama.cpp")
                    .arg("-o0")
                    .arg("./main")
                    .arg("-m")
                    // .arg("./models/7B/ggml-model-f16.bin")
                    .arg("./models/7B/ggml-model-q4_0.bin")
                    .arg("-t")
                    .arg("12")
                    .arg("-n")
                    .arg("32")
                    .arg("-p")
                    .arg(line.trim())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("Failed to execute command");
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
