use std::{path::Path, process::Command};

use dunce::canonicalize;

fn main() -> std::io::Result<()> {
    {
        let status = Command::new("cbindgen")
            .arg("-o")
            .arg("io_dartcobject.h")
            .spawn()?
            .wait()?;
        if !status.success() {
            panic!("cbindgen failed")
        }
    }

    {
        let status = Command::new("dart")
            .args(vec!["run", "ffigen"])
            .current_dir(canonicalize(Path::new(".."))?)
            .spawn()?
            .wait()?;
        if !status.success() {
            panic!("ffigen failed")
        }
    }

    Ok(())
}
