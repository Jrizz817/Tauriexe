use libloading::{Library, Symbol};
use std::sync::OnceLock;

static LIB: OnceLock<Library> = OnceLock::new();

pub struct CzkFUNC;

impl CzkFUNC {
    fn get_lib() -> &'static Library {
        LIB.get_or_init(|| {
            unsafe { Library::new("src-tauri/libs/CzkAPI.dll").expect("Falha ao carregar DLL CzkAPI.dll") }
        })
    }

    pub fn execute(script: &str) {
        unsafe {
            let lib = Self::get_lib();
            let func: Symbol<unsafe extern "C" fn(*const u8, usize)> = lib.get(b"Execute").expect("Função Execute não encontrada na DLL");
            func(script.as_ptr(), script.len());
        }
    }

    pub fn inject() {
        unsafe {
            let lib = Self::get_lib();
            let inject_fn: Symbol<unsafe extern "C" fn()> = lib.get(b"Inject").expect("Função Inject não encontrada na DLL");
            inject_fn();
        }
    }
}