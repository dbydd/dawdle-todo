use std::{
    env::home_dir,
    fs::{self, File},
    io::Read,
};
pub(crate) fn get_configs_at(location: &str) -> fs::ReadDir {
    let mut userhome = dirs::config_dir().expect("error on locate config dir");
    userhome.push("dawdle_todo");
    userhome.push(location);
    if std::fs::try_exists(userhome.as_path()).is_err() {
        std::fs::create_dir_all(userhome.as_path());
    }

    std::fs::read_dir(userhome.as_path()).unwrap()
}
