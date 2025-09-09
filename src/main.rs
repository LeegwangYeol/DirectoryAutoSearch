use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::time::Instant;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    // 검색할 문자열
    let target = "rasberry";

    // 현재 디렉토리 경로
    let current_dir = std::env::current_dir()?;

    let walker = WalkDir::new(current_dir).into_iter();

    // 재귀적으로 디렉토리를 탐색하고 병렬로 처리합니다.
    walker
        .filter_map(Result::ok)
        .par_bridge()
        .filter(|entry| entry.file_type().is_file())
        .for_each(|entry| {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "xml" || ext == "txt" {
                    if let Ok(file) = File::open(path) {
                        if let Ok(mmap) = unsafe { memmap2::Mmap::map(&file) } {
                            if memchr::memmem::find(&mmap, target.as_bytes()).is_some() {
                                // 메모리 매핑을 사용하면 특정 줄 번호를 찾는 것이 더 복잡해집니다.
                                // 여기서는 파일 이름만 출력합니다.
                                println!("Found in file: {:?}", path);
                            }
                        }
                    }
                }
            }
        });

    let duration = start_time.elapsed();
    println!("검색 완료. 총 소요 시간: {:?}", duration);

    Ok(())
}

