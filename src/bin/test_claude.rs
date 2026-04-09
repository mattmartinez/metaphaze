use std::process::{Command, Stdio};
use std::io::Read;

fn main() {
    let home = std::env::var("HOME").unwrap_or_default();
    let real = format!("{}/.local/bin/claude", home);

    // Test 5: capture stdout via pipe but inherit stderr
    eprintln!("=== Test 5: pipe stdout, inherit stderr ===");
    let mut child = Command::new(&real)
        .args(["-p", "say hello", "--output-format", "text"])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let mut stdout = String::new();
    child.stdout.take().unwrap().read_to_string(&mut stdout).unwrap();
    let status = child.wait().unwrap();
    eprintln!("Exit: {}", status);
    eprintln!("Stdout: {:?}", stdout);

    // Test 6: inherit everything (output goes straight to terminal)
    eprintln!("\n=== Test 6: inherit all (output to terminal) ===");
    let status = Command::new(&real)
        .args(["-p", "say hello", "--output-format", "text"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    eprintln!("Exit: {}", status);

    // Test 7: use script command to force TTY
    eprintln!("\n=== Test 7: via script -q (force TTY) ===");
    let output = Command::new("script")
        .args(["-q", "/dev/null", &real, "-p", "say hello", "--output-format", "text"])
        .output()
        .unwrap();
    eprintln!("Exit: {}", output.status);
    eprintln!("Stdout: {:?}", String::from_utf8_lossy(&output.stdout));
}
