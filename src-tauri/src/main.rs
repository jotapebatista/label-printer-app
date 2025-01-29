use sys_info::os_type;
use std::process::Command;
use std::collections::HashMap;
use tokio;

#[tauri::command]
async fn fetch_printers() -> Result<Vec<HashMap<String, String>>, String> {
    let os = os_type().unwrap_or_default();
    let mut printers_info = Vec::new();

    if os.to_lowercase().contains("darwin") || os.to_lowercase().contains("linux") {
        // Use lpstat or lpinfo for Unix-like systems (macOS, Linux with CUPS)
        let output = Command::new("lpstat")
            .arg("-e") // List available printers
            .output();

        let output = match output {
            Ok(output) => output,
            Err(_) => return Err("Failed to fetch printers on macOS/Linux.".to_string()),
        };

        let printers = String::from_utf8(output.stdout).unwrap_or_default();

        let tasks: Vec<_> = printers.lines().map(|printer| {
            let printer_name = printer.to_string();
            tokio::spawn(async move {
                let mut printer_details = HashMap::new();
                printer_details.insert("name".to_string(), printer_name);

                // Fetch the device information asynchronously
                let ip_output = Command::new("lpinfo")
                    .arg("-v") // List devices
                    .output();

                if let Ok(ip_output) = ip_output {
                    let ip = String::from_utf8(ip_output.stdout).unwrap_or_default();
                    let cleaned_ip = ip.lines()
                        .filter(|line| line.contains("socket://") || line.contains("ipp://"))
                        .find_map(|line| {
                            // Extract IP address after "socket://" or "ipp://"
                            line.split("://")
                                .nth(1)
                                .map(|addr| addr.split_whitespace().next().unwrap_or_default().to_string())
                        })
                        .unwrap_or_else(|| "Unknown IP".to_string());
                    printer_details.insert("ip".to_string(), cleaned_ip);
                }

                // Placeholder for MAC Address (replace with real query if needed)
                let mac_address = "00:08:e1:20:81:46"; // Replace with a real query
                printer_details.insert("mac".to_string(), mac_address.to_string());

                printer_details
            })
        }).collect();

        // Wait for all tasks to finish
        let results = futures::future::join_all(tasks).await;

        for result in results {
            printers_info.push(result.unwrap());
        }
    } else {
        return Err("Unsupported OS for printer detection.".to_string());
    }

    Ok(printers_info)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_printers])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
