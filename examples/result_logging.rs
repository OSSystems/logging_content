// Copyright (C) 2021 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

logging_content::trait_LogContent!();
logging_content::trait_LogDisplay!();
logging_content::impl_Result!();

impl LogDisplay for u8 {
    fn as_log_display(&self, _: logging_content::Level) -> String {
        format!("{}", self)
    }
}

fn might_fail() -> Result<u8, u8> {
    match rand::random::<u8>() % 20 {
        x if x <= 10 => Ok(x),
        x => Err(x),
    }
}

fn main() {
    simplelog::TermLogger::init(
        log::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let _ = might_fail()
        .log_info_msg("First value generated")
        .log_warn_msg("First value is out of bounds");
    let _ = might_fail()
        .log_info_msg("Second value generated")
        .log_warn_msg("Second value is out of bounds");
    let _ = might_fail()
        .log_info_msg("Third value generated")
        .log_warn_msg("Third value is out of bounds");
}
