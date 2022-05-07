use std::fmt::{Display, Formatter};
use mlua::{Lua, MultiValue, Value};
use mlua::prelude::LuaResult;
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn print_multivalue(m: MultiValue) {
    let values = m.into_vec();

    for value in values {
        match value {
            Value::Nil => println!("nil"),
            Value::Boolean(b) => println!("{}", b),
            Value::LightUserData(lu) => println!("{:?}", lu),
            Value::Integer(i) => println!("{}", i),
            Value::Number(n) => println!("{}", n),
            Value::String(str) => println!("{}", str.to_str().unwrap()),
            Value::Table(tbl) => println!("{:?}", tbl),
            Value::Function(func) => println!("{:?}", func.info().what),
            Value::Thread(th) => println!("{:?}", th),
            Value::UserData(ud) => println!("{:?}", ud),
            Value::Error(e) => println!("{:?}", e),
            _ => println!("Can not print!"),
        }
    }
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let load_library = lua.create_function(|d, library: String| {
        unsafe {
            let lib = libloading::Library::new(library.as_str()).unwrap();
            let func :libloading::Symbol<unsafe extern fn()> = lib.get(b"load_rust").unwrap();
            func();
        }
        println!("Loading: {}", library);
        Ok(())
    })?;

    lua.globals().set("load_library", load_library);

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = lua.load(line.as_str()).eval::<MultiValue>()?;
                print_multivalue(result);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}
