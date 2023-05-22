#![allow(dead_code)]

use std::{
    fs::{*, copy}, io::{*, self}, path::*, 
};

pub fn prompt() -> io::Result<usize> {
    println!("List of commands:");
    println!("\tfile-delete");
    println!("\tdelete-many");
    println!("\tremove-suffix");
    println!("\tlook-for");
    println!();
 
    println!("Input your command");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match input.trim() {
        "file-delete" => {
            println!("Path of the file?");
            stdout().flush()?;
            let mut answer = String::new();
            stdin().read_line(&mut answer)?;
            delete_file(answer.as_str())?;
        },
        "delete-many" => {
            delet_many()?;
        },
        "remove-suffix" => {
            println!("Enter a director");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            println!("Enter a file extesion");
            let mut input2 = String::new();
            stdin().read_line(&mut input2)?;
            let trimmed_input_one = input.trim();
            let trimmed_input_two = input2.trim();
            extension(&trimmed_input_one, &trimmed_input_two)?;
        },
        "look-for" => {
            println!("Enter a file name");
            stdout().flush()?;
            let mut file_name = String::new();
            stdin().read_line(&mut file_name)?;
            let trimmed_name = file_name.trim();

            println!("Enter a drive or folder");
            stdout().flush()?;
            let mut drive = String::new();
            stdin().read_line(&mut drive)?;
            let trimmed_drive = drive.trim();
            

            look_for(trimmed_drive, trimmed_name)?;
        }
        _ => {},
        }
    Ok(1)
}

pub fn delete_file(file_path: &str) -> io::Result<usize> {
    let path_trim = &file_path.trim();
    let path = Path::new(&path_trim);
    let mut files_deleted = 0;

    // check if file exists 
    if !path.exists() {
        println!("File does not exist");
    }

    if !path.is_dir() {
        remove_file(&path)?;
        println!("Deleted {:?}", &path.display());
    } else {
        println!("This is a folder");
        println!("Are you sure you want to delete it? (y/n)");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let confirm = input.trim();

        if confirm == "y" {
            for entry in read_dir(&path)? {
                let entry_path = entry?.path().clone();
                remove_file(&entry_path)?;
                println!("Deleted {:?}", &entry_path);
                files_deleted += 1;
            }
            remove_dir(&path)?;
            files_deleted += 1;
            println!("Deleted {:?}", &file_path);
            println!("You've deleted {} files", files_deleted);
        }
    }

    Ok(files_deleted)
}

pub fn delet_many() -> io::Result<usize> {
    // Prompt for file names
    println!("Enter file names (separated by spaces):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let file_names: Vec<&str> = input.split_whitespace().collect();

    let mut files_deleted = 0;
    for file_name in file_names {
        let trimmed_path = file_name.trim();

        if Path::new(&trimmed_path).is_dir() {
            println!("One or more of these files are folders");
        } else if Path::new(trimmed_path).exists() {
            remove_file(&trimmed_path)?;
            println!("Deleted {}", trimmed_path);
            files_deleted += 1;
        } else {
            println!("{:?} does not exist", trimmed_path);
        }
    }

    if files_deleted > 0 {
        println!("You've deleted {} files", files_deleted);
    } else {
        println!("Zero files were deleted");
    }

    Ok(files_deleted)
}

pub fn extension(dir: &str, suffixes: &str) -> io::Result<usize> {
    let mut files_deleted = 0;
    let trimmed_dir = dir.trim();
    
    for entry in read_dir(trimmed_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(suffix) = path.extension() {
                if suffix == suffixes {
                    remove_file(&path)?;
                    println!("Deleted {:?}", &path);
                    files_deleted += 1;
                }
            }
        }
    }

    println!("{} were deleted", files_deleted);
    Ok(files_deleted)
}

pub fn look_for(dir: &str, file_name: &str) -> io::Result<()> {
    let _entries = match read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()), // Skip inaccessible directories
    };



    for entry in read_dir(&dir)? {
        let _entries = match &entry {
            Ok(entry) => entry,
            Err(_) => continue,
        }; 

        let entry = entry?;
        let path = entry.path(); 

        if path.is_dir() {
            look_for(path.to_str().unwrap(), file_name)?;
        } else if let Some(name) = path.file_name() {
            if name == file_name {
                println!("Found at {:?}", path)
            }
        }
    }
    Ok(())
}

pub fn copied_to() -> io::Result<()> {
    println!("Enter the path of the file you want copied");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("Enter the path of the new file");
    let mut input2 = String::new();
    stdin().read_line(&mut input2)?;
    let trimmed_input_one = input.trim();

    let old_file_path = Path::new(trimmed_input_one);

    if old_file_path.exists() {
        copy(&old_file_path, &input2)?;
        println!("Copied file to {:?}", &input2);
    }
    Ok(())
}