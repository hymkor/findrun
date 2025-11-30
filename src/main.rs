fn find_in_paths(filename: &str) -> Option<String> {
    let paths = match std::env::var_os("PATH") {
        None => return None,
        Some(value) => value,
    };
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(filename);
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    None
}

fn findrun(mut args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let _ = args.next();
    let mut param: Vec<String> = Vec::new();
    let mut done = false;
    while let Some(mut arg) = args.next() {
        if !done && arg.contains(".") && !arg.contains("/") && !arg.contains("\\") {
            if let Some(value) = find_in_paths(&arg) {
                arg = value;
                done = true
            }
        }
        param.push(arg)
    }
    if param.is_empty() {
        eprintln!("Usage: findrun [-d] INTERPRETER SCRIPT [ARGS...]");
        eprintln!("");
        eprintln!("Searches the first argument containing a dot but no path separators in PATH,");
        eprintln!("replaces it with its full path if found, and executes it with the specified");
        eprintln!("interpreter.");
        return Ok(());
    }
    let interp = param.remove(0);
    if interp == "-d" {
        println!("{}", param.join(" "));
        return Ok(());
    }
    match std::process::Command::new(interp).args(&param).status() {
        Err(err) => return Err(Box::new(err)),
        Ok(_) => return Ok(()),
    }
}

fn main() {
    if let Err(err) = findrun(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
