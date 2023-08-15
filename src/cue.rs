use std::{env, path::PathBuf, fs, io, process::Command};

pub fn export_as_json(cuepath: &PathBuf, jsonpath: &PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(cuepath)? {
        let entry = entry?;
        let p = entry.path();
        if let Some(extension) = p.extension() {
            if extension == "cue" {
                let mut pp = p.clone();
                pp.set_extension("");
                if let Some(filename_ostr) = pp.file_name() {
                    if let Some(filename) = filename_ostr.to_str() {                      
                       let mut json_outfile = jsonpath.clone();
                       json_outfile.push(filename);
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
        }
    }
    Ok(())
}
