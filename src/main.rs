use std::env;
use webbrowser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let url = if args.len() > 1 {
        let query = args[1..].join(" ");
        let encoded_query = urlencoding::encode(&query);
        format!("https://t3.chat/new?q={}", encoded_query)
    } else {
        "https://t3.chat".to_string()
    };
    
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Error during T3.chat CLI startup: {}", e);
    }
}
