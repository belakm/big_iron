use std::process::Command;

fn ohlc_to_string(ohlc: &(f64, f64, f64, f64)) -> String {
    format!(
        "O:{:.2}, H:{:.2}, L:{:.2}, C:{:.2}",
        ohlc.0, ohlc.1, ohlc.2, ohlc.3
    )
}

fn run_r_script(path_to_script: &str, arguments: Option<&str>) -> Result<String, String> {
    // Set the Rscript command and the path to the R script
    let rscript_cmd = "Rscript";
    let output: std::process::Output;

    match arguments {
        None => {
            // Execute the Rscript command with the input parameters
            let output = Command::new(rscript_cmd)
                .arg(path_to_script)
                .stderr(std::process::Stdio::piped())
                .output()
                .expect("Failed to execute Rscript command");
        }
        Some(argument) => {
            // Execute the Rscript command with the input parameters
            let output = Command::new(rscript_cmd)
                .arg(path_to_script)
                .arg(argument)
                .stderr(std::process::Stdio::piped())
                .output()
                .expect("Failed to execute Rscript command");
        }
    }
    // Check if the R script returned a success code
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        let status = output.status.to_string();
        Err(status)
    }
}

pub fn create_model() -> Option<String> {
    let output = run_r_script("models/create.R", None);
    match output {
        Ok(_) => Some(String::from("Ok")),
        Err(err) => {
            // TODO: Log error
            None
        }
    }
}

pub fn signal(ohlc: &(f64, f64, f64, f64)) -> Option<String> {
    // Set the input parameters to the R script
    let ohlc = ohlc_to_string(ohlc);
    let output = run_r_script("models/predict.R", Some(&ohlc));
    match output {
        Ok(signal) => Some(signal),
        Err(err) => {
            // TODO: Log error
            None
        }
    }
}
