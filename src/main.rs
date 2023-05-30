use std::{fs, path::Path};
use walkdir::WalkDir;
use zip;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let mut input = String::new();

    println!("Enter the path to the folder you want to unzip: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let root = input.trim();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let filename = entry.file_name().to_str().unwrap();
        println!("Filename: {}", filename);
        let path = entry.path();

        if path.extension().unwrap() == "zip" {
            unzip_file(path);
        } else {
            println!("Found file: {}", path.to_string_lossy());
        }
    }

    return 0;
}

fn unzip_file(file_path: &Path) {
    println!("Unzipping file: {}", file_path.to_string_lossy());
    let found_archieve = zip::ZipArchive::new(fs::File::open(&file_path).unwrap());

    let root = file_path.parent().unwrap();

    match found_archieve {
        Ok(mut found_archieve) => {
            for i in 0..found_archieve.len() {
                let mut file = found_archieve.by_index(i).unwrap();
                let outpath = match file.enclosed_name() {
                    Some(path) => root.join(path.to_owned()),
                    None => continue,
                };

                if (&*file.name()).ends_with('/') {
                    println!("File {} extracted to \"{}\"", i, outpath.display());
                    fs::create_dir_all(&outpath).unwrap();
                } else {
                    println!(
                        "File {} extracted to \"{}\" ({} bytes)",
                        i,
                        outpath.display(),
                        file.size()
                    );
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(&p).unwrap();
                        }
                    }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    std::io::copy(&mut file, &mut outfile).unwrap();
                }
                if (&*file.name()).ends_with(".zip") {
                    unzip_file(&outpath);
                }
            }
            fs::remove_file(file_path).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
