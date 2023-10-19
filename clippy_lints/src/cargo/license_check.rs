use cargo_metadata::Metadata;
use clippy_utils::diagnostics::span_lint;
use rustc_lint::LateContext;
use rustc_span::source_map::DUMMY_SP;

use super::LICENSE_CHECK;

pub(super) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    for package in &metadata.packages {
        if !is_empty_str(&package.license) {
            println!("package license is not empty!");
            let message = format!("package `{}` has license OR license_file defined", package.name);
            span_lint(cx, LICENSE_CHECK, DUMMY_SP, &message);
        }
    }
}

fn is_empty_str<T: AsRef<std::ffi::OsStr>>(value: &Option<T>) -> bool {
    value.as_ref().map_or(true, |s| s.as_ref().is_empty())
}

// fn exist_warning(cx: &LateContext<'_>, package: &cargo_metadata::Package, field: &str) {
//     let message = format!("package `{}` is missing `{field}` metadata", package.name);
//     span_lint(cx, LICENSE_CHECK, DUMMY_SP, &message);
// }
