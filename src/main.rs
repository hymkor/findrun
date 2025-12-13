fn find_in_env_paths(env_name: &str, filename: &str) -> Option<String> {
    let paths = match std::env::var_os(env_name) {
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

fn help() {
    eprintln!("Usage: findrun [-v ENVNAME] [-d] INTERPRETER SCRIPT [ARGS...]");
    eprintln!("");
    eprintln!("Searches the first argument containing a dot but no path separators");
    eprintln!("in the specified environment variable (default: PATH), replaces it");
    eprintln!("with its full path if found, and executes it.");
}

fn run_with_env(
    search_env: String,
    mut param: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut done = false;

    for arg in param.iter_mut() {
        if !done && arg.contains('.') && !arg.contains('/') && !arg.contains('\\') {
            if let Some(value) = find_in_env_paths(&search_env, arg) {
                *arg = value;
                done = true;
            }
        }
    }

    let interp = param.remove(0);
    if interp == "-d" {
        println!("{}", param.join(" "));
        return Ok(());
    }

    match std::process::Command::new(interp).args(&param).status() {
        Err(err) => Err(Box::new(err)),
        Ok(_) => Ok(()),
    }
}

fn findrun(mut args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let _ = args.next();
    let mut search_env = "PATH".to_string();

    // オプション処理
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-v" => {
                if let Some(envname) = args.next() {
                    search_env = envname;
                } else {
                    eprintln!("-v requires an environment variable name");
                    return Ok(());
                }
            }
            "-h" => {
                help();
                return Ok(());
            }
            _ => {
                // 最初の非オプション引数を処理対象に戻す
                let mut param = vec![arg];
                param.extend(args.map(|a| a));
                return run_with_env(search_env, param);
            }
        }
    }

    help();
    Ok(())
}

fn main() {
    if let Err(err) = findrun(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
