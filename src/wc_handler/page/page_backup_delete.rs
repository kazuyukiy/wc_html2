use super::page_utility;
use super::Page;
use chrono::prelude::*;
use std::fs;

use tracing::{error, info, info_span}; //  event, instrument, span, Level debug,

pub fn backup_delete(page: &mut super::Page) {
    info!("backup_delete file_path: {}", page.file_path());

    // files_leave: number of files to leave; ie. not delete and keep
    let files_leave = 10;
    let files_leave = 3;

    // days atleas keep
    let days_keep = 30;
    // let days_keep = 900;

    // let dir_gabage = String::from(page.stor_root());
    // let dir_gabage = dir_gabage + "/gabage";
    // info!("gabage: {}", dir_gabage);
    // let dir_gabage = std::path::Path::new(&dir_gabage);

    let Ok(dir_gabage) = dir_gabage(page) else {
        return;
    };

    // let file_gabage = match std::fs::File::new("") {

    delete_files(page, files_leave, days_keep, &dir_gabage);

    // script in below is only to show the current page info.

    let path = page.file_path();

    let file = match std::fs::File::open(&path) {
        Ok(v) => v,
        Err(e) => {
            error!("{} on {:?}", e, path);
            // return None;
            return;
        }
    };

    // duration_modified_days(&file);
    // let modified_days = match duration_modified_days(&file) {
    match duration_modified_days(&file) {
        // Ok(v) => v,
        Ok(modified_days) => info!("{:?}: {}", &path, modified_days),
        Err(e) => {
            error!("Failed to get duration days: {}", e);
            // continue;
        }
    };

    // test01(page);
}

// fn duration_modified_days(file: &std::fs::File) -> Result<u64, String> {
fn duration_modified_days(file: &std::fs::File) -> Result<usize, String> {
    let metadata = file.metadata().or_else(|e| Err(e.to_string()))?;
    let modified = metadata.modified().or_else(|e| Err(e.to_string()))?;
    let now = std::time::SystemTime::now();
    let duration_modified = now
        .duration_since(modified)
        .or_else(|e| Err(e.to_string()))?;
    let one_day_in_secs = 60 * 60 * 24;
    let duration_modified_days = duration_modified.as_secs() / one_day_in_secs;

    usize::try_from(duration_modified_days).or_else(|e| Err(e.to_string()))

    // let duration_modified_days =
    //     usize::try_from(duration_modified_days).or_else(|e| Err(e.to_string()))?;
    // Ok(duration_modified_days)
}

fn delete_files(
    page: &mut super::Page,
    files_leave: usize,
    days_keep: usize,
    dir_gabage: &std::path::Path,
) -> Result<(), ()> {
    let rev = match page.rev() {
        Ok(v) => v,
        Err(_) => {
            error!("Failed to get rev on {}", page.file_path());
            return Err(());
        }
    };

    info!("rev latest: {}", rev);

    let Some(rev_sub) = rev.checked_sub(files_leave) else {
        return Ok(());
    };

    for rev_app in { 0..=rev_sub }.rev() {
        let path_rev = page.path_rev_form(rev_app);
        let path_rev = std::path::Path::new(&path_rev);
        let file = match std::fs::File::open(&path_rev) {
            Ok(v) => v,
            Err(e) => {
                error!("{} on {:?}", e, path_rev);
                break;
            }
        };

        let modified_days = match duration_modified_days(&file) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get duration days: {} on {:?}", e, path_rev);
                // continue;
                break;
            }
        };

        info!("{:?}: {}", path_rev, modified_days);

        if modified_days < days_keep {
            info!("Less than days_keep ({})", days_keep);
            break;
        }

        // file;
        // dir_gabage;

        let Some(filename_rev) = path_rev.file_name() else {
            break;
        };
        let path_gabage = dir_gabage.join(filename_rev);

        info!("move {:?} to {:?}", &path_rev, path_gabage);

        // info!("path_gabage: {:?}", path_gabage);

        if let Err(e) = std::fs::rename(&path_rev, &path_gabage) {
            error!("{} on {:?}", e, &path_rev);
        }
    }

    // { 0..=rev_sub }.rev().map(|rev| rev);

    // temp
    Ok(())
}

fn dir_gabage(page: &mut super::Page) -> Result<std::path::PathBuf, ()> {
    let dir_gabage = String::from(page.stor_root());
    let dir_gabage = dir_gabage + "/gabage";
    info!("gabage: {}", dir_gabage);
    // let dir_gabage = std::path::PathBuf::new(&dir_gabage);
    let dir_gabage = std::path::PathBuf::from(&dir_gabage);

    // page_utility::dir_build required a path for a file as an argument
    // but create only directories.
    // let path_gabage = dir_gabage.clone().join("dummy");
    let path_gabage = dir_gabage.join("dummy");

    let recursive = true;
    if let Err(_) = page_utility::dir_build(&path_gabage, recursive) {
        return Err(());
    };

    Ok(dir_gabage)
}

fn test01(page: &mut super::Page) {
    // ex, ./pages/wc_top.html
    let page_path_str = page.file_path();
    let page_path = std::path::Path::new(&page_path_str);

    info!("page_path: {:?}", page_path);
    // "./pages/wc_top.html"

    // ex, ./pages/
    let Some(page_dir) = page_path.parent() else {
        return;
    };

    info!("page_dir: {:?}", page_dir);
    // "./pages"

    // let page_dir = "./pages/";

    // let dir_entires = match fs::read_dir(page_path) {
    let dir_entires = match fs::read_dir(page_dir) {
        Ok(v) => v,
        Err(e) => {
            error!("{} {:?}", e, page_dir);
            return;
        }
    };

    // DBG breaking loop in some times to make run work not big.
    // let mut dbg_cnt = 0;

    for entry_rs in dir_entires {
        // if 10 < dbg_cnt {
        //     break;
        // }

        // dbg_cnt = dbg_cnt + 1;

        let entry = match &entry_rs {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        // entry.path()
        // PathBuf

        // Go on only "./pages/wc_top.html"
        if let Some(path) = entry.path().as_path().to_str() {
            // if path != "./pages/wc_top.html" {
            if path != page_path_str {
                continue;
            }
        } else {
            continue;
        }

        info!("entry_rs: {:?}", entry_rs);

        // created and modified
        // created is not always the oldest date conserned with the file.
        // When the file is copied, the created date becoms the date copied.
        // In this case modified date might be older than the created date.
        let created = match entry.metadata() {
            Ok(metadata) => {
                // created: std::time::SystemTime
                // let created = metadata.created();
                let created = metadata.modified();
                // info!("created: {:?}", created);
                match created {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            Err(e) => {
                error!("{}", e);
                None
            }
        };

        let created = match created {
            Some(v) => v,
            None => {
                error!("Failed to get time created: {}", page_path_str);
                continue;
            }
        };

        let now = std::time::SystemTime::now();
        // info!("");
        let duration_file_made = match now.duration_since(created) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get duration file created: {}", e);
                continue;
            }
        };

        println!("duration.as_secs: {}", duration_file_made.as_secs());
        println!(
            "duration.subsec_millis: {:?}",
            duration_file_made.subsec_millis()
        );

        // let duration_mills = duration_file_made.subsec_millis();
        // let one_day_in_secs = 1000 * 60 * 60 * 24;
        let one_day_in_secs = 60 * 60 * 24;
        // let duration_days = duration_mills / one_day_in_secs;

        let duration_days = duration_file_made.as_secs() / one_day_in_secs;

        info!("duration_days: {}", duration_days);

        let created_local: DateTime<Local> = chrono::DateTime::from(created);
        // let created_local: DateTime<Utc> = chrono::DateTime::from(created);
        info!("created_local: {}", created_local);
        info!("now: {:?}", now);

        let file_type = match entry.file_type() {
            Ok(file_type) => {
                // info!("file_type: {:?}", file_type);
                file_type
            }
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        if file_type.is_dir() {
            continue;
            // info!("dir: {:?}", entry.path());
        }
        if file_type.is_file() {
            info!("file: {:?}", entry.path());
        }
        //
    }
}
