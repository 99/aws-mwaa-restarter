# MWAA Restarter

> Restart environments in Amazon MWAA with ease.


## 📌 Table of Contents

- [Introduction](#-introduction)
- [Requirements](#-requirements)
- [Setup](#-setup)
- [Usage](#-usage)
- [License](#-license)

## 🚀 Introduction

`mwaa-restarter` is a command-line utility that provides a streamlined approach to list and selectively update environments in Amazon Managed Workflows for Apache Airflow (MWAA).

## 📋 Requirements

- Rust
- AWS SDK for Rust
- AWS CLI (configured with the necessary credentials)

## 🛠 Setup

1. Clone the repository:

   ```sh
   git clone https://github.com/99/mwaa-restarter.git
   ```

2. Navigate to the directory and build the project:

   ```sh
   cd mwaa-restarter
   cargo build --release
   ```

3. Copy the built binary to a location in your PATH.

## 💡 Usage

Run the tool with the required flags:

```sh
cargo run -- -p [AWS_PROFILE] -r [AWS_REGION]
```

Replace `[AWS_PROFILE]` and `[AWS_REGION]` with your desired AWS CLI profile and region, respectively.

### Running example 
```bash
MyAirflowEnvironment
Do you want to update this environment? (y/n) y
Updating MyAirflowEnvironment ...
MyAirflowEnvironment2
Do you want to update this environment? (y/n) y
Updating MyAirflowEnvironment2...
MyAirflowEnvironment3
Do you want to update this environment? (y/n) y
Updating MyAirflowEnvironment3 ...
Successfully initiated update command for environment MyAirflowEnvironment
Successfully initiated update command for environment MyAirflowEnvironment2
Successfully initiated update command for environment MyAirflowEnvironment3

Please wait while the following environments are updating: ["MyAirflowEnvironment", "MyAirflowEnvironment2", "MyAirflowEnvironment3"]
```

```bash
MyAirflowEnvironment
Do you want to update this environment? (y/n) n
MyAirflowEnvironment2
Do you want to update this environment? (y/n) n
MyAirflowEnvironment3
Do you want to update this environment? (y/n) n
No environments chosen for update. Exiting.
```

## ⚖️ License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/99/aws-mwaa-restarter/blob/main/LICENSE) file for details.
