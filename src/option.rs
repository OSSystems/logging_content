// Copyright (C) 2021 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! impl_Option {
    () => {
        $crate::impl_Option!(::log::Level);
    };
    ($log_level:ty) => {
        impl<T: LogDisplay> LogContent for Option<T> {
            fn check_content(&self, level: $log_level) -> Option<String> {
                match (self, level) {
                    (None, <$log_level>::Error) => Some("None".to_owned()),
                    (None, <$log_level>::Warn) => Some("None".to_owned()),
                    (Some(ref t), <$log_level>::Info) => Some(t.as_log_display(level)),
                    (Some(ref t), <$log_level>::Debug) => Some(t.as_log_display(level)),
                    (Some(ref t), <$log_level>::Trace) => Some(t.as_log_display(level)),
                    _ => None,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_Option_no_some {
    () => {
        $crate::impl_Option_no_some!(::log::Level);
    };
    ($log_level:ty) => {
        impl<T> LogContent for Option<T> {
            fn check_content(&self, level: $log_level) -> Option<String> {
                match (self, level) {
                    (None, <$log_level>::Error) => Some("None".to_owned()),
                    (None, <$log_level>::Warn) => Some("None".to_owned()),
                    _ => None,
                }
            }
        }
    };
}

#[test]
#[serial_test::serial]
fn load_env_vars() {
    let output_path = crate::tests::init_log();

    crate::trait_LogDisplay!();
    crate::trait_LogContent!();
    impl_Option!();

    impl LogDisplay for std::ffi::OsString {
        fn as_log_display(&self, _: log::Level) -> String {
            self.to_string_lossy().to_string()
        }
    }

    let _ = std::env::var_os("CARGO_PKG_NAME")
        .log_info_msg("loaded package name")
        .log_warn_msg("package name not present");
    let _ = std::env::var_os("CONFIG_DIR")
        .log_info_msg("config dir loaded from env")
        .log_warn_msg("CONFIG_DIR not present");

    insta::assert_snapshot!(std::fs::read_to_string(&output_path).unwrap(), @r###"
    [INFO] loaded package name: logging_content
    [WARN] CONFIG_DIR not present: None
    "###);
}
