use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                post_form("hello".to_string());

                if false {
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

pub fn post_form(prompt: String) {
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get("http://localhost:8000")
            .body(format!(
                r#"
            {{
            "prompt": "{}",
            }}
            "#,
                prompt
            ))
            .send();

        let response = response.unwrap();

        let mut reader = BufReader::new(response);
        let mut buf = String::new();

        let mut i = 0;

        while let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            }

            // *progress.lock() = i;
            println!("response {i} is ... {:?}", &buf);
            buf = String::new();
            i += 1;
        }
    });
}
