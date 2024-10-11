# KonnectCTL

![Build Status](https://github.com/rajeevramani/konnect-cli/actions/workflows/rust.yml/badge.svg)

## Overview

KonnectCTL is a command-line interface (CLI) tool designed to interact with Kong's Konnect API. It provides a streamlined way to manage various aspects of your Konnect configuration, including API products, versions, specifications, and control planes.

This project is a result of me trying to learn `rust`.

The goal of the project was to ofcourse learn rust but also to try and create a framework where I can incrementally improve the cli to add more controls.

This particular repo is not the original. I created a copy because my original repo was messy. I realised along with Rust what got exposed how little I knew git.

I was also trying to model this around kubectl. 

### Key Features

- Manage API Products (create, read, update, delete)
- Handle API Product Versions
- Manage API Specifications
- Create and manage Control Planes
- Support for different Konnect regions (US, EU, AU)

### To be built

- [ ] Add more Konnect cli cability.
- [ ] Clean up the code a lot more.
- [ ] Improve test cases.

## Installation

### Prerequisites

- Rust 1.54 or later
- Cargo (usually comes with Rust)

### Using Cargo

```sh
cargo install konnectctl
```

### Building from Source

1. Clone the repository:
   ```sh
   git clone https://github.com/rajeevkong/konnect-cli.git
   cd konnect-cli
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. The executable will be available in `target/release/konnectctl`

## Configuration

KonnectCTL uses environment variables for configuration. Copy the `.env.copy` file to `.env` and modify it:

```sh
cp .env.copy .env
```

Edit the `.env` file and set the following variables:

- `KONNECT_PAT`: Your [Konnect Personal Access Token](https://docs.konghq.com/konnect/api/#authentication)

- `KONNECT_REGION`: Your Konnect region (`us`, `eu`, or `au`)

## Usage

Here are some example commands:

```sh
# Get all API products
konnectctl get api-product

# Create an API product
konnectctl create api-product -n "My API" -d "My API description"

# Create a control plane
konnectctl create control-plane --name "My Control Plane" --cluster-type CLUSTER_TYPE_CONTROL_PLANE

# Get help
konnectctl --help
```

For more detailed usage instructions, run `konnectctl <command> --help`.

## Guide: Adding a New CLI Command to KonnectCTL

This guide will walk you through the process of adding a new CLI command to KonnectCTL. We'll use the example of adding a "list users" command.

### Step 1: Update the CLI Command Structure

1. Open `src/cli/main_cli.rs`
2. Add the new command to the appropriate enum (in this case, `GetSubCommand`):

```rust
pub enum GetSubCommand {
    // ... existing subcommands ...
    /// List Konnect users
    ListUsers(ListUsers),
}
```

### Step 2: Create the CLI Command Structure

1. Create a new file `src/cli/list_users_cli.rs`
2. Define the command structure:

```rust
use clap::Args;

#[derive(Debug, Args)]
pub struct ListUsers {
    #[arg(short, long)]
    /// Filter users by email
    pub email: Option<String>,

    #[arg(short, long)]
    /// Maximum number of users to return
    pub limit: Option<u32>,
}
```

3. Update `src/cli/mod.rs` to include the new module:

```rust
pub mod list_users_cli;
```

### Step 3: Implement the API Request

1. Create a new file `src/api/list_users.rs`
2. Implement the request structure and `Executable` trait:

```rust
use serde::{Deserialize, Serialize};
use crate::api::api_utils::Executable;
use crate::cli::list_users_cli::ListUsers;

#[derive(Debug, Serialize)]
pub struct ListUsersRequest {
    email: Option<String>,
    limit: Option<u32>,
}

impl ListUsersRequest {
    pub fn new(args: ListUsers) -> Self {
        ListUsersRequest {
            email: args.email,
            limit: args.limit,
        }
    }
}

impl Executable for ListUsersRequest {
    type Response = ListUsersResponse;
}

#[derive(Debug, Deserialize)]
pub struct ListUsersResponse {
    data: Vec<User>,
    total: u32,
}

#[derive(Debug, Deserialize)]
pub struct User {
    id: String,
    email: String,
    // Add other fields as per the API response
}
```

3. Update `src/api/mod.rs` to include the new module:

```rust
pub mod list_users;
```

### Step 4: Update the Main Execution Logic

1. Open `src/main.rs`
2. Add the new import:

```rust
use crate::api::list_users::ListUsersRequest;
```

3. Add a new match arm in the `main` function:

```rust
match args.entity_type {
    cli::main_cli::EntityType::Get(get_command) => match get_command.command {
        // ... existing matches ...
        cli::main_cli::GetSubCommand::ListUsers(list_users) => {
            let request = ListUsersRequest::new(list_users);
            handle_request(&request, "/users", Operation::Fetch(FetchFilter::None));
        }
    },
    // ... other matches ...
}
```

### Step 5: Implement Optional Parameter Handling

In the `ListUsersRequest::new` method in `src/api/list_users.rs`, handle the optional parameters:

```rust
impl ListUsersRequest {
    pub fn new(args: ListUsers) -> Self {
        ListUsersRequest {
            email: args.email,
            limit: args.limit,
        }
    }
}
```

In the `fetch` method of the `Executable` trait implementation, you'll need to handle these optional parameters when constructing the API request. This might involve adding query parameters to the URL or modifying the request body.

### Step 6: Add Tests

1. Create a new file `tests/list_users_test.rs`
2. Implement tests for the new command:

```rust
use assert_cmd::Command;

#[test]
fn test_list_users() {
    let mut cmd = Command::cargo_bin("konnectctl").unwrap();
    let assert = cmd
        .arg("get")
        .arg("list-users")
        .assert();

    assert.success();
}

#[test]
fn test_list_users_with_email_filter() {
    let mut cmd = Command::cargo_bin("konnectctl").unwrap();
    let assert = cmd
        .arg("get")
        .arg("list-users")
        .arg("--email")
        .arg("test@example.com")
        .assert();

    assert.success();
}
```

### Step 7: Update Documentation

1. Update the README.md file with information about the new command:


### Usage

```sh
# List all users
konnectctl get list-users

# List users with email filter
konnectctl get list-users --email test@example.com

# List users with a limit
konnectctl get list-users --limit 10
```

2. Update any other relevant documentation files.

## Step 8: Build and Test

1. Build the project:

```sh
cargo build
```

2. Run the tests:

```sh
cargo test
```

3. Try out the new command:

```sh
cargo run -- get list-users
cargo run -- get list-users --email test@example.com --limit 5
```

Remember to handle errors appropriately, validate input, and ensure the command follows the overall style and conventions of the existing codebase.

By following these steps, you should have a fully functional new CLI command in KonnectCTL. Make sure to test thoroughly with various inputs and edge cases before considering the implementation complete.

Remember to consult the [Konnect API documentation](https://developer.konghq.com/) for details on available endpoints and request/response structures when implementing new commands.

## Documentation

For more detailed documentation, please visit our [Wiki](https://github.com/rajeevkong/konnect-cli/wiki) (Note: Update this link when documentation is available)

For comprehensive information about the Konnect API, visit the [Konnect API documentation](https://developer.konghq.com/).

## Contributing

We welcome contributions to KonnectCTL! Here are some ways you can contribute:

1. Report bugs or request features by opening an issue
2. Submit pull requests for bug fixes or new features
3. Improve documentation
4. Add new CLI commands to extend functionality

Please read our [Contributing Guidelines](CONTRIBUTING.md) for more details. (Note: Create this file with contribution details)

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/rajeevkong/konnect-cli/tags).

## Troubleshooting

If you encounter issues, please check the following:

1. Ensure your `.env` file is correctly configured
2. Verify that you're using a supported Rust version
3. Check our [Issues](https://github.com/rajeevkong/konnect-cli/issues) page for known problems

If your issue persists, please open a new issue with details about your environment and the problem you're experiencing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Kong for providing the Konnect API
- The Rust community for their excellent tools and libraries
- My AI colleague [claude](https://claude.ai/) who helped patiently by validating ideas and explaining `rust` as I went about writing this code.
