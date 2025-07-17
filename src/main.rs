use std::fs::{self, File};
use std::io::{self, BufRead};
// use std::path::Path;

fn main() -> io::Result<()> {
    // 검색할 문자열
    let target = "rasberry";

    // 현재 디렉토리 경로
    let current_dir = std::env::current_dir()?;

    // 디렉토리 내 파일 탐색
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // 확장자가 xml 또는 txt 인지 확인
        if let Some(ext) = path.extension() {
            if ext == "xml" || ext == "txt" {
                // 파일 열기 시도
                match File::open(&path) {
                    Ok(file) => {
                        let reader = io::BufReader::new(file);
                        
                        // 각 라인별로 검색
                        for (idx, line) in reader.lines().enumerate() {
                            match line {
                                Ok(line_content) if line_content.contains(target) => {
                                    println!(
                                        "파일: {:?}, 라인: {}, 내용: {}",
                                        path.file_name().unwrap_or_default(),
                                        idx + 1,
                                        line_content.trim()
                                    );
                                }
                                Err(e) => {
                                    eprintln!("경고: 파일 {:?}의 {}번째 라인을 읽는 중 오류 발생: {}", 
                                        path.file_name().unwrap_or_default(), 
                                        idx + 1, 
                                        e
                                    );
                                    continue; // 다음 라인으로 계속 진행
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("경고: 파일을 열 수 없습니다: {:?} - {}", path, e);
                        continue; // 다음 파일로 계속 진행
                    }
                }
            }
        }
    }

    Ok(())
}
