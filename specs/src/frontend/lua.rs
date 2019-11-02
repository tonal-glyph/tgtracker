//@ - [ ] wu language compiles to Lua. It's inspired by Moonscript and has features like Traits

#![allow(unused_imports)]
#[cfg(test)]
use include_lua::*;
use rlua::Lua;
#[test]
fn test_lua() -> rlua::Result<()> {
    let lua = Lua::new();
    let modules = include_lua!("lib_name": "scripts");
    lua.context(|ctx| -> rlua::Result<()> {
        ctx.add_modules(modules)?;
        println!("{}", ctx.load("require('test')").eval::<String>()?);
        Ok(())
    })
}
