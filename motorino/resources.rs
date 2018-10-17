extern crate image;
use self::image::{DynamicImage};

use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::CString;


pub struct ResourceLoader {
    res_root_path: PathBuf
}

impl ResourceLoader {
    pub fn from_relative_path(path: &Path) -> Result<ResourceLoader, String> {
        let exe_file = ::std::env::current_exe()
            .map_err(|_| "Unable to get current exe")?;

        let exe_path = exe_file.parent()
            .ok_or("Unable to get exe path")?;

        Ok(ResourceLoader {
            res_root_path: exe_path.join(path)
        })
    }

    pub fn to_real_path(&self, path: &Path) -> PathBuf {
        return self.res_root_path.join(path);
    }

     pub fn load_cstring(&self, resource_name: &str) -> Result<CString, String> {
        let filename = self.res_root_path.join(resource_name);
        let contents = fs::read_to_string(&filename).
            map_err(|_| format!("Unable to read file {}", filename.display()))?;
        let ret_val = CString::new(contents.as_bytes()).
            map_err(|_| "Unable to convert file contents")?;
        return Ok(ret_val);
    }

    pub fn load_image(&self, resource_name: &str) -> Result<DynamicImage, String> {
        let filename = self.res_root_path.join(resource_name);
        let img = image::open(&filename).
            map_err(|_| format!("Unable to read file {}", filename.display()))?;
        return Ok(img);
    }
}