#[allow(dead_code)]
pub fn logo() -> String {
    if std::env::var("TERM").as_deref() == Ok("linux") || !atty::is(atty::Stream::Stdout) {
        ascii_logo().to_string()
    } else {
        include_str!("../../watn-logo.txt").to_string()
    }
}

#[allow(dead_code)]
pub fn ascii_logo() -> &'static str {
    r#"__      __ __ _ | |_  _ __   ___
\ \ /\ / // _` || __|| '_ \ |__ \
 \ V  V /| (_| || |_ | | | |  / /
  \_/\_/  \__,_| \__||_| |_| |_|
                              (_)"#
}
