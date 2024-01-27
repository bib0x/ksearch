use std::{env, fs, io, path::Path, path::PathBuf, process::Command};

use rayon::prelude::*;

fn is_cue_file(path: &PathBuf) -> bool {
    match path.extension() {
        Some(extension) => extension == "cue",
        None => false,
    }
}

pub fn export_as_json(cuepath: &PathBuf, jsonpath: &PathBuf) -> io::Result<()> {
    let entries = fs::read_dir(&cuepath)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.par_iter().for_each(|path| {
        if is_cue_file(&path) {
            let mut tmp_path = path.clone();
            tmp_path.set_extension("");
            let filename_ostr = tmp_path.file_name().unwrap();
            if let Some(filename) = filename_ostr.to_str() {
                let mut json_outfile = Path::new(jsonpath).join(filename);
                json_outfile.set_extension("json");
                let _ = env::set_current_dir(&cuepath).is_ok();
                Command::new("cue")
                    .arg("export")
                    .arg(path.display().to_string())
                    .arg("-o")
                    .arg(json_outfile.display().to_string())
                    .arg("-f")
                    .status()
                    .expect("failed to execute cue command export");
            }
        }
    });

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
