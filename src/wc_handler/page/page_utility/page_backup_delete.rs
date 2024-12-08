use super::Page;
// use chrono::prelude::*;
// use std::fs::File;
use tracing::{error, info}; //  event, instrument, span, Level debug,, info_span, warn

pub fn backup_delete(page: &mut Page) {
    // files_leave: number of files to leave; ie. not delete and keep
    let files_leave = 10;

    // days atleas keep
    let days_keep = 30;
    // let days_keep = 900;

    let Ok(dir_gabage) = dir_gabage(page) else {
        return;
    };

    let moved = match delete_html_dot_rev_s(page, files_leave, days_keep, &dir_gabage) {
        Ok(v) => v,
        Err(_) => return,
    };

    // if delete_html_dot_rev_s moved more than one page,
    // wc_top.html.7 style backup more than file_leave,
    // so all wc_top_rev2.html style are to be moved.
    if 0 < moved {
        let _ = delete_rev_dot_html_s(page, &dir_gabage);
    }
}

fn duration_modified_days(path: &std::path::Path) -> Result<usize, String> {
    let metadata = path.metadata().or_else(|e| Err(e.to_string()))?;
    let modified = metadata.modified().or_else(|e| Err(e.to_string()))?;
    let now = std::time::SystemTime::now();
    let duration_modified = now
        .duration_since(modified)
        .or_else(|e| Err(e.to_string()))?;
    let one_day_in_secs = 60 * 60 * 24;
    let duration_modified_days = duration_modified.as_secs() / one_day_in_secs;

    usize::try_from(duration_modified_days).or_else(|e| Err(e.to_string()))
}

/// This funciton rename may not need to a separate function.
/// But to make rename procedure in one place, it easy to skip renaming for debug.
fn rename(path_rev: &std::path::Path, path_gabage: &std::path::Path) -> std::io::Result<()> {
    // DBG
    //let dbg = true;
    // warn!("DBG skip renaming for delete backup");
    let dbg = false;

    // let err = std::io::Error::new(std::io::ErrorKind::NotFound, "");
    // std::io::ErrorKind::NotFound;

    if dbg {
        // warn!("DBG skip renaming for delete backup");
        // return Ok(());

        // if path_rev exists, behave as rename was successed
        // path_rev.try_exists().and(Ok(()))
        if path_rev.is_file() {
            Ok(())
            // return Ok(());
        } else {
            let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Not file");
            Err(err)
            // return Err(err);
        }
    } else {
        std::fs::rename(&path_rev, &path_gabage)
    }
}

/// wc_top.html.7
fn delete_html_dot_rev_s(
    page: &mut Page,
    files_leave: usize,
    days_keep: usize,
    dir_gabage: &std::path::Path,
    // ) -> Result<(), ()> {
) -> Result<usize, ()> {
    let rev = match page.rev() {
        Ok(v) => v,
        Err(_) => {
            error!("Failed to get rev on {}", page.file_path());
            return Err(());
        }
    };

    let Some(rev_sub) = rev.checked_sub(files_leave) else {
        // return Ok(());
        return Ok(0);
    };

    let mut moved_rev = vec![];
    for rev_app in { 0..=rev_sub }.rev() {
        let path_rev = page.path_rev_form(rev_app);

        // this is_file() filter avoids error message at duration_modified_days call later.
        if !path_rev.is_file() {
            break;
        }

        let modified_days = match duration_modified_days(&path_rev) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get duration days: {} on {:?}", e, path_rev);
                // continue;
                break;
            }
        };

        // info!("{:?}: {}", path_rev, modified_days);

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

        // info!("move {:?} to {:?}", &path_rev, path_gabage);

        // info!("path_gabage: {:?}", path_gabage);

        // if let Err(e) = std::fs::rename(&path_rev, &path_gabage) {
        if let Err(e) = rename(&path_rev, &path_gabage) {
            error!("{} on {:?}", e, &path_rev);
        } else {
            moved_rev.push(rev_app);
        }
    }

    info!(
        "gabage moved html.rev {} rev:{:?}",
        page.file_path(),
        moved_rev
    );

    Ok(moved_rev.len())
}

fn dir_gabage(page: &mut Page) -> Result<std::path::PathBuf, ()> {
    let dir_gabage = String::from(page.stor_root());
    let dir_gabage = dir_gabage + "/gabage";
    // info!("gabage: {}", dir_gabage);
    let dir_gabage = std::path::PathBuf::from(&dir_gabage);

    // page_utility::dir_build required a path for a file as an argument
    // but create only directories.
    let path_gabage = dir_gabage.join("dummy");

    let recursive = true;
    if let Err(_) = super::dir_build(&path_gabage, recursive) {
        return Err(());
    };

    Ok(dir_gabage)
}

// computing_iroiro_rev2.html
fn delete_rev_dot_html_s(page: &mut Page, dir_gabage: &std::path::Path) -> Result<usize, ()> {
    //
    let mut rev: usize = 0;
    // if rename(move) error happens more than probes times,
    // rev will reach rev_max and break the loop.
    let probes = 10;
    let Some(mut rev_max) = rev.checked_add(probes) else {
        return Err(());
    };

    // let path_rev = path_with_rev_dot(page, 2);
    // info!("path_rev: {:?}", path_rev);

    let mut moved_rev = vec![];

    loop {
        // parent + file_stem + "_rev" + rev + (.) + extension
        let path_rev = match path_with_rev_dot(page, rev) {
            Ok(v) => v,
            Err(_) => break,
        };
        // info!("path_rev: {:?}", path_rev);

        let Some(filename_rev) = path_rev.file_name() else {
            break;
        };
        let path_gabage = dir_gabage.join(filename_rev);

        // info!("rev_dot_html move {:?} to {:?}", path_rev, path_gabage);

        // // DBG comment out to not move
        // if let Err(e) = std::fs::rename(&path_rev, &path_gabage) {
        if let Err(_e) = rename(&path_rev, &path_gabage) {
            // error!("{} on {:?}", e, &path_rev);
        } else {
            // moved_rev.push(rev_app);
            moved_rev.push(rev);
            rev_max = match rev.checked_add(probes) {
                Some(v) => v,
                None => rev,
            };
        }

        // DBG push it
        // moved_rev.push(rev);

        rev = match rev.checked_add(1) {
            Some(v) => v,
            None => break,
        };

        if rev_max < rev {
            info!("rev over rev_max");
            break;
        }
    }

    // if 0 < moved_rev.len() {
    info!(
        "gabage moved rev.html {} rev.html:{:?}",
        page.file_path(),
        moved_rev
    );
    // }

    // temp
    // Err(())
    Ok(moved_rev.len())
}

/// parent + file_stem + "_rev" + rev + (.) + extension
fn path_with_rev_dot(page: &mut Page, rev: usize) -> Result<std::path::PathBuf, ()> {
    // page.path()
    // ./pages/wc_top.html

    // wc_top
    let file_stem = match page.path().file_stem() {
        Some(v) => v,
        None => return Err(()),
    };
    // info!("file_stem: {:?}", file_stem);

    // html
    let extension = match page.path().extension() {
        Some(v) => v,
        None => return Err(()),
    };
    // info!("extension: {:?}", extension);

    // wc_top_rev2.html
    let mut file_name = file_stem.to_os_string();
    file_name.push("_rev");
    // file_name.push(2.to_string().as_str());
    file_name.push(rev.to_string().as_str());
    file_name.push(".");
    file_name.push(extension);
    // info!("file_name: {:?}", file_name);

    let path_rev = page.path().with_file_name(file_name);
    // info!("path_rev: {:?}", path_rev);

    Ok(path_rev)
}
