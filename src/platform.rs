pub fn canonicalize_path(path: &String) -> String {
    match dunce::canonicalize(path) {
        Ok(p) => p.display().to_string(),
        Err(_) => path.to_string(),
    }
}
