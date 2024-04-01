use std::{
    env::home_dir,
    fs::{self, DirEntry, File},
    io::Read,
};

pub(crate) fn get_configs_at<FM, R>(s: &str, f: FM) -> Vec<R>
where
    FM: FnMut(String) -> R,
{
    read_configs_at(s)
        .flat_map(|dir| match dir {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) if file_type.is_file() => {
                    vec![std::fs::read_to_string(entry.path()).unwrap()]
                }
                Ok(file_type) if file_type.is_dir() => {
                    let mut vec = Vec::new();
                    solve_dir(entry, &mut vec);
                    vec
                }
                Err(err) => {
                    panic!("err!: {err}");
                }
                _ => Vec::new(),
            },
            Err(err) => panic!("err at file:{}", err),
        })
        .map(f)
        .collect()
}

fn solve_dir(dir: DirEntry, ret: &mut Vec<String>) {
    std::fs::read_dir(dir.path())
        .unwrap()
        .for_each(|f| match f {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) if file_type.is_file() => {
                    ret.push(std::fs::read_to_string(entry.path()).unwrap())
                }
                Ok(file_type) if file_type.is_dir() => solve_dir(entry, ret),
                Err(err) => {
                    panic!("err!: {err}");
                }
                _ => (),
            },
            Err(err) => panic!("err at file:{}", err),
        });
}

fn read_configs_at(location: &str) -> fs::ReadDir {
    let mut userhome = dirs::config_dir().expect("error on locate config dir");
    userhome.push("dawdle_todo");
    userhome.push(location);
    if std::fs::try_exists(userhome.as_path()).is_err() {
        std::fs::create_dir_all(userhome.as_path());
    }

    std::fs::read_dir(userhome.as_path()).unwrap()
}
