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

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "none" {
        // Check if arm-none-eabi-gcc is available
        let gcc_available = Command::new("arm-none-eabi-gcc").arg("--version").output().is_ok();
        if !gcc_available {
            eprintln!("Warning: arm-none-eabi-gcc not found. Skipping C compilation (ok for cargo check).");
            return;
        }

        unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };

        let nwlink_flags = String::from_utf8(
            Command::new(MANAGER)
                .args(["--yes", "--", "nwlink@0.0.19", "eadk-cflags"])
                .output()
                .expect("Failed to get nwlink eadk-cflags")
                .stdout,
        )
        .expect("Invalid UTF-8 in nwlink flags");

        let mut build = cc::Build::new();
        build.file("src/libs/storage.c");
        build.flag("-std=c99");
        build.flag("-Os");
        build.flag("-Wall");
        build.flag("-ggdb");
        build.warnings(false);

        for flag in nwlink_flags.split_whitespace() {
            build.flag(flag);
        }

        build.compile("storage_c");
    }
}
