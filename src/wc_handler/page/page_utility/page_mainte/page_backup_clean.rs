pub use super::page_children_url;
use super::Page;
use tracing::{error, info}; //  event, instrument, span, Level debug,, info_span, warn
                            //

pub fn page_backup_clean(page: &mut Page, recursive: bool) {
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

    if recursive {
        page_backup_clean_children(page, recursive);
    }
}

fn page_backup_clean_children(page: &mut Page, recursive: bool) {
    let stor_root = page.stor_root().to_string();
    let child_url_s = page_children_url(page);
    for child_url in child_url_s {
        let mut child_page = super::Page::new(&stor_root, child_url.path());
        page_backup_clean(&mut child_page, recursive);
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

/// ex. wc_top.html.7
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
        if let Ok(()) = delete_html_dot_rev(page, rev_app, days_keep, dir_gabage) {
            moved_rev.push(rev_app);
        };
    }

    info!(
        "gabage moved html.rev {} rev:{:?}",
        page.file_path(),
        moved_rev
    );

    Ok(moved_rev.len())
}

/// Delete page file with rev as a backup.
/// Return
fn delete_html_dot_rev(
    page: &mut Page,
    rev: usize,
    days_keep: usize,
    dir_gabage: &std::path::Path,
) -> Result<(), ()> {
    let path_rev = page.path_rev_form(rev);

    // this is_file() filter avoids error message at duration_modified_days call later.
    if !path_rev.is_file() {
        return Err(());
    }

    let modified_days = match duration_modified_days(&path_rev) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to get duration days: {} on {:?}", e, path_rev);
            // continue;
            return Err(());
        }
    };

    if modified_days < days_keep {
        info!("Less than days_keep ({})", days_keep);
        return Err(());
    }

    // file;
    // dir_gabage;

    let Some(filename_rev) = path_rev.file_name() else {
        return Err(());
    };
    let path_gabage = dir_gabage.join(filename_rev);

    if let Err(e) = rename(&path_rev, &path_gabage) {
        error!("{} on {:?}", e, &path_rev);
        Err(())
    } else {
        // moved_rev.push(rev);
        Ok(())
    }
}

fn dir_gabage(page: &mut Page) -> Result<std::path::PathBuf, ()> {
    let dir_gabage = String::from(page.stor_root());
    let dir_gabage = dir_gabage + "/gabage";
    let dir_gabage = std::path::PathBuf::from(&dir_gabage);

    // page_utility::dir_build required a path for a file as an argument
    // but create only directories. So "dummy " is for a some file name.
    let path_gabage = dir_gabage.join("dummy");

    let recursive = true;
    if let Err(_) = super::super::dir_build(&path_gabage, recursive) {
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

    let mut moved_rev = vec![];
    loop {
        // // parent + file_stem + "_rev" + rev + (.) + extension
        // let path_rev = match path_with_rev_dot(page, rev) {
        //     Ok(v) => v,
        //     Err(_) => break,
        // };

        // let Some(filename_rev) = path_rev.file_name() else {
        //     break;
        // };
        // let path_gabage = dir_gabage.join(filename_rev);

        // if let Err(_e) = rename(&path_rev, &path_gabage) {
        if let Err(_) = delete_rev_dot_html(page, rev, dir_gabage) {
            // error!("{} on {:?}", e, &path_rev);
        } else {
            moved_rev.push(rev);
            rev_max = match rev.checked_add(probes) {
                Some(v) => v,
                None => rev,
            };
        }

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

    Ok(moved_rev.len())
}

fn delete_rev_dot_html(
    page: &mut Page,
    rev: usize,
    dir_gabage: &std::path::Path,
) -> Result<(), ()> {
    // parent + file_stem + "_rev" + rev + (.) + extension
    let path_rev = path_with_rev_dot(page, rev)?;

    let filename_rev = path_rev.file_name().ok_or(())?;
    let path_gabage = dir_gabage.join(filename_rev);
    rename(&path_rev, &path_gabage).or(Err(()))
}

/// parent + file_stem + "_rev" + rev + (.) + extension
/// ex. ./pages/wc_top.html to ./pages/wc_top_rev2.html
fn path_with_rev_dot(page: &mut Page, rev: usize) -> Result<std::path::PathBuf, ()> {
    // page.path() ex. ./pages/wc_top.html

    // wc_top
    let file_stem = match page.path().file_stem() {
        Some(v) => v,
        None => return Err(()),
    };

    // html
    let extension = match page.path().extension() {
        Some(v) => v,
        None => return Err(()),
    };

    // wc_top_rev2.html
    let mut file_name = file_stem.to_os_string();
    file_name.push("_rev");
    file_name.push(rev.to_string().as_str());
    file_name.push(".");
    file_name.push(extension);

    // ./pages/wc_top_rev2.html
    let path_rev = page.path().with_file_name(file_name);

    Ok(path_rev)
}
