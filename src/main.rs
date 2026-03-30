// chrono: 날짜/시간을 다루는 라이브러리
use chrono::Local;
// 파일 읽기/쓰기를 위한 표준 라이브러리
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
// 커맨드라인 인자를 받기 위한 라이브러리
use std::env;

// 로그 파일 경로 (상수로 정의)
const LOG_FILE: &str = "log.txt";

// 로그를 추가하는 함수
fn add_log(message: &str) {
    let now = Local::now();
    let timestamp = now.format("[%Y-%m-%d %H:%M]").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("파일을 열 수 없습니다");

    writeln!(file, "{} {}", timestamp, message)
        .expect("파일에 쓸 수 없습니다");

    println!("로그가 추가되었습니다: {} {}", timestamp, message);
}

// 모든 로그를 출력하는 함수
fn list_logs() {
    let file = match File::open(LOG_FILE) {
        Ok(f) => f,
        Err(_) => {
            println!("로그가 없습니다.");
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(log) = line {
            println!("{}", log);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("사용법:");
        println!("  log add \"메시지\"  - 로그 추가");
        println!("  log list          - 로그 목록 보기");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("오류: 메시지를 입력하세요");
                return;
            }
            add_log(&args[2]);
        }
        "list" => {
            list_logs();
        }
        _ => {
            println!("알 수 없는 명령어: {}", args[1]);
        }
    }
}