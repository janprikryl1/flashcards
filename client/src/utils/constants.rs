pub fn api_base() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let protocol = location.protocol().unwrap();
    let hostname = location.hostname().unwrap();
    format!("{}//{}:3000", protocol, hostname)
}