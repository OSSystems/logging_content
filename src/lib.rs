// Copyright (C) 2021 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

pub use log::Level;
mod option;
mod result;

#[macro_export]
macro_rules! trait_LogContent {
    () => {
        $crate::trait_LogContent!(::log);
    };
    ($log_lib:path) => {
        pub trait LogContent: Sized {
            fn check_content(&self, level: $crate::Level) -> Option<String>;

            fn log_error_msg(self, msg: &str) -> Self {
                self.log($crate::Level::Error, msg)
            }

            fn log_warn_msg(self, msg: &str) -> Self {
                self.log($crate::Level::Warn, msg)
            }

            fn log_info_msg(self, msg: &str) -> Self {
                self.log($crate::Level::Info, msg)
            }

            fn log_debug_msg(self, msg: &str) -> Self {
                self.log($crate::Level::Debug, msg)
            }

            fn log_trace_msg(self, msg: &str) -> Self {
                self.log($crate::Level::Trace, msg)
            }

            fn log(self, level: $crate::Level, msg: &str) -> Self {
                if let Some(content) = self.check_content(level) {
                    use $log_lib::{debug, error, info, trace, warn};
                    #[allow(unreachable_patterns)]
                    match level {
                        $crate::Level::Error => error!("{}: {}", msg, content),
                        $crate::Level::Warn => warn!("{}: {}", msg, content),
                        $crate::Level::Info => info!("{}: {}", msg, content),
                        $crate::Level::Debug => debug!("{}: {}", msg, content),
                        $crate::Level::Trace => trace!("{}: {}", msg, content),
                        _ => {}
                    }
                }
                self
            }
        }
    };
}

#[macro_export]
macro_rules! trait_LogDisplay {
    () => {
        pub trait LogDisplay {
            fn as_log_display(&self, level: $crate::Level) -> String;
        }
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use once_cell::sync::OnceCell;
    use std::io::{Seek, SeekFrom};
    use tempfile::NamedTempFile;

    static LOG_FILE: OnceCell<NamedTempFile> = OnceCell::new();

    pub(crate) fn init_log() -> std::path::PathBuf {
        let tmp_file = LOG_FILE.get_or_init(|| {
            let tmp_file = tempfile::NamedTempFile::new().unwrap();
            simplelog::WriteLogger::init(
                log::LevelFilter::Trace,
                simplelog::ConfigBuilder::new()
                    .set_time_level(log::LevelFilter::Off)
                    .build(),
                // Cloned so we can seek it to the start later
                tmp_file.as_file().try_clone().unwrap(),
            )
            .unwrap();
            tmp_file
        });
        let path = tmp_file.path();

        // Cleans up any leftover entries from previous tests
        std::fs::write(path, "").unwrap();
        tmp_file.as_file().seek(SeekFrom::Start(0)).unwrap();

        path.to_owned()
    }

    #[test]
    #[serial_test::serial]
    fn custom_struct_example() {
        let output_path = crate::tests::init_log();

        // Some data structure
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Product {
            id: i32,
            project: String,
            status: String,
            model: String,
            version: String,
            created_by: i32,
            created_at: String,
        }

        trait_LogDisplay!();
        trait_LogContent!();

        impl LogDisplay for Product {
            fn as_log_display(&self, level: Level) -> String {
                match level {
                    // Dump all fields
                    Level::Trace | Level::Debug => format!("{:?}", self),
                    // Show only most relevant data
                    _ => format!(
                        "Product {{ project: {}, status: {}, version: {} }}",
                        self.project, self.status, self.version
                    ),
                }
            }
        }

        impl LogContent for Product {
            fn check_content(&self, level: Level) -> Option<String> {
                Some(self.as_log_display(level))
            }
        }

        fn new_product() -> Product {
            Product {
                id: 32,
                project: "Foo".to_owned(),
                status: "Finished".to_owned(),
                model: "axf32".to_owned(),
                version: "3.1.2".to_owned(),
                created_by: 1,
                created_at: "2021-09-03 17:29".to_owned(),
            }
        }

        // If we want to log it as info
        let _ = new_product().log_info_msg("product info loaded from DB");
        // At some other point in the code if we wan't to log it as debug
        let _ = new_product().log_debug_msg("received from API");

        insta::assert_snapshot!(std::fs::read_to_string(&output_path).unwrap(), @r###"
        [INFO] product info loaded from DB: Product { project: Foo, status: Finished, version: 3.1.2 }
        [DEBUG] (4) logging_content::tests: received from API: Product { id: 32, project: "Foo", status: "Finished", model: "axf32", version: "3.1.2", created_by: 1, created_at: "2021-09-03 17:29" }
        "###);
    }
}
