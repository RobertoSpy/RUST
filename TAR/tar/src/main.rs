use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const BLOCK: usize = 512;

fn padd(data: &mut Vec<u8>) {
    let current_size = data.len();
    let remainder = current_size % BLOCK;

    if remainder != 0 {
        let padding = BLOCK - remainder;
        for _ in 0..padding {
            data.push(0);
        }
    }
}

fn create_tar_header(name: &str, size: u64) -> Result<Vec<u8>, io::Error> {
    let mut header = vec![0u8; 512];

    let name_bytes = name.as_bytes();
    if name_bytes.len() > 100 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Name to long"));
    }

    for (i, &byte) in name_bytes.iter().enumerate() {
        header[i] = byte;
    }

    let size_octal = format!("{:o}", size);
    let size_start = 124;
    for (i, &byte) in size_octal.as_bytes().iter().enumerate() {
        header[size_start + i] = byte;
    }

    let typeflag = if name.ends_with("/") { b'5' } else { b'0' };
    header[156] = typeflag;

    let mut checksum: u64 = 0;
    for &byte in &header {
        checksum += byte as u64;
    }

    checksum += 8 * b' ' as u64;

    let checksum_octal = format!("{:o}", checksum);
    let checksum_start = 148; 
    for (i, &byte) in checksum_octal.as_bytes().iter().enumerate() {
        header[checksum_start + i] = byte;
    }

    Ok(header)
}

fn pack(input: &str, compress: bool) -> io::Result<()> {
    let input_path = Path::new(input);

    if !input_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "path"));
    }

    let output = if compress {
        match input_path.file_name() {
            Some(name) => match name.to_str() {
                Some(valid_name) => format!("{}.tar.gz", valid_name),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Name file no valid UTF-8",
                    ));
                }
            },
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Name file no valid",
                ));
            }
        }
    } else {
        match input_path.file_name() {
            Some(name) => match name.to_str() {
                Some(valid_name) => format!("{}.tar", valid_name),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Name file no UTF-8",
                    ));
                }
            },
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Name file no valid",
                ));
            }
        }
    };

    let mut tar_data = Vec::new();

    fn add_to_tar(tar_data: &mut Vec<u8>, path: &Path, base: &Path) -> io::Result<()> {
        if path.is_dir() {
            let relative_path = match path.strip_prefix(base) {
                Ok(rel_path) => rel_path.to_str().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Relative path no UTF-8")
                })?,
                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Relative path")),
            };

            let dir_name = format!("{}/", relative_path);
            let header = create_tar_header(&dir_name, 0)?;
            tar_data.extend(header);
            padd(tar_data);

            for entry in fs::read_dir(path)? {
                let entry = entry?;
                add_to_tar(tar_data, &entry.path(), base)?;
            }
        } else {
            let content = fs::read(path)?;
            let relative_path = match path.strip_prefix(base) {
                Ok(rel_path) => rel_path.to_str().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Relative path no UTF-8")
                })?,
                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Relative path")),
            };

            let header = create_tar_header(relative_path, content.len() as u64)?;
            tar_data.extend(header);
            tar_data.extend(content);
            padd(tar_data);
        }
        Ok(())
    }

    add_to_tar(&mut tar_data, input_path, input_path)?;

    let output_path = Path::new(&output);
    if compress {
        let file = fs::File::create(output_path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&tar_data)?;
        encoder.finish()?;
    } else {
        let mut file = fs::File::create(output_path)?;
        file.write_all(&tar_data)?;
    }

    println!("Files from '{}' packed in '{}'", input, output);
    Ok(())
}

fn get_output_dir(input_path: &Path) -> PathBuf {
    let file_stem = match input_path.file_stem() {
        Some(stem) => stem.to_str().unwrap_or_else(|| {
            eprintln!(
                "Warning: Invalid file name in path '{}'",
                input_path.display()
            );
            ""
        }),
        None => {
            eprintln!("Error: No file name in path '{}'", input_path.display());
            return PathBuf::new();
        }
    };

    let name_without_gz = if file_stem.ends_with(".tar") {
        let trimmed_name = &file_stem[..file_stem.len() - 4];
        trimmed_name.to_string()
    } else {
        file_stem.to_string()
    };

    let output_dir = Path::new(&name_without_gz);

    output_dir.to_path_buf()
}

fn unpack(input: &str) -> io::Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input file does not exist",
        ));
    }

    let tar_data = if input_path.extension().map_or(false, |ext| ext == "gz") {
        let file = fs::File::open(input_path)?;
        let mut decoder = GzDecoder::new(file);
        let mut decoded_data = Vec::new();
        decoder.read_to_end(&mut decoded_data)?;
        decoded_data
    } else {
        fs::read(input_path)?
    };

    let output_dir = get_output_dir(input_path);

    fs::create_dir_all(&output_dir)?;

    let mut cursor = io::Cursor::new(tar_data);

    loop {
        let mut header = vec![0u8; BLOCK];
        if let Err(e) = cursor.read_exact(&mut header) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                break;
            } else {
                eprintln!("Error reading tar header: {}", e);
                return Err(e);
            }
        }

        if header.iter().all(|&byte| byte == 0) {
            break;
        }

        let name = {
            let raw_name = &header[..100];
            let name = String::from_utf8_lossy(raw_name)
                .trim_matches(char::from(0))
                .to_string();
            if name.is_empty() {
                eprintln!("Error: Found an empty file name in the archive.");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Empty file name in tar archive",
                ));
            }

            if name.contains('\0') || name.contains('/') && name.trim().is_empty() {
                eprintln!("Error: Invalid file name detected: '{}'", name);
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid file name in tar archive",
                ));
            }
            name
        };

        let size = {
            let raw_size = &header[124..136];
            let size_octal_str = String::from_utf8_lossy(raw_size).into_owned();
            let size_octal = size_octal_str.trim_matches(char::from(0));

            match u64::from_str_radix(size_octal, 8) {
                Ok(size) => size,
                Err(_) => {
                    eprintln!(
                        "Warning: Invalid size value in tar header: '{}'",
                        size_octal
                    );
                    0
                }
            }
        };

        let mut content = vec![0u8; size as usize];
        if let Err(e) = cursor.read_exact(&mut content) {
            eprintln!("Error reading content for file {}: {}", name, e);
            return Err(e);
        }

        let padding_size = (BLOCK - (size as usize % BLOCK)) % BLOCK;
        if padding_size > 0 {
            let mut padding = vec![0u8; padding_size];
            if let Err(e) = cursor.read_exact(&mut padding) {
                eprintln!("Error reading padding for file {}: {}", name, e);
                return Err(e);
            }
        }

        let output_path = output_dir.join(PathBuf::from(name));

        if header[156] == b'5' {
            fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            if let Err(e) = fs::write(&output_path, &content) {
                eprintln!("Error writing file {}: {}", output_path.display(), e);
                return Err(e);
            }
        }
    }

    println!("Unpacked {} into {}", input, output_dir.display());
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Error: Insufficient arguments.");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Insufficient arguments",
        ));
    }

    match args[1].as_str() {
        "pack" => {
            let compress = args.contains(&"-c".to_string());
            pack(&args[2], compress).map_err(|e| {
                eprintln!("Failed to pack: {}", e);
                e
            })
        }
        "unpack" => unpack(&args[2]).map_err(|e| {
            eprintln!("Failed to unpack: {}", e);
            e
        }),
        _ => {
            eprintln!("Invalid command: {}", args[1]);
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid command",
            ))
        }
    }
}
