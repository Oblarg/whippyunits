fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Args: {:?}", args);
    
    if args.contains(&"--version".to_string()) {
        println!("whippyunits-lsp-proxy 0.1.0");
        return;
    }
    
    if args.contains(&"--help".to_string()) {
        println!("WhippyUnits LSP Proxy");
        return;
    }
    
    println!("No special args found");
}

