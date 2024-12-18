use std::io::{self, Read, Write};
use std::{fs, path};
use std::path::{Path, PathBuf};
use flate2::{Compression, read::GzDecoder, write::GzEncoder};

const SIZE_BLOCK: usize = 512;



fn pack(input: &str, gzcompres: bool) -> io::Result<()> {
    let path_i = Path::new(input);
    let mut info = Vec::new();
    let output = if gzcompres 
    {
        format!("{}.tar.gz", path_i.file_name().unwrap().to_str().unwrap())
    } 
    else 
    {
        format!("{}.tar", path_i.file_name().unwrap().to_str().unwrap())
    };


    fn add(info: &mut Vec<u8>, path: &Path, base: &Path) -> io::Result<()> {

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                add(info, &entry.path(), base)?;
            }
        }
         else 
        {
            let content = fs::read(path)?;
            let relative_path = path.strip_prefix(base).unwrap().to_str().unwrap();
            let header = create_header(relative_path, content.len() as u64);

            info.extend(header);
            info.extend(content);
            size_b(info);
        }
        Ok(())
    }

    add(&mut info, path_i, path_i)?;

    let path_o = Path::new(&output);

    if gzcompres {
        let file = fs::File::create(path_o)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(&info)?;
        encoder.finish()?;
    } else {
        let mut file = fs::File::create(path_o)?;
        file.write_all(&info)?;
    }

    println!(input, output);
    Ok(())
}


fn main() -> io::Result<()> 
{
    let args: Vec<String> = std::env::args().collect();

    match args[1].as_str() {
        "pack" => {
            let gzcompres = args.contains(&"-c".to_string());
            pack(&args[2], gzcompres)
        }
        
    }
}
