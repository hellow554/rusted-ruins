
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use common::obj::Object;
use common::pakutil::write_object;
use toml::de::from_str;
use tar;
use error::*;
use verbose::print_verbose;
use dir;

use tomlinput::TomlInput;
use buildobj::build_object;

pub fn compile(files: &[&str], output_file: &String) {
    let out = File::create(output_file).unwrap();
    let mut builder = tar::Builder::new(out);
    
    for f in files {
        let f = Path::new(f);
        if f.is_relative() {
            dir::set_src_dir(f.parent());
        }else{
            dir::set_src_dir(None);
        }
        
        let obj = match read_toml(f) {
            Ok(o) => o,
            Err(echain) => {
                eprintln!("Cannot process \"{}\"", f.to_string_lossy());
                for e in echain.iter_chain() {
                    eprintln!("{}", e);
                }
                continue;
            }
        };
        let v = write_to_vec(&obj).unwrap();
        write_data_to_tar(&mut builder, &v, &obj.get_id());
    }
    builder.finish().unwrap();
}

fn read_toml<P: AsRef<Path>>(path: P) -> Result<Object, Error> {
    let s = {
        let mut f = File::open(path.as_ref())?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        s
    };

    let tomlinput: TomlInput = from_str(&s)?;

    print_verbose(|| format!("Processing \"{:?}\"", path.as_ref()));
    print_verbose(|| format!("{:?}", tomlinput));
    let object = build_object(tomlinput)?;

    Ok(object)
}

fn write_to_vec(obj: &Object) -> Result<Vec<u8>, Error> {
    let mut v = Vec::new();
    match write_object(&mut v, obj) {
        Ok(_) => Ok(v),
        Err(e) => bail!(PakCompileError::ObjWriteError { description: e }),
    }
}

fn write_data_to_tar<W: Write>(builder: &mut tar::Builder<W>, data: &[u8], path: &str) {
    let mut header = tar::Header::new_gnu();
    header.set_path(path).unwrap();
    header.set_size(data.len() as u64);
    header.set_mtime(0);
    header.set_cksum();

    builder.append(&header, data).unwrap();
}

