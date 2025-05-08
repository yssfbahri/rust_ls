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
    pub author: bool,// --author
    pub reverse: bool,// -r
    pub human: bool, // -h
}

struct FileData {
    name: String,
    size: u64,
    permissions: String,
    user: String,
    group: String,
    hard_links: u64,
    modified: String,
    is_dir: bool,
}


fn build_file_data(path: &Path,) -> std::io::Result<FileData> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());
    let datetime: DateTime<Local> = modified_time.into();

    let uid = metadata.uid();
    let gid = metadata.gid();
    let user = get_user_by_uid(uid)
        .and_then(|u| u.name().to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| uid.to_string());

    let group = get_group_by_gid(gid)
        .and_then(|g| g.name().to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| gid.to_string());
    
    let permissions = format_permissions(metadata.mode(), metadata.is_dir());


    Ok(FileData {
        name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        size: metadata.len(),
        permissions,
        user,
        group,
        hard_links: metadata.nlink(),
        modified: datetime.format("%b %e %H:%M").to_string(),
        is_dir: metadata.is_dir(),
    })
}

fn format_permissions(mode: u32, is_dir: bool) -> String {
    let mut perms = String::new();
    perms.push(if is_dir { 'd' } else { '-' });
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


fn format_size(size: u64) -> String {
    if size >= 1_000_000_000 {
        format!("{:.1}G", size as f64 / 1_000_000_000.0)
    } else if size >= 1_000_000 {
        format!("{:.1}M", size as f64 / 1_000_000.0)
    } else if size >= 1_000 {
        format!("{:.1}K", size as f64 / 1_000.0)
    } else {
        format!("{}B", size)
    }
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
            if !config.all && file.name.starts_with('.') {
                continue;
            }
            if config.long_format {
                long_format_print(&file,config.author,config.human);
            }
            else {
                print!("{} ", file.name);
            }
            
        }
    } else {
        println!("Path is not a directory");
    }

    Ok(())
}


fn long_format_print(file_data: &FileData, author: bool,human:bool){
            let size = if file_data.is_dir {"-".to_string()} else{file_data.size.to_string()};
            if human {
                let human_size = if !file_data.is_dir {format_size(file_data.size)} else {size} ;
                if author {
                    println!(
                        "{:<3} {:<8} {:<8} {:>8} {} {} {} {}",
                        file_data.permissions,
                        file_data.hard_links,
                        human_size,
                        file_data.user,
                        file_data.group,
                        file_data.user, 
                        file_data.modified,
                        file_data.name
                    );
                } else {
                    println!(
                        "{:<3} {:<8} {:<8} {:>8} {} {} {}",
    
                        file_data.permissions,
                        file_data.hard_links,
                        human_size,
                        file_data.user,
                        file_data.group,
                        file_data.modified,
                        file_data.name
                    );
                }
            } else {
                if author {
                    println!(
                        "{:<3} {:<8} {:<8} {:>8} {} {} {} {}",
                        file_data.permissions,
                        file_data.hard_links,
                        size,
                        file_data.user,
                        file_data.group,
                        file_data.user, 
                        file_data.modified,
                        file_data.name
                    );
                } else {
                    println!(
                        "{:<3} {:<8} {:<8} {:>8} {} {} {}",
    
                        file_data.permissions,
                        file_data.hard_links,
                        size,
                        file_data.user,
                        file_data.group,
                        file_data.modified,
                        file_data.name
                    );
                }
            }
            
        }