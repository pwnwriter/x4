# How to Deploy Your Website on a Server Using X4

In this guide, I’ll walk you through the process of deploying a website on a server using **X4**.

## Requirements

Before we begin, ensure you have the following:

- A GitHub repository containing your project.
- Access to a server that you can SSH into using either a password or an SSH key pair.

## What is X4?

**X4** is a tool that allows you to execute shell commands on a server via SSH. In this guide, we’ll use GitHub CI to trigger deployments through X4 every time there’s a change in the repository.

## Setting Up X4 Configuration

The configuration for X4 is done in a JSON format. Below is an example of how your `x4.json` file should look:

```json
{
  "servers": [
    {
      "description": "Deploy Node.js website using X4",
      "name": "AWS EC2 Instance",
      "host": "fawn.pwnwriter.xyz",
      "user": "fawn",
      "password": "env:WOLF_PASS",
      "commands": [
        "cd ~/x4-website && ./deployer.sh"
      ]
    }
  ]
}
```

### Explanation of the Configuration
- **description**: A brief note about what the configuration is for.
- **name**: The name of the server you are deploying to.
- **host**: The server’s hostname or IP address.
- **user**: Your SSH username.
- **password**: The password for SSH access, which can be set as an environment variable for security.
- **commands**: A list of shell commands to execute after SSHing into the server.

## Checking the Pipeline Schema

X4 provides a command to validate your configuration schema. To ensure everything is set up correctly, run:

```bash
x4 --check-pipeline <path/to/your/schema>
```

![Check Pipeline Schema](https://github.com/user-attachments/assets/80e1d15d-290e-40af-a639-bb89b126aa30)

## Setting Up GitHub Secrets

Since our configuration uses an environment variable for the password (`WOLF_PASS`), we need to set this up in GitHub Secrets:

1. Navigate to your GitHub repository.
2. Go to **Settings** > **Secrets and Variables** > **Actions** > **Repository Secret**.
3. Create a new secret named `WOLF_PASS` and set its value to your server's password.

![GitHub Secrets](https://github.com/user-attachments/assets/a102c373-bb64-49a7-ab22-74aec4d1bae6)

## Configuring GitHub Actions

Next, we will create a GitHub Action that triggers the deployment whenever code is pushed to the `main` branch. Below is an example of a GitHub Actions workflow configuration:

```yaml
name: Install and Run X4

on:
  push:
    branches:
      - main  # Change this to your target branch if needed

jobs:
  install_and_run_x4:
    runs-on: ubuntu-latest

    env:
      WOLF_PASS: ${{ secrets.WOLF_PASS }}  # Set WOLF_PASS from secrets

    steps:
      - name: Checkout repository
        uses: actions/checkout@v2

      - name: Install X4
        run: |
          # Download the X4 binary
          wget -O x4.tar.gz "https://github.com/pwnwriter/x4/releases/download/v0.1.0/x4-0.1.0-x86_64-unknown-linux-gnu.tar.gz"
          
          # Extract the downloaded tarball
          tar -xzf x4.tar.gz --strip-components=1
          
          # Make the X4 binary executable
          chmod +x ./x4

      - name: Run X4
        run: |
          ./x4 -p x4.json
```

### Breakdown of the Action
- **Install X4**: Downloads and sets up the X4 binary.
- **Run X4**: Executes the deployment using the defined configuration.

## Creating the Deployer Script

We need a script that manages the server for our app. This script, `deployer.sh`, will handle dependencies, build the application, and manage the server process:

```bash
#!/usr/bin/env bash

pnpm install && pnpm run build

# Find and kill the process running on port 1337 if it exists
pid=$(ss -lptn 'sport = :1337' | grep -oP '(?<=pid=)\\d+')
if [ -n "$pid" ]; then
    kill "$pid"
fi

# Start the server in the background
nohup pnpm preview > preview.log 2>&1 &

disown
```

### Key Steps in the Script
1. **Install dependencies and build**: Uses `pnpm` to install and build your project.
2. **Manage processes**: Finds and kills any existing server process on port `1337`.
3. **Start the server**: Runs the server in the background and logs output.

## File Structure

Ensure your project has the following file structure:

```
/your-repo
├── .github
│   └── workflows
│       └── deploy.yml
├── x4.json
└── deployer.sh
```

<img width="566" alt="Screenshot 2024-10-05 at 7 43 26 PM" src="https://github.com/user-attachments/assets/42205c49-80af-42ab-b1cf-525b498a4eeb">


## Final Steps

Once you've set everything up, simply push your code to the `main` branch. This will trigger the workflow, deploying your website seamlessly using X4!

![Deployment Workflow](https://github.com/user-attachments/assets/5995ab25-c567-4ecf-897f-032a1268a3eb)

## Conclusion

You’ve successfully set up an automated deployment process using X4! If you found X4 helpful, consider giving it a star ⭐ on [GitHub](https://github.com/pwnwriter/x4). Happy deployment :Dizzy: 
