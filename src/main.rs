use std::env;
use std::process::Command;
use std::path::Path;
use difflib::sequencematcher::SequenceMatcher;
use std::sync::mpsc::channel;
use std::thread;


#[derive(Debug, PartialEq, Clone)]
enum Mode {
    Reference,
    Student,
}

#[derive(Debug, PartialEq, Clone)]
struct Script {
    mode: Mode,
    text: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: speak-eval <reference-filename> <student-filename>");
        return;
    }

    let file1 = args[1].clone();
    let file2 = args[2].clone();

    let (tx, rx) = channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let text = run_transcribe(&file1.as_str());
        let script = Script {
            mode: Mode::Reference,
            text,
        };
        tx.send(script).unwrap();
    });

    thread::spawn(move || {
        let text = run_transcribe(&file2.as_str());
        let script = Script {
            mode: Mode::Student,
            text,
        };
        tx1.send(script).unwrap();
    });

    let mut script1: Script = Script {
        mode: Mode::Reference,
        text: "".to_string(),
    };
    let mut script2: Script = Script {
        mode: Mode::Student,
        text: "".to_string(),
    };
    for script in rx {
        if script.mode == Mode::Reference {
            script1 = script;
        } else {
            script2 = script;
        }
    }

    let mut matcher = SequenceMatcher::new(&script1.text, &script2.text);
    println!("{:?}", script1);
    println!("{:?}", script2);
    println!("{:?}", matcher.ratio());
    println!("{:?}", evaluate_similiarity(matcher.ratio()));
}


#[allow(dead_code)]
fn get_txt_filename_from(file_path: &str) -> String {
    let file_path = Path::new(file_path);
    let dir = file_path.parent().unwrap();
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
    let file_name = dir.join(file_stem).with_extension("txt");
    println!("{:?}", file_name);
    file_name.to_str().unwrap().to_string()
}

fn run_transcribe(file_path: &str) -> String {
    let output = Command::new("python3")
        .args(["./transcribe.py", &file_path])
        .output()
        .expect("failed to execute process");

    output.stdout.iter().map(|&x| x as char).collect()
}

fn evaluate_similiarity(score: f32) -> &'static str {
    if score > 0.9 {
        "Excellent"
    } else if score > 0.7 {
        "Very Good"
    } else if score > 0.6 {
        "Good"
    } else if score > 0.4 {
        "Not Bad"
    } else {
        "Try Again"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_txt_filename_from() {
        let file_path = "./data/1.mp3";
        assert_eq!(get_txt_filename_from(file_path), "./data/1.txt".to_string());
    }
}