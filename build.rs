use std::process::Command;

const MANAGER: &str = "bunx";

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.png");
    let output = {
        if let Ok(out) = Command::new("sh")
            .arg("-c")
            .arg(MANAGER.to_string() + " --yes -- nwlink@0.0.19 png-nwi assets/icon.png target/icon.nwi")
            .output()
        {
            out
        } else {
            panic!("Your OS is not supported! If you're using Windows, please compile in WSL.");
        }
    };
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

}
