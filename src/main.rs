use aws_sdk_mwaa::Client;
use aws_config::from_env;
use clap::{Arg, App};
use aws_types::region::Region;
use std::io::{self, Write};
use tokio::task::JoinHandle;
use std::process::Command;
use std::collections::HashMap;
use std::time::Instant;
use tokio::time::{sleep, Duration};
// use std::sync::Arc;
// use tokio::sync::Mutex;


#[tokio::main]
async fn main() {
    let matches = App::new("List MWAA Environments")
        .version("1.0")
        .author("Your Name")
        .arg(Arg::with_name("PROFILE")
             .short("p")
             .long("profile")
             .help("Sets the AWS profile to use")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("REGION")
             .short("r")
             .long("region")
             .help("Sets the AWS region")
             .required(true)
             .takes_value(true))
        .get_matches();

    let profile = matches.value_of("PROFILE").unwrap().to_string();
    let region_str = matches.value_of("REGION").unwrap().to_string();
    let region = Region::new(region_str.clone());
    // let exit_flag = Arc::new(Mutex::new(false));
    
    let provider = from_env()
        .profile_name(&profile)
        .region(region)
        .load()
        .await;
    

    let client = Client::new(&provider);
    let mut env_versions: HashMap<String, String> = HashMap::new();

    let mut handles: Vec<JoinHandle<Result<(), String>>> = Vec::new();
    // let mut count = 0;
    let update_start_times: HashMap<String, Instant> = HashMap::new();
    // let mut failed_envs: Vec<String> = Vec::new();
    let mut envs_to_check: Vec<String> = Vec::new();
    
    
    match client.list_environments().send().await {
        Ok(response) => {
            if let Some(envs) = response.environments {
                for env in &envs {
                    println!("{}", env);
                    if ask_update() {
                        println!("Updating {} ...", env);
                        let future = run_update_command(env.clone(), profile.clone(), region_str.clone());

                        let handle = tokio::spawn(future);
                        handles.push(handle);
                        envs_to_check.push(env.clone());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing MWAA environments: {}", e);
        }
    }
    
     if handles.is_empty() {
    println!("No environments chosen for update. Exiting.");
    return;
} 
else {
    println!("Please wait while the following environments are updating: {:?}", envs_to_check);
}

tokio::time::sleep(Duration::from_secs(10*60)).await;

while !envs_to_check.is_empty() {
        let mut next_check: Vec<String> = Vec::new();

        for env in &envs_to_check {
            match check_update_status(&client, env).await {
            Ok((status, version)) => {
            if status == "UPDATING" {
                next_check.push(env.clone());
            println!("{} is still updating. Checking again in 3 minutes...", env);
            } else {
            let elapsed_time = match update_start_times.get(env) {
            Some(time) => time.elapsed(),
            None => {
        eprintln!("Error: Start time not found for environment {}", env);
        Duration::new(0, 0) // or some default value
    }
};
            if let Some(ver) = version {
                env_versions.insert(env.clone(), ver);
            }
            println!("Update for {} is done! Status: {}. Took: {:.2?} minutes.", env, status, elapsed_time.as_secs() / 60);
        }
    },
    Err(e) => {
        eprintln!("Error checking update status for {}: {}", env, e);
    }
}

        }

        envs_to_check = next_check;
       
        
        if !envs_to_check.is_empty() {
            sleep(Duration::from_secs(3 * 60)).await;
        
        }
    }

    println!("All updates are completed!");

for (env, start_time) in &update_start_times {
    let elapsed_time = start_time.elapsed();
    let unknown_version = "Unknown".to_string();
    let version = env_versions.get(env).unwrap_or(&unknown_version);

    println!("Environment: {}, Version: {}, Updated in: {:.2?} minutes.", env, version, elapsed_time.as_secs() / 60);
}
    
}
// #[allow(dead_code)]
// fn get_environment_status(_client: &Client, env_name: &str, profile: &str, region: &str) -> String {
//     // Using AWS CLI to get the MWAA environment details
//     let output = Command::new("aws")
//     .args(&[
//         "mwaa",
//         "get-environment",
//         "--name",
//         env_name,
//         "--profile",
//         profile,
//         "--region",
//         region
//     ])
//     .output()
//     .expect("Failed to execute command");

//     if output.status.success() {
//         let result_str = String::from_utf8_lossy(&output.stdout);
//         // Assuming the output is in JSON format, you'll need to parse the JSON to get the status.
//         // For a more robust solution, consider using a JSON parsing library like `serde_json`.
//         if let Some(status_line) = result_str.lines().find(|line| line.contains("\"Status\":")) {
//             let status = status_line.split(":").nth(1).unwrap_or("\"Unknown\"").trim().trim_matches('\"').to_string();
//             return status;
//         }
//     } else {
//         eprintln!("Error fetching environment status: {}", String::from_utf8_lossy(&output.stderr));
//     }

//     "Unknown".to_string()
// }



fn ask_update() -> bool {
    print!("Do you want to update this environment? (y/n) ");
    io::stdout().flush().unwrap();  // Flush here to ensure the output appears immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "y" | "Y" | "yes" | "Yes" | "YES" | "sure" | "yep" => true,
        _ => false,
    }
}

async fn check_update_status(client: &Client, env_name: &str) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    let resp = client.get_environment().name(env_name).send().await?;
    let status = match &resp.environment {
        Some(env) => match &env.status {
            Some(status) => format!("{:?}", status),
            None => "Unknown".to_string(),
        },
        None => "Unknown".to_string(),
    };
    let version = resp.environment.and_then(|env| env.airflow_version.clone());

    Ok((status, version))
}


async fn run_update_command(env_name: String, profile: String, region: String) -> Result<(), String> {
    let output = Command::new("aws")
        .args(["mwaa", "update-environment", "--name", &env_name, "--profile", &profile, "--region", &region])
        .output()
        .expect("Failed to run 'update-environment' command");

    if output.status.success() {
        println!("Successfully initiated update command for environment {}.", env_name);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        
        if error_message.contains("Environments with UPDATING status must complete previous operation") {
            println!("Environment {} is already in the process of updating. Please wait for it to finish before trying again.", env_name);
        } else {
            eprintln!("Error running update command for environment {}: {}.\n Press [ENTER] to continue...", env_name, error_message);
            return Err(format!("Failed to update environment: {}", env_name));
        }
    }

    Ok(())
}

