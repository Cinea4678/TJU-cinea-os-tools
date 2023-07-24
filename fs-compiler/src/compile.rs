use std::{
    fs::{self, DirEntry},
    path::{self, PathBuf},
    process::exit
};
use std::io::Write;

fn is_directory(entry: &DirEntry) -> usize {
    let file_type = entry.file_type();

    if let Ok(is_directory) = file_type.map(|ft| ft.is_dir()) {
        if is_directory {
            0
        } else {
            1
        }
    } else {
        2
    }
}

pub fn copy_files_recursive<T>(fs_dir: fatfs::Dir<T>, real_path: &path::PathBuf)
where
    T: fatfs::ReadWriteSeek
{
    for entry in fs::read_dir(real_path).unwrap() {
        if let Ok(entry) = entry {
            match is_directory(&entry){
                0=>{
                    let mut sub_path=PathBuf::from(real_path);
                    sub_path.push(entry.file_name());
                    // println!("{:?}->{:?}", entry.file_name(), sub_path);

                    let sub_dir = fs_dir.create_dir(entry.file_name().to_str().unwrap()).unwrap();

                    copy_files_recursive(sub_dir, &sub_path);
                },
                1=>{
                    let mut sub_path=PathBuf::from(real_path);
                    sub_path.push(entry.file_name());
                    let file_content = fs::read(&sub_path).unwrap();
                    // println!("{:?}->{:?}", entry.file_name(), sub_path);

                    let mut sub_file = fs_dir.create_file(entry.file_name().to_str().unwrap()).unwrap();

                    sub_file.truncate().unwrap();
                    sub_file.write_all(file_content.as_slice()).unwrap();
                },
                _=>{}
            }
        }
    }
}

pub fn compile(pathname: &str, output_filename: &str) {
    println!("Compiling {} to {}", pathname, output_filename);

    if let Ok(mut img_file) = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output_filename)
    {
        fatfs::format_volume(&mut img_file, fatfs::FormatVolumeOptions::new().fat_type(fatfs::FatType::Fat32)).unwrap();
        let fs = fatfs::FileSystem::new(&img_file, fatfs::FsOptions::new()).unwrap();
        let root_dir = fs.root_dir();
        let root_path = path::PathBuf::from(pathname);

        copy_files_recursive(root_dir, &root_path);

        fs.unmount().unwrap();
    } else {
        println!("fatal error: cannot open output file '{}'", output_filename);
        exit(1);
    }
}
