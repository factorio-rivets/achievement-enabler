use std::ffi::CString;
use std::panic;
use rivets::defines;
use rivets::detour;
use rivets::Opaque;
use rivets::AsPcstr;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::LibraryLoader::GetProcAddress;

mod luastate;

#[detour(?run@LuaEventDispatcher@@AEAAXW4LuaEventType@@VMapTickType@@P8LuaGameScript@@EAA_NAEBVGameAction@@@Z2@Z)]
fn run(
    this: Opaque,
    lua_event_type: usize,
    map_tick_type: Opaque,
    lua_game_script: Opaque,
    game_action: Opaque,
) {
    let event = defines::events::try_from(&lua_event_type);
    match event {
        Ok(defines::events::on_tick) => {}
        Ok(event) => {
            println!("Rust event handler! {event:?}");
        }
        Err(_) => {
            println!("Unknown event {lua_event_type:?}");
        }
    }
    unsafe {
        back(
            this,
            lua_event_type,
            map_tick_type,
            lua_game_script,
            game_action,
        );
    }
}





#[detour(?valid@LuaSurface@@UEBA_NXZ)]
fn valid(this: Opaque) -> bool {
    println!("bbb!");
    unsafe {
        back(this)
    }
}


fn get_op(lua_state: *mut luastate::lua_State) {
    type lua_gettop = extern "C" fn(lua_state: *mut luastate::lua_State) -> i64;
    unsafe {
        // Get the handle to the parent module (the executable)
        let h_module: HMODULE = GetModuleHandleA(None).unwrap();
        assert!(!h_module.is_invalid(), "Failed to get parent module handle");

        // Specify the mangled function name (you must know it exactly)
        let mangled_name = CString::new("lua_gettop").unwrap();

        // Get the address of the function
        let func_addr = GetProcAddress(h_module, mangled_name.as_pcstr());
        let func_addr = func_addr.map_or_else(|| panic!("Failed to get function address"), |addr| addr);

        // Cast the address to the appropriate function type
        let parent_function: lua_gettop = std::mem::transmute(func_addr);

        // Call the function
        let result = parent_function(lua_state);
        println!("Result from parent function: {result}");
    }
}

#[detour(?luaCountTilesFiltered@LuaSurface@@QEAAHPEAUlua_State@@@Z)]
fn lua_count_tiles_filtered(this: Opaque, lua_state: *mut luastate::lua_State) -> i64 {
    let res = unsafe { back(this, lua_state) };
    println!("lua_count_tiles_filtered!");
    let result = panic::catch_unwind(|| {
        //let lua_state = unsafe { *lua_state };
        get_op(lua_state);
    }).inspect_err(|e| {
        println!("Error: {e:?}");
    });
    println!("res: {res}");
    res
}

rivets::finalize!();
