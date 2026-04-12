#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use oxxlplay_scenario::{CaptureRunBatchManifest, validate_batch_manifest};

const COMMANDS: &[&str] = &[
    "validate-scenario",
    "capture-run",
    "capture-run-batch",
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
                eprintln!("usage: dna-xl-play capture-run --scenario <path> [--output-dir <path>]");
                ExitCode::from(2)
            }
        },
        "capture-run-batch" => match parse_capture_run_batch_args(args.collect()) {
            Ok(options) => match run_capture_run_batch(&options) {
                Ok(()) => ExitCode::SUCCESS,
                Err(err) => {
                    eprintln!("{err}");
                    ExitCode::from(1)
                }
            },
            Err(err) => {
                eprintln!("{err}");
                eprintln!(
                    "usage: dna-xl-play capture-run-batch --manifest <path> [--output-dir <path>]"
                );
                ExitCode::from(2)
            }
        },
        other => {
            eprintln!(
                "command `{other}` remains scaffolded; stable Windows execution is currently wired through `capture-run` and `capture-run-batch`"
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

#[derive(Debug, PartialEq, Eq)]
struct CaptureRunBatchOptions {
    manifest_path: PathBuf,
    output_dir: Option<PathBuf>,
}

fn print_banner() {
    println!("dna-xl-play");
    println!("available commands: {}", COMMANDS.join(", "));
    println!("stable path: capture-run --scenario <path> [--output-dir <path>]");
    println!("batch path: capture-run-batch --manifest <path> [--output-dir <path>]");
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

fn parse_capture_run_batch_args(args: Vec<String>) -> Result<CaptureRunBatchOptions, String> {
    let mut manifest_path = None;
    let mut output_dir = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--manifest" => {
                let Some(value) = args.get(index + 1) else {
                    return Err("missing value for `--manifest`".to_owned());
                };
                manifest_path = Some(PathBuf::from(value));
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

    let Some(manifest_path) = manifest_path else {
        return Err("`--manifest` is required".to_owned());
    };

    Ok(CaptureRunBatchOptions {
        manifest_path,
        output_dir,
    })
}

fn run_capture_run_batch(options: &CaptureRunBatchOptions) -> Result<(), String> {
    let manifest_contents = fs::read_to_string(&options.manifest_path).map_err(|err| {
        format!(
            "failed to read batch manifest `{}`: {err}",
            options.manifest_path.display()
        )
    })?;
    let manifest: CaptureRunBatchManifest =
        serde_json::from_str(&manifest_contents).map_err(|err| {
            format!(
                "failed to parse batch manifest `{}` as JSON: {err}",
                options.manifest_path.display()
            )
        })?;
    validate_batch_manifest(&manifest).map_err(|err| {
        format!(
            "batch manifest `{}` failed validation: {err}",
            options.manifest_path.display()
        )
    })?;

    let script_path = resolve_script_path()?;
    let mut command = Command::new("pwsh");
    command
        .arg("-File")
        .arg(script_path)
        .arg("-BatchManifestPath")
        .arg(&options.manifest_path);

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
    use super::{
        CaptureRunBatchOptions, CaptureRunOptions, parse_capture_run_args,
        parse_capture_run_batch_args,
    };
    use std::path::PathBuf;

    #[test]
    fn parses_capture_run_arguments() {
        let options = parse_capture_run_args(vec![
            "--scenario".to_owned(),
            "docs/test-corpus/excel/xlplay_capture_values_formulae_001/scenario.json".to_owned(),
            "--output-dir".to_owned(),
            ".tmp/w006".to_owned(),
        ])
        .expect("expected args to parse");

        assert_eq!(
            options,
            CaptureRunOptions {
                scenario_path: PathBuf::from(
                    "docs/test-corpus/excel/xlplay_capture_values_formulae_001/scenario.json"
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

    #[test]
    fn parses_capture_run_batch_arguments() {
        let options = parse_capture_run_batch_args(vec![
            "--manifest".to_owned(),
            ".tmp/batch-manifest.json".to_owned(),
            "--output-dir".to_owned(),
            ".tmp/batch-output".to_owned(),
        ])
        .expect("expected batch args to parse");

        assert_eq!(
            options,
            CaptureRunBatchOptions {
                manifest_path: PathBuf::from(".tmp/batch-manifest.json"),
                output_dir: Some(PathBuf::from(".tmp/batch-output")),
            }
        );
    }

    #[test]
    fn rejects_missing_manifest_argument() {
        let err =
            parse_capture_run_batch_args(vec!["--output-dir".to_owned(), ".tmp/batch".to_owned()])
                .expect_err("expected missing manifest to fail");
        assert_eq!(err, "`--manifest` is required");
    }
}
