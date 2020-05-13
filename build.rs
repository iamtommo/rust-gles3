use std::fs::File;
use std::{io};

use gl_generator::{Api, Fallbacks, Generator, Profile, Registry};

fn main() {
    let mut file = File::create("src/bindings.rs").expect("file create failed");
    /*let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();*/

    Registry::new(
        Api::Gles2,
        (3, 2),
        Profile::Core,
        Fallbacks::All,
        [],
    ).write_bindings(FfiGen, &mut file).expect("write bindings failed");
}

pub struct FfiGen;

impl Generator for FfiGen {
    fn write<W>(&self, registry: &Registry, w: &mut W) -> io::Result<()> where W: io::Write {
        //writeln!(w, "#![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]\n")?;
        writeln!(w, "use std::os::raw::*;")?;
        //writeln!(w, "use std::ptr::null;")?;
        writeln!(w, "use gl::types::*;")?;

        write_consts(registry, w);

        // GetProcAddr:
        /*writeln!(w, r#"pub fn get_proc_addr(s: &str) -> *const c_void {{ match s {{"#)?;
        for cmd in &registry.cmds {
            let name = &cmd.proto.ident;
            writeln!(w, r#""gl{}" => {} as *const c_void,"#, name, name)?;
        }
        writeln!(w, r#"_ => null(),}} }}"#)?;*/

        // Stubs:
        /*for cmd in &registry.cmds {
            writeln!(w, r#"pub extern "system" fn {}("#, cmd.proto.ident)?;
            for param in &cmd.params {
                writeln!(w, r#"{}: {},"#, param.ident, clean_type(&param.ty))?;
            }
            writeln!(w, r#") -> {} {{ unimplemented!() }}"#, clean_type(&cmd.proto.ty))?;
        }
*/


        write!(w, "\n\n\n")?;

        writeln!(w, "#[link(name = \"GLESv3\")]")?;
        writeln!(w, "extern \"C\" {{")?;
        for cmd in &registry.cmds {
            writeln!(w, r#"pub fn {}("#, fix_ident(&cmd.proto.ident))?;
            for param in &cmd.params {
                writeln!(w, r#"{}: {},"#, param.ident, clean_type(&param.ty))?;
            }
            writeln!(w, r#") -> {};"#, clean_type(&cmd.proto.ty))?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }
}

fn fix_ident(s: &String) -> String {
    format!("gl{}", s)
}

fn clean_type(s: &str) -> String {
    s.replace("types::", "").replace("__gl_imports::raw::", "")
}

fn write_consts<W>(registry: &Registry, dest: &mut W) where W: io::Write, {
    //writeln!(dest, r#"pub mod types {{"#)?;

    for alias in &registry.enums {
        writeln!(dest, "pub const GL_{}: {} = {};", alias.ident, alias.ty, alias.value);
    }

    //writeln!(dest, "}}")?;
}