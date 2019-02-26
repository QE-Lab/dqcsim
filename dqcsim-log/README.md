# DQCsim-log

This crate defines the combination of a `LogThread` and `LogProxy` both implementing `log::Log` to start a dedicated logger thread and logger proxies forwarding their log records to the dedicated logger thread via a `crossbeam_channel`.
