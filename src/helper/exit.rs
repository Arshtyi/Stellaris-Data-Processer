pub fn safely_exit(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}
