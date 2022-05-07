use mlua::Lua;
use mlua::prelude::LuaResult;

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

    lua.load("\
    local test = function() return 100 end
    print(test() * 400)").exec()?;

    Ok(())
}
