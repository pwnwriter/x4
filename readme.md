<h1 align="center">x4</h1>

<p align="center">
  <strong>Execute shell commands to a server via SSH protocol</strong>
</p>

<p align="center">
  <img src="https://github.com/user-attachments/assets/db6ea484-be58-4e68-bfcf-e868291867e7" width="200" height="200" alt="Server Image">
</p>

<p align="center">
  A lightweight implementation of <code>libssh2</code> to execute shell commands on remote servers with rich configuration via json.
</p>


<div align="center">
  <a href="https://github.com/pwnwriter/x4/releases">
    <img src="https://img.shields.io/github/v/release/pwnwriter/x4?style=flat&labelColor=f38ba8&color=585b70&logo=GitHub&logoColor=white" alt="Release Version">
  </a>
  
  <a href="https://crates.io/crates/x4/">
    <img src="https://img.shields.io/crates/v/x4?style=flat&labelColor=b4befe&color=eba0ac&logo=Rust&logoColor=white" alt="Crates.io Version">
  </a>
  
  <a href="https://github.com/pwnwriter/x4/actions?query=workflow%3A%22Continuous+Deployment%22">
    <img src="https://img.shields.io/github/actions/workflow/status/pwnwriter/x4/ci.yml?style=flat&labelColor=eba0ac&color=74c7ec&label=nix-build&logo=GitHub%20Actions&logoColor=white" alt="GitHub Actions Status">
  </a>
  
  <a href="https://github.com/pwnwriter/x4/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT License">
  </a>
  
</div>

<br></br>


<!--toc:start-->
- [Features and TODOs](#features-and-todos)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Options](#options)
    - [General](#general)
- [Development](#dev)
- [Support](#support)
  <!--toc:end-->



Possible configuration values:

### Configuration Options

The configuration JSON supports the following key values for server definitions:

| Key         | Description                                                 | Example Value                  |
|-------------|-------------------------------------------------------------|--------------------------------|
| `description` | description of the server.                        | "using a private key for SSH authentication and default 22 port" |
| `name`      | Name identifier for the server.                   | "ec2"                          |
| `host`      | The hostname or IP address of the server.                  | "fawn.pwnwriter.xyz"          |
| `user`      | The username for SSH authentication.                        | "fawn"                        |
| `password`  | The method of authentication. Can be an environment variable (prefix with `env:`) or a command (prefix with `cmd:`). | "env:wolf_pass" or "cmd:pass uni/server/wolf" |
| `commands`  | An array of commands to be executed after SSH connection.   | ["ls -l", "cat /etc/hostname"] |


### Development

If you are using Nix, 

Get into the development shell 

`$ nix develop` or use 

`$ direnv allow` to enter a shell with all required deps. 

`$ nix build/run` to build and run the app. 



<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2024 <a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz<a> 🍃</a> 
