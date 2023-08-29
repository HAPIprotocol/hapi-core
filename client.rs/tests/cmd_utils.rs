use std::process::Command;

#[derive(Debug)]
pub struct CmdOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn ensure_cmd(command: &mut Command) -> anyhow::Result<()> {
    let output = command.output();

    println!(
        "Exec: {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .map(|s| format!("\"{}\"", s.to_string_lossy()))
            .collect::<Vec<_>>()
            .join(" ")
    );

    if let Err(e) = output {
        panic!("Failed to execute command: {e}");
    }

    let output = output.unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stderr.trim().is_empty() {
        println!("STDERR:\n{stderr}");
    }
    if !stdout.trim().is_empty() {
        println!("STDOUT:\n{stdout}");
    }

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to execute command {:?}", command));
    }

    Ok(())
}

pub fn wrap_cmd(command: &mut Command) -> anyhow::Result<CmdOutput> {
    let output = command.output()?;

    println!(
        "Exec: {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .map(|s| format!("\"{}\"", s.to_string_lossy()))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(CmdOutput {
        success: output.status.success(),
        stdout: stdout.trim().to_owned(),
        stderr: stderr.trim().to_owned(),
    })
}
