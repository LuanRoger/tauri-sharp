pub fn get_url(port: Option<&u16>) -> String {
    let port = match port {
        Some(port) => port,
        None => &5030,
    };
    format!("http://[::1]:{}", port)
}

#[macro_export]
macro_rules! connect_using {
    ($client_type:ident, $port:expr) => {{
        async {
            let url = crate::connection::get_url($port);
            let client = $client_type::<Channel>::connect(url).await;
            client
        }
    }};
}