use std::env;
use std::process::Command;
use std::path::Path;
use difflib::sequencematcher::SequenceMatcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let file1 = &args[1];
    let file2 = &args[2];

    println!("file1: {}", file1);
    println!("file2: {}", file2);
    let txt_file1 = get_transcript(file1.as_str());
    let txt_file2 = get_transcript(&file2.as_str());

    let script1 = std::fs::read_to_string(&txt_file1).unwrap();
    let script2 = std::fs::read_to_string(&txt_file2).unwrap();

    let mut matcher = SequenceMatcher::new(&script1, &script2);
    println!("{:?}", matcher.ratio());
    println!("{:?}", evaluate_similiarity(matcher.ratio()));
}


fn get_txt_filename_from(file_path: &str) -> String {
    let path = Path::new(file_path);
    let dir = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let file_name = dir.join(file_stem).with_extension("txt");
    println!("{:?}", file_name);
    file_name.to_str().unwrap().to_string()
}

fn get_transcript(file_path: &str) -> String{
    let _ = Command::new("whisper")
        .args([&file_path, "-f", "txt", "-o", "./data", "--language", "en", "--model", "base"])
        .output()
        .expect("failed to execute process");

    get_txt_filename_from(file_path)
}

fn evaluate_similiarity(score: f32) -> &'static str {
    if score > 0.95 {
        "Perfect"
    } else if score > 0.9 {
        "Excellent"
    } else if score > 0.7 {
        "Good"
    } else if score > 0.5 {
        "Fair"
    } else {
        "Poor"
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