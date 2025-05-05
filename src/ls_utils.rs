use std::fs;
use std::io;
use std::path::Path;
//use std::time::SystemTime;
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_group_by_gid};
use chrono::{DateTime, Local};


pub struct Options {
    pub all: bool, // -a
    pub long_format: bool, // -l
    pub author: bool,
    pub reverse: bool,
}

use std::path::PathBuf;

struct FileData {
    name: String,
    size: u64,
    permissions: String,
    user: String,
    group: String,
    hard_links: u64,
    modified: String,
    is_dir: String,
    path: PathBuf,
}


fn build_file_data(path: &Path,) -> std::io::Result<FileData> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
    let datetime: DateTime<Local> = modified_time.into();

    let uid = metadata.uid();
    let gid = metadata.gid();
    let mut is_dir = String::from("");
    if metadata.is_dir() {
        is_dir = "d".to_string();
    } else {
        is_dir = "-".to_string();
    }
    let user = users::get_user_by_uid(uid)
        .and_then(|u| u.name().to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| uid.to_string());

    let group = users::get_group_by_gid(gid)
        .and_then(|g| g.name().to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| gid.to_string());

    Ok(FileData {
        name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        size: metadata.len(),
        permissions: format_permissions(metadata.mode()),
        user,
        group,
        hard_links: metadata.nlink(),
        modified: datetime.format("%b %e %H:%M").to_string(),
        is_dir,
        path: path.to_path_buf(),
    })
}

fn format_permissions(mode: u32) -> String {
    let mut perms = String::new();
    perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    perms
}





pub fn ls_(path: &Path, config: Options,sort_mode:&str) -> io::Result<()> {
    let mut files = Vec::new();
    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(Result::ok)
            .collect();
        
        match sort_mode {
            "name" => entries.sort_by_key(|e| e.file_name().to_string_lossy().to_string()),
            "size" => entries.sort_by_key(|e| e.metadata().map(|m| m.len()).unwrap_or(0)),
            "time" => {
                entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
                entries.reverse()
            },
            &_ => todo!(),
        }

        if config.reverse {
            entries.reverse()
        }
        for entry in entries {
            if let Ok(file_data) = build_file_data(&entry.path()) {
                files.push(file_data);
            } else {
                eprintln!("Could not read metadata for: {:?}", path);
            }
        }
        
        for file in files {
            get_file_metadata(&file,&config);
        }
    } else {
        println!("Path is not a directory");
    }

    Ok(())
}

pub fn get_file_metadata(file_data: &FileData, options: &Options) -> io::Result<()> {
    if !options.all && file_data.name.starts_with('.') {
            return Ok(()); 
        }
    
        if options.long_format {
            if options.author {
                println!(
                    "{}{} {} {} {} {} {} {}",
                    file_data.is_dir,
                    file_data.permissions,
                    file_data.hard_links,
                    file_data.size,
                    file_data.user,
                    file_data.group,
                    file_data.user, 
                    file_data.modified
                );
            } else {
                println!(
                    "{}{} {} {} {} {} {}",
                    file_data.is_dir,
                    file_data.permissions,
                    file_data.hard_links,
                    file_data.size,
                    file_data.user,
                    file_data.group,
                    file_data.modified
                );
            }
        } else {
            print!("{} ", file_data.name);
        }
    
        Ok(())
    }