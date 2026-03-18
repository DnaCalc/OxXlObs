#![forbid(unsafe_code)]

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const COMMANDS: &[&str] = &[
    "validate-scenario",
    "capture-run",
    "fingerprint-env",
    "emit-bundle",
    "emit-diff-seed",
    "validate-handoff",
];

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_banner();
        return ExitCode::SUCCESS;
    };

    match command.as_str() {
        "--help" | "-h" | "help" => {
            print_help();
            ExitCode::SUCCESS
        }
        "capture-run" => match parse_capture_run_args(args.collect()) {
            Ok(options) => match run_capture_run(&options) {
                Ok(()) => ExitCode::SUCCESS,
                Err(err) => {
                    eprintln!("{err}");
                    ExitCode::from(1)
                }
            },
            Err(err) => {
                eprintln!("{err}");
                eprintln!("usage: dna-xl-obs capture-run --scenario <path> [--output-dir <path>]");
                ExitCode::from(2)
            }
        },
        other => {
            eprintln!(
                "command `{other}` remains scaffolded; stable Windows execution is currently wired through `capture-run`"
            );
            ExitCode::from(2)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CaptureRunOptions {
    scenario_path: PathBuf,
    output_dir: Option<PathBuf>,
}

fn print_banner() {
    println!("dna-xl-obs");
    println!("available commands: {}", COMMANDS.join(", "));
    println!("stable path: capture-run --scenario <path> [--output-dir <path>]");
}

fn print_help() {
    print_banner();
}

fn parse_capture_run_args(args: Vec<String>) -> Result<CaptureRunOptions, String> {
    let mut scenario_path = None;
    let mut output_dir = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--scenario" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("missing value for `--scenario`".to_owned());
                };
                scenario_path = Some(PathBuf::from(value));
                index += 2;
            }
            "--output-dir" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("missing value for `--output-dir`".to_owned());
                };
                output_dir = Some(PathBuf::from(value));
                index += 2;
            }
            unexpected => {
                return Err(format!("unexpected argument `{unexpected}`"));
            }
        }
    }

    let Some(scenario_path) = scenario_path else {
        return Err("`--scenario` is required".to_owned());
    };

    Ok(CaptureRunOptions {
        scenario_path,
        output_dir,
    })
}

fn run_capture_run(options: &CaptureRunOptions) -> Result<(), String> {
    let script_path = resolve_script_path()?;
    let mut command = Command::new("pwsh");
    command
        .arg("-File")
        .arg(script_path)
        .arg("-ScenarioPath")
        .arg(&options.scenario_path);

    if let Some(output_dir) = &options.output_dir {
        command.arg("-OutputDir").arg(output_dir);
    }

    let status = command
        .status()
        .map_err(|err| format!("failed to start PowerShell Excel bridge: {err}"))?;

    if status.success() {
        return Ok(());
    }

    match status.code() {
        Some(code) => Err(format!("PowerShell Excel bridge exited with code {code}")),
        None => Err("PowerShell Excel bridge exited without a status code".to_owned()),
    }
}

fn resolve_script_path() -> Result<PathBuf, String> {
    let candidates = [
        PathBuf::from("scripts").join("invoke-excel-observation.ps1"),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("scripts")
            .join("invoke-excel-observation.ps1"),
    ];

    candidates
        .into_iter()
        .find(|candidate| candidate.is_file())
        .ok_or_else(|| "could not locate scripts/invoke-excel-observation.ps1".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{CaptureRunOptions, parse_capture_run_args};
    use std::path::PathBuf;

    #[test]
    fn parses_capture_run_arguments() {
        let options = parse_capture_run_args(vec![
            "--scenario".to_owned(),
            "docs/test-corpus/excel/xlobs_capture_values_formulae_001/scenario.json".to_owned(),
            "--output-dir".to_owned(),
            ".tmp/w006".to_owned(),
        ])
        .expect("expected args to parse");

        assert_eq!(
            options,
            CaptureRunOptions {
                scenario_path: PathBuf::from(
                    "docs/test-corpus/excel/xlobs_capture_values_formulae_001/scenario.json"
                ),
                output_dir: Some(PathBuf::from(".tmp/w006")),
            }
        );
    }

    #[test]
    fn rejects_missing_scenario_argument() {
        let err = parse_capture_run_args(vec!["--output-dir".to_owned(), ".tmp/w006".to_owned()])
            .expect_err("expected missing scenario to fail");
        assert_eq!(err, "`--scenario` is required");
    }
}
