use crate::repository::Repositories;
use crate::repository::Repository;
use failure::format_err;
use failure::Error;
use std::path::Path;
use std::process::Command;

pub struct GitDirectory;

impl GitDirectory {
    pub fn get_repositories<S: Into<String>>(path: S) -> Result<Repositories, Error> {
        let p = path.into();
        log::debug!("git repository root {}", p);
        if !GitDirectory::exists_plugin_manager(&p)? {
            return Ok(vec![]);
        }
        GitDirectory::create_repositories_struct(&p)
    }

    fn exists_plugin_manager<S: Into<String>>(path: S) -> Result<bool, Error> {
        let path = path.into();
        if path.is_empty() {
            return Ok(false);
        }
        if !std::path::Path::new(&path).exists() {
            return Ok(false);
        }
        log::debug!("repositories exists");
        Ok(true)
    }

    fn get_url<P: AsRef<Path>>(path: P) -> Result<String, Error> {
        let cmd = format!(
            r##"cd {} && git config --get remote.origin.url"##,
            path.as_ref().to_str().ok_or(format_err!("convert error"))?
        );
        log::debug!("command to get tpm remote origin: {}", cmd);
        let output = Command::new("sh").arg("-c").arg(cmd).output()?;
        log::debug!("process exited with: {}", output.status);
        let stdout = output.stdout;
        let mut url = String::from_utf8(stdout)?;
        let len = url.trim_end_matches(&['\r', '\n'][..]).len();
        url.truncate(len);
        Ok(url)
    }

    fn create_repositories_struct<S: Into<String>>(path: S) -> Result<Repositories, Error> {
        let mut r = vec![];
        for entry in std::fs::read_dir(path.into())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let url = GitDirectory::get_url(&path)?;
                let repo = Repository {
                    uri: url,
                    dir: path
                        .to_str()
                        .ok_or(format_err!("convert error"))?
                        .to_string(),
                };
                r.push(repo);
            }
        }
        Ok(r)
    }
}