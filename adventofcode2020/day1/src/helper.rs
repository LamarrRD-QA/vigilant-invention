use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
// use itertools::Itertools;

pub fn load_input() -> Result<Vec<u32>, Box<dyn Error>> {
    let mut line_err_state = Ok(());
    let mut parse_err_state = Ok(());

    let input_path = get_input_path()?;
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let list_numbers: Vec<u32> = reader
        .lines()
        .scan(&mut line_err_state, |line_err_state, res| match res {
            Ok(o) => Some(o),
            Err(e) => {
                **line_err_state = Err(e);
                None
            }
        })
        .map(|x| x.parse::<u32>())
        .scan(&mut parse_err_state, |parse_err_state, res| match res {
            Ok(o) => Some(o),
            Err(e) => {
                **parse_err_state = Err(e);
                None
            }
        })
        .collect();

    line_err_state?;
    parse_err_state?;
    Ok(list_numbers)
}

fn get_input_path() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable should be in a directory");
    let mut dir = dir.join("inputs");
    dir.push("day1.txt");
    Ok(dir)
}
