
# Outlook Email Checker

This tool checks if email addresses are available for use with **Outlook** or **Hotmail**. It performs a quick check to determine whether an email address is available, printing the result without the need for proxies, restrictions, or delays.
## Feel free to contact me in instagram @_0x0 or telegaram @iwannakillyoudumb

## Features

- **Fast and Efficient**: Designed for speed, checking emails in a streamlined and lightweight manner.
- **No Proxy Required**: Operates without the need for a proxy.
- **No Blocking or Bans**: Avoids blocking or banning issues.
- **Email Availability Check**: Verifies if the email address is available for registration.

## Requirements

- **Rust**: The tool is written in Rust, so you will need the Rust programming environment set up.
- **Tokio**: The asynchronous runtime for Rust, to handle concurrent email checks.
- **Reqwest**: A Rust HTTP client used to send requests and check the availability of email addresses.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/0xx9/outlook.git
```

2. Change to the project directory:

```bash
cd outlook
```

3. Install the necessary dependencies:

```bash
cargo build --release
```

## Usage

1. Prepare a file (`emails.txt`) that contains the list of email addresses you want to check (one per line).
2. Run the program:

```bash
cargo run
```

3. The tool will prompt you for the path to your email file. After that, it will check each email for availability.

4. The results will be saved to two files:
   - `Available.txt` for available emails.
   - `unAvailable.txt` for unavailable emails.

## Example

```txt
[ 0 ] - Please enter the path to the email file (e.g., emails.txt) : emails.txt
[ 0 ] - Found 100 emails in the file!
[ 0 ] - Press Enter to start :
```

The program will start checking emails and print the results in real-time. Available emails will be printed with the message `Available email:`, while unavailable ones will show `Not free`.

## How It Works

- The program uses an HTTP GET request to check if the email is available by sending a request to the Outlook service.
- The result is parsed, and if the email is free, it is stored in `Available.txt`. Otherwise, it is saved in `unAvailable.txt`.

## Exit

After the program finishes checking all the emails, it will prompt you to press Enter to exit the application.

---
