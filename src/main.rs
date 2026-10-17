use std::fs::OpenOptions;
use std::io::{self, Write, BufReader, BufRead};
use std::path::Path;

const DATA_FILE: &str = "onepay_sales_db.txt";

#[derive(Clone, Debug)]
pub struct SaleRecord {
    pub order_id: String,
    pub xlm_amount: f64,
    pub usd_proceeds: f64,
    pub timestamp: String,
}

/// Appends a new sale entry directly to your system storage file
fn append_to_file(record: &SaleRecord) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DATA_FILE)?;
    
    writeln!(
        file,
        "{},{},{},{}",
        record.order_id, record.xlm_amount, record.usd_proceeds, record.timestamp
    )?;
    Ok(())
}

/// Reads the local data storage file and prints the history log
fn view_ledger() -> io::Result<()> {
    if !Path::new(DATA_FILE).exists() {
        println!("\n[!] No sales data found yet. Start by logging a new sale!");
        return Ok(());
    }

    let file = OpenOptions::new().read(true).open(DATA_FILE)?;
    let reader = BufReader::new(file);
    
    println!("\n================== ONEPAY BLOCKCHAIN SALES LEDGER ==================");
    println!("{:<36} | {:<12} | {:<12} | {:<10}", "Order ID", "XLM Spent", "USD Proceeds", "Time (Local)");
    println!("--------------------------------------------------------------------");

    let mut total_xlm = 0.0;
    let mut total_usd = 0.0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 4 {
            let order_id = parts[0];
            let xlm: f64 = parts[1].parse().unwrap_or(0.0);
            let usd: f64 = parts[2].parse().unwrap_or(0.0);
            let time = parts[3];

            total_xlm += xlm;
            total_usd += usd;

            println!("{:<36} | {:<12.4} | ${:<11.2} | {:<10}", order_id, xlm, usd, time);
        }
    }
    println!("--------------------------------------------------------------------");
    println!("{:<36} | {:<12.4} | ${:<11.2} | Running Totals", "TOTALS", total_xlm, total_usd);
    println!("====================================================================\n");
    Ok(())
}

fn main() {
    loop {
        println!("=== ONEPAY LEDGER TERMINAL ===");
        println!("1. Log a new system sale record");
        println!("2. View local running transaction database");
        println!("3. Exit program");
        print!("Select an option (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let mut order_id = String::new();
                let mut xlm_str = String::new();
                let mut usd_str = String::new();
                let mut time_str = String::new();

                print!("Enter Order ID (e.g. order.3Kn...): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut order_id).unwrap();

                print!("Enter XLM Amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut xlm_str).unwrap();
                let xlm: f64 = xlm_str.trim().parse().unwrap_or(0.0);

                print!("Enter USD Proceeds ($): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut usd_str).unwrap();
                let usd: f64 = usd_str.trim().parse().unwrap_or(0.0);

                print!("Enter Timestamp (e.g. 16:25:00): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut time_str).unwrap();

                let new_record = SaleRecord {
                    order_id: order_id.trim().to_string(),
                    xlm_amount: xlm,
                    usd_proceeds: usd,
                    timestamp: time_str.trim().to_string(),
                };

                if let Err(e) = append_to_file(&new_record) {
                    println!("[!] Error saving data: {}", e);
                } else {
                    println!("\n[+] Transaction logged successfully to local database!\n");
                }
            }
            "2" => {
                if let Err(e) = view_ledger() {
                    println!("[!] Error reading file: {}", e);
                }
            }
            "3" => {
                println!("Exiting tracking console. Keep building!");
                break;
            }
            _ => println!("[!] Invalid choice, try again.\n"),
        }
    }
}
