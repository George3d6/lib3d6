use std::process::Command;
use std::fs::File;
use std::path::Path;


fn random_name() -> String {
    let file = File::open(Path::new("/dev/random")).unwrap();
    let mut random_name = String::new();
    file.read_to_string(&mut random_name, 12);
    return random_name
}

pub fn launch_tmux() {
    let name = random_name();
    let output = Command::new("tmux")
        .arg("new").arg("-s").arg(&name)
        .output().expect(&format!("Could not start tmux {}", &name));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmux_launch() {
        launch_tmux();
    }
}