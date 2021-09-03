// Copyright (C) 2021 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! impl_Result {
    () => {
        impl<T, E> LogContent for Result<T, E>
        where
            T: LogDisplay,
            E: LogDisplay,
        {
            fn check_content(&self, level: $crate::Level) -> Option<String> {
                match (self, level) {
                    (Err(ref e), $crate::Level::Error) => Some(e.as_log_display(level)),
                    (Err(ref e), $crate::Level::Warn) => Some(e.as_log_display(level)),
                    (Ok(ref t), $crate::Level::Info) => Some(t.as_log_display(level)),
                    (Ok(ref t), $crate::Level::Debug) => Some(t.as_log_display(level)),
                    (Ok(ref t), $crate::Level::Trace) => Some(t.as_log_display(level)),
                    _ => None,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_Result_no_ok {
    () => {
        impl<T, E> LogContent for Result<T, E>
        where
            E: LogDisplay,
        {
            fn check_content(&self, level: $crate::Level) -> Option<String> {
                match (self, level) {
                    (Err(ref e), $crate::Level::Error) => Some(e.as_log_display(level)),
                    (Err(ref e), $crate::Level::Warn) => Some(e.as_log_display(level)),
                    _ => None,
                }
            }
        }
    };
}

#[test]
#[serial_test::serial]
fn conversion_result() {
    let output_path = crate::tests::init_log();

    crate::trait_LogDisplay!();
    crate::trait_LogContent!();
    impl_Result!();

    impl LogDisplay for i32 {
        fn as_log_display(&self, _: log::Level) -> String {
            self.to_string()
        }
    }
    impl LogDisplay for std::num::ParseIntError {
        fn as_log_display(&self, _: log::Level) -> String {
            format!("{} ({:?})", self, self)
        }
    }

    use std::str::FromStr;
    let _ = i32::from_str("42")
        .log_info_msg("Successfully parsed")
        .log_error_msg("Failed to parse str");
    let _ = i32::from_str("forty two")
        .log_info_msg("Successfully parsed")
        .log_error_msg("Failed to parse str");

    insta::assert_snapshot!(std::fs::read_to_string(&output_path).unwrap(), @r###"
        [INFO] Successfully parsed: 42
        [ERROR] Failed to parse str: invalid digit found in string (ParseIntError { kind: InvalidDigit })
        "###);
}
