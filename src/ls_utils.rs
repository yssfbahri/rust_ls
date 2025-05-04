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


pub fn ls_(path:&Path,config:Options) -> io::Result<()> {

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Err(e) = get_file_metadata(path.as_path(),&config){
                eprintln!("Error: {}", e);
            }
        }
    } else {
        println!("Path is not a directory");
    }

    Ok(())
}


pub fn get_file_metadata(file_path: &Path,options: &Options) -> io::Result<()> {

    let metadata = fs::metadata(file_path)?;
        
    let name = file_path.file_name().unwrap().to_string_lossy();
    let size = metadata.len();

    let mut is_dir = String::from("");
    if metadata.is_dir() {
        is_dir = "d".to_string();
    } else {
        is_dir = "-".to_string();
    }

    let modified = metadata.modified()?;
    let mode = metadata.mode(); 
    let hard_links = metadata.nlink();

    let uid = metadata.uid();
    let gid = metadata.gid();

    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or(uid.to_string());

    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or(gid.to_string());

    let permissions = format_permissions(mode);

    let datetime: DateTime<Local> = modified.into();
    let formatted_modified = datetime.format("%b %e %H:%M").to_string();
    let file_name = file_path
    .file_name()
    .unwrap_or_else(|| std::ffi::OsStr::new("Unknown"))
    .to_string_lossy()
    .to_string();

    if !options.all && file_name.starts_with('.') {
        return Ok(()); 
    }
    if options.long_format {
        if options.author {
            println!("{is_dir}{permissions} {hard_links} {size} {user} {group} {user} {formatted_modified} {name}");
        } else {
            println!("{is_dir}{permissions} {hard_links} {size} {user} {group} {formatted_modified} {name}");
        }
    } else {
        let file_name = file_path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("Unknown"))
            .to_string_lossy()
            .to_string();
    
        print!("{file_name} ");
    }
    
    Ok(())
}
