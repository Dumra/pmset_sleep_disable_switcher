use std::io;
use std::process::{Child, Command, Stdio};

fn main() {
    if cfg!(target_os = "macos") {
        let mut pmset_output_child = Command::new("pmset")
            .arg("-g")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        if let Some(pmset_output) = pmset_output_child.stdout.take() {
            let grep_output_child = Command::new("grep")
                .arg("SleepDisabled")
                .stdin(pmset_output)
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute process");

            let grep_output = grep_output_child
                .wait_with_output()
                .expect("cant wait grep result");
            pmset_output_child.wait().expect("cant wait pmset result");

            println!("Current status:");
            println!("{}", String::from_utf8(grep_output.stdout).unwrap().trim());

            let value = Command::new("sudo")
                .arg("pmset")
                .arg("-a")
                .arg("disablesleep")
                .arg("0")
                .output()
                .expect("Errorrrrrrrrr");

            print!("Value {}", String::from_utf8(value.stdout).unwrap());
        }
    } else {
        panic!("Sorry, this app is using on MacOS only!");
    };
}

fn get_pmset_result() -> Result<Child, io::Error> {
    let out = Command::new("pmset")
        .arg("-g")
        .stdout(Stdio::piped())
        .spawn()?;
    Ok(out)
}
