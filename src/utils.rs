use std::{
    env::{self, VarError},
    fs,
    process::Command,
};

#[derive(Debug)]
pub enum GetInputError {
    IoError(std::io::Error),
    VarError(VarError),
    FileIsEmpty(String),
    HttpError(String),
}

#[derive(Debug)]
pub enum PreparationDone {
    FileExists(String),
    FileCreatedWithData(String),
}

pub fn read_file(day: u8, puzzle: Option<u8>, run_sample: bool) -> String {
    let filename = format!(
        "src/day{}/input{}{}.txt",
        day,
        puzzle.map_or("".into(), |p| p.to_string()),
        if run_sample { "_sample" } else { "" }
    );

    fs::read_to_string(&filename).expect(format!("file {} not found", filename).as_str())
}

pub fn prepare_for(day: &str) -> Result<PreparationDone, GetInputError> {
    let output_dir = format!("src/day{}", day);
    let output_file = format!("{}/input.txt", output_dir);

    if fs::metadata(&output_dir).is_err() {
        fs::create_dir(output_dir).map_err(|e| GetInputError::IoError(e))?;
    }

    if check_file(&output_file).is_ok() {
        return Ok(PreparationDone::FileExists(output_file));
    }

    let session = env::var("AOC_SESSION").map_err(|e| GetInputError::VarError(e))?;
    let output = Command::new("wget")
        .arg(format!("--header=Cookie: session={}", session))
        .arg(format!("https://adventofcode.com/2024/day/{}/input", day))
        .arg("-O")
        .arg(&output_file)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                check_file(&output_file).map(|_| PreparationDone::FileCreatedWithData(output_file))
            } else {
                Err(GetInputError::HttpError(format!(
                    "wget command failed.\nstdout: {}\nstderr: {}",
                    String::from_utf8_lossy(&result.stdout),
                    String::from_utf8_lossy(&result.stderr)
                )))
            }
        }
        Err(err) => Err(GetInputError::IoError(err)),
    }
}

fn check_file(filename: &str) -> Result<(), GetInputError> {
    fs::metadata(filename)
        .map_err(|e| GetInputError::IoError(e))
        .and_then(|metadata| {
            if metadata.len() == 0 {
                Err(GetInputError::FileIsEmpty(filename.into()))
            } else {
                Ok(())
            }
        })
}
