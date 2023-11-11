use std::{env, fs, io, path::Path, path::PathBuf, process::Command};

fn is_cue_file(path: &PathBuf) -> bool {
    match path.extension() {
        Some(extension) => extension == "cue",
        None => false,
    }
}

pub fn export_as_json(cuepath: &PathBuf, jsonpath: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(cuepath)? {
        let entry = entry?;
        let p = entry.path();
        if is_cue_file(&p) {
            let mut tmp_path = p.clone();
            tmp_path.set_extension("");
            // Safe to unwrap cause we checked that we got a file with a cue extension previously
            let filename_ostr = tmp_path.file_name().unwrap();
            if let Some(filename) = filename_ostr.to_str() {
                let mut json_outfile = Path::new(jsonpath).join(filename);
                json_outfile.set_extension("json");
                let _ = env::set_current_dir(&cuepath).is_ok();
                Command::new("cue")
                    .arg("export")
                    .arg(p.display().to_string())
                    .arg("-o")
                    .arg(json_outfile.display().to_string())
                    .arg("-f")
                    .status()
                    .expect("failed to execute cue command export");
            }
        }
    }
    Ok(())
}

pub fn list_fullpath(cuepath: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(cuepath)? {
        let entry = entry?;
        let p = entry.path();
        if is_cue_file(&p) {
            println!("{}", p.display());
        }
    }
    Ok(())
}
