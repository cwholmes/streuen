use std::{
    io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

pub struct ProjectConfig {
    _project_dirs: Option<ProjectDirs>,
    data_dir: PathBuf,
}

impl ProjectConfig {
    pub fn new() -> io::Result<Self> {
        let project_dirs = ProjectDirs::from("com", "github", "streuen-chat");

        let data_dir = Self::get_data_dir(&project_dirs)?;

        std::fs::create_dir_all(data_dir.clone())?;

        Ok(Self {
            _project_dirs: project_dirs,
            data_dir,
        })
    }

    fn get_data_dir(project_dirs: &Option<ProjectDirs>) -> io::Result<PathBuf> {
        let data_folder = std::env::var("STREUEN_CHAT_DATA".to_string())
            .ok()
            .map(PathBuf::from);
        let directory = if let Some(s) = data_folder.clone() {
            s
        } else if let Some(proj_dirs) = project_dirs {
            proj_dirs.data_local_dir().to_path_buf()
        } else {
            PathBuf::from(".").join(".data").join("streuen-chat")
        };
        std::fs::create_dir_all(directory.clone())?;
        Ok(directory)
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
}
