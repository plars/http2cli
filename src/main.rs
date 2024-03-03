use percent_encoding::percent_decode;
use std::borrow::Cow;
use std::env;
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use warp::reject::Reject;
use warp::Filter;

#[derive(Debug)]
struct CommandError {
    message: String,
}

impl Reject for CommandError {}

#[tokio::main]
async fn main() {
    let port = env::var("HTTP2CLI_PORT")
        .unwrap_or_else(|_| String::from("8000"))
        .parse()
        .expect("HTTP2CLI_PORT must be a number");

    let routes =
        warp::path("command")
            .and(warp::path::param())
            .and_then(|cmd: String| async move {
                let decoded: Cow<str> =
                    percent_decode(cmd.as_bytes())
                        .decode_utf8()
                        .map_err(|err| {
                            warp::reject::custom(CommandError {
                                message: format!("URL decoding failed: {}", err),
                            })
                        })?;
                println!("Running command: {}", decoded);
                let mut parts = decoded.split_whitespace();
                let command = parts.next().ok_or_else(|| {
                    warp::reject::custom(CommandError {
                        message: "Missing command".into(),
                    })
                })?;
                let args = parts.collect::<Vec<_>>();

                let output = match timeout(
                    Duration::from_secs(5),
                    Command::new(command).args(args).output(),
                )
                .await
                {
                    Ok(result) => match result {
                        Ok(output) => output,
                        Err(err) => {
                            return Err(warp::reject::custom(CommandError {
                                message: format!("Command execution failed: {}", err),
                            }))
                        }
                    },
                    Err(_) => {
                        return Err(warp::reject::custom(CommandError {
                            message: "Command timeout".into(),
                        }))
                    }
                };

                let stderr = String::from_utf8_lossy(&output.stderr);

                if output.status.success() {
                    Ok(format!(
                        "stdout: {}\nstderr: {}",
                        String::from_utf8_lossy(&output.stdout),
                        stderr
                    ))
                } else {
                    Err(warp::reject::custom(CommandError {
                        message: format!("Command failed with error: {}", stderr),
                    }))
                }
            });

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
