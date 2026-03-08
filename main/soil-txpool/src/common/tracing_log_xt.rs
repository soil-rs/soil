// This file is part of Soil.

// Copyright (C) Soil contributors.
// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Utility for logging transaction collections with tracing crate.

/// Logs every transaction from given `tx_collection` with given level.
macro_rules! log_xt {
    (data: hash, target: $target:expr, $level:expr, $tx_collection:expr, $text_with_format:expr) => {
        for tx_hash in $tx_collection {
            tracing::event!(
                target: $target,
                $level,
                ?tx_hash,
                $text_with_format,
            );
        }
    };
    (data: hash, target: $target:expr, $level:expr, $tx_collection:expr, $text_with_format:expr, $($arg:expr),*) => {
        for tx_hash in $tx_collection {
            tracing::event!(
                target: $target,
                $level,
                ?tx_hash,
                $text_with_format,
                $($arg),*
            );
        }
    };
    (data: tuple, target: $target:expr, $level:expr, $tx_collection:expr, $text_with_format:expr) => {
        for (tx_hash, arg) in $tx_collection {
            tracing::event!(
                target: $target,
                $level,
                ?tx_hash,
                $text_with_format,
                arg
            );
        }
    };
}
macro_rules! log_xt_debug {
    (data: $datatype:ident, target: $target:expr, $($arg:tt)+) => {
        $crate::common::tracing_log_xt::log_xt!(data: $datatype, target: $target, tracing::Level::DEBUG, $($arg)+);
    };
    (target: $target:expr, $tx_collection:expr, $text_with_format:expr) => {
        $crate::common::tracing_log_xt::log_xt!(data: hash, target: $target, tracing::Level::DEBUG, $tx_collection, $text_with_format);
    };
    (target: $target:expr, $tx_collection:expr, $text_with_format:expr, $($arg:expr)*) => {
        $crate::common::tracing_log_xt::log_xt!(data: hash, target: $target, tracing::Level::DEBUG, $tx_collection, $text_with_format, $($arg)*);
    };
}

macro_rules! log_xt_trace {
    (data: $datatype:ident, target: $target:expr, $($arg:tt)+) => {
        $crate::common::tracing_log_xt::log_xt!(data: $datatype, target: $target, tracing::Level::TRACE, $($arg)+);
    };
    (target: $target:expr, $tx_collection:expr, $text_with_format:expr) => {
        $crate::common::tracing_log_xt::log_xt!(data: hash, target: $target, tracing::Level::TRACE, $tx_collection, $text_with_format);
    };
    (target: $target:expr, $tx_collection:expr, $text_with_format:expr, $($arg:expr)*) => {
        $crate::common::tracing_log_xt::log_xt!(data: hash, target: $target, tracing::Level::TRACE, $tx_collection, $text_with_format, $($arg)*);
    };
}

pub(crate) use log_xt;
pub(crate) use log_xt_debug;
pub(crate) use log_xt_trace;
