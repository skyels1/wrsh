use std::io; // obvious
use std::process::Command; // used to take in external commands
use std::env; // changing environment variables
use std::fs; // file system
use std::path::Path; // used to get ls to list files
use std::fs::File;
use std::io::BufReader; // for better cat with big files
use std::io::prelude::*;
use std::process::Stdio; // not even sure what this is for
use sysinfo::System; // system info for fetch

pub fn pipe(left_cmd: &str, right_cmd: &str) {
    let mut left_parts = left_cmd.split_whitespace();
    let mut right_parts = right_cmd.split_whitespace();

    let mut left = Command::new(left_parts.next().unwrap())
        .args(left_parts)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn left command");

    let mut right = Command::new(right_parts.next().unwrap())
        .args(right_parts)
        .stdin(left.stdout.take().unwrap())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn right command");

    right.wait().unwrap();
    left.wait().unwrap();
}

pub fn fetch() -> io::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let hostname = sysinfo::System::host_name().unwrap_or_else(|| "Unknown version".to_string());

    let uptime = sysinfo::System::uptime();
    let uptime_hours = uptime / 3600;
    let uptime_minutes = (uptime % 3600) / 60;

    // for let cpu check on windows needed if empty
    let cpu = {
    let brand = sys.global_cpu_info().brand();
    if brand.is_empty() {
        sys.cpus().first().map(|c| c.brand()).unwrap_or("Unknown CPU") // fix for empty cpu on windows
    } else {
        brand
    }
    };
    let total = sys.total_memory() / 1024 / 1024;
    let used = sys.used_memory() / 1024 / 1024;

    let mem_percent = (used as f32/ total as f32) * 100.0;

    let os_version = sysinfo::System::os_version().unwrap_or_else(|| "Unknown version".to_string());

    println!("Hostname: {}", hostname);
    println!("Uptime: {}h {}m", uptime_hours, uptime_minutes);
    println!("OS: {} {}", std::env::consts::OS, os_version);
    println!("Shell: wrsh");
    println!("CPU: {} ({} cores)", cpu.trim(), sys.cpus().len());
    println!("Ram: {} MiB / {} MiB ({:.1}%)", used, total, mem_percent);

    println!();

    Ok(())
}

pub fn grep<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let pattern = parts.next().unwrap_or(".");
    let path = parts.next().unwrap_or(".");
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            println!("{}", line)
        }
    }

    Ok(())
}

pub fn cat<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("{}", contents);
    Ok(())
}

// function for cd to change to the desired dir
pub fn cd<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    std::env::set_current_dir(path)?;
    Ok(())
}

// function for ls to list the files
pub fn ls<'a>(mut parts: impl Iterator<Item = &'a str>) -> io::Result<()> {
    let path = parts.next().unwrap_or(".");
    let path = Path::new(path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("{}  ", entry.file_name().to_string_lossy());
    }

    println!();
    Ok(())
}

// print working dir
pub fn pwd() -> io::Result<()> {
    let dir = env::current_dir()?;
    println!("{}", dir.display());
    Ok(())
}