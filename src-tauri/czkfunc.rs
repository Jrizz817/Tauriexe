use libloading::{Library, Symbol};
use std::sync::OnceLock;
use std::path::PathBuf;

static LIB: OnceLock<Library> = OnceLock::new();

pub struct CzkFUNC;

impl CzkFUNC {
    fn get_lib() -> &'static Library {
        LIB.get_or_init(|| {
            let path: PathBuf = std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("src-tauri/libs/CzkAPI.dll");
            unsafe { Library::new(path).expect("Falha ao carregar a DLL") }
        })
    }

    pub fn execute(script: &str) {
        unsafe {
            let lib = Self::get_lib();
            let func: Symbol<unsafe extern "C" fn(*const u8, usize)> =
                lib.get(b"execute").expect("Função 'execute' não encontrada");
            func(script.as_ptr(), script.len());
        }
    }

    pub fn inject() {
        unsafe {
            let lib = Self::get_lib();
            let func: Symbol<unsafe extern "C" fn()> =
                lib.get(b"inject").expect("Função 'inject' não encontrada");
            func();
        }
    }
}
