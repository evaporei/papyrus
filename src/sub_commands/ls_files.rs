use crate::fs::{FileSystem, Fs};

pub fn execute(fs: &mut FileSystem, stage: bool) -> Result<String, String> {
    if !stage {
        return Err("fatal: ls-files without --stage is not implemented yet".to_string());
    }

    Ok("nothing".to_string())
}
