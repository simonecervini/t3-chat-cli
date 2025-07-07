use webbrowser;

fn main() {
    if let Err(e) = webbrowser::open("https://t3.chat") {
        eprintln!("Error during T3.chat CLI startup: {}", e);
    }
}
