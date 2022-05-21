/**
 * Author: hustcer
 * Created: 2022/05/21 10:21:00
 * Description: Show the build info of this App
 */
use shadow_rs::shadow;
use tabled::{object::Segment, style::Style, Alignment, Extract, Modify, TableIteratorExt};

shadow!(build);

pub fn show_info() {
    let is_debug = format!("{}", shadow_rs::is_debug());
    let data = vec![
        ["debug", is_debug.as_str()],
        ["pkg_version", build::PKG_VERSION],
        ["release_tag", build::TAG],
        ["short_commit", build::SHORT_COMMIT],
        ["build_os", build::BUILD_OS],
        ["build_target", build::BUILD_TARGET],
        ["build_time", build::BUILD_TIME],
        ["commit_date", build::COMMIT_DATE],
        ["rust_channel", build::RUST_CHANNEL],
        ["rust_version", build::RUST_VERSION],
    ];

    let table = data
        .table()
        .with(Style::github_markdown()) // psql, blank, github_markdown
        .with(Extract::segment(1.., ..))
        .with(Modify::new(Segment::all()).with(Alignment::left()));

    println!("{}", table);
}
