use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use serde_json::Value;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use tokio::task;
use std::process::Command;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    display_welcome_message();

    print!(" [ 0 ] - Please enter the path to the email file (e.g., emails.txt) : ");
    io::stdout().flush()?;

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    let file_path = file_path.trim();

    match read_lines(file_path) {
        Ok(lines) => {
            let email_list: Vec<String> = lines.filter_map(Result::ok).collect();
            let email_count = email_list.len();
            print!(" [ 0 ] - Found {} emails in the file !", email_count);
            io::stdout().flush()?;

            print!("\n [ 0 ] - Press Enter to start :");
            io::stdout().flush()?;

            let mut start_input = String::new();
            io::stdin().read_line(&mut start_input)?;

            let output_file_path = "Available.txt";
            let client = reqwest::Client::new();

            let request_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let free_email_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

            let mut tasks = vec![];

            for email in email_list {
                let client = client.clone();
                let request_count = std::sync::Arc::clone(&request_count);
                let free_email_count = std::sync::Arc::clone(&free_email_count);
                let email_clone = email.clone();
                let good = "GOOD";
                let bad = "BAD";
                let current_count = request_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                update_cmd_title(&format!("OUTLOOK CHECKER - _0X0 : Requests: {}", current_count));

                let task = task::spawn(async move {
                    match send_request(&client, &email_clone).await {
                        Ok(is_free) => {
                            let free_count = free_email_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                            if is_free {
                                println!("[ {} ] Available email: {} - Req Num : {}", good.green(), email_clone.green(), free_count.to_string().green());
                                if let Err(e) = save_email_to_file(output_file_path, &email_clone) {
                                    eprintln!("Failed to save email: {}", e);
                                }
                            } else {
                                println!("[ {} ] Not free: {} - Req Num : {}", bad.red(), email_clone.red(), free_count.to_string().red());
                                if let Err(e) = save_email_to_file("unAvailable.txt", &email_clone) {
                                    eprintln!("Failed to save email: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error checking email {}: {}", email_clone, e);
                        }
                    }
                });

                tasks.push(task);
            }

            for task in tasks {
                task.await?;
            }

            Ok(())
        }
        Err(_) => {
            eprintln!("Failed to read the file: {}", file_path);
            Err("File reading failed".into())
        }
    }
}

fn display_welcome_message() {
    println!(" ");
    println!(" ");
    println!("{}", "   █▀█ ▀▄▀ █▀█   █▀█ █░█ ▀█▀ █░░ █▀█ █▀█ █▄▀   █▀▀ █░█ █▀▀ █▀▀ █▄▀ █▀▀ █▀█".green());
    println!("{}", "   █▄█ █░█ █▄█   █▄█ █▄█ ░█░ █▄▄ █▄█ █▄█ █░█   █▄▄ █▀█ ██▄ █▄▄ █░█ ██▄ █▀▄".green());
    println!("\n [ 0 ] - Welcome to the Outlook Email Checker! programmed by @_0x0");
}

async fn send_request(client: &reqwest::Client, email: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url = "https://odc.officeapps.live.com/odc/v2.1/idp";

    let params = [
        ("hm", "0"),
        ("emailAddress", email),
        ("_", "1732816953204"),
    ];

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str("mkt=en-US; mkt1=en-US; ...")?);
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Linux; Android 9; SM-G977N Build/PQ3A.190705.06121522; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/124.0.6367.82 Mobile Safari/537.36"));
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Chromium\";v=\"124\", \"Android WebView\";v=\"124\", \"Not-A.Brand\";v=\"99\""));
    headers.insert("X-Oneauth-Version", HeaderValue::from_static("4.0.2"));
    headers.insert("X-Oneauth-Appname", HeaderValue::from_static("OutlookOneAuth"));
    headers.insert("X-Correlationid", HeaderValue::from_static("0b6c443d-214e-4cbe-aac1-e440769aadd5"));
    headers.insert("X-Oneauth-Appid", HeaderValue::from_static("com.microsoft.office.outlook"));
    headers.insert("X-Office-Application", HeaderValue::from_static("OneAuthMsal"));
    headers.insert("X-Office-Platform", HeaderValue::from_static("Android"));
    headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));
    headers.insert("Enlightened-Hrd-Client", HeaderValue::from_static("1"));

    let response = client.get(url).query(&params).headers(headers).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if let Some(account) = json.get("account") {
            if account == "Neither" {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn save_email_to_file(file_path: &str, email: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", email)?;

    Ok(())
}

fn update_cmd_title(title: &str) {
    Command::new("cmd")
        .arg("/C")
        .arg(format!("title {}", title))
        .output()
        .expect("Failed to update console title");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
