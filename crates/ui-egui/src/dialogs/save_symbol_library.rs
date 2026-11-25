//! Save Symbol Library: a name and where it goes: the user library folder (listed under Window →
//! Symbol Libraries → User Defined) or a `.vectorz` file picked in a save dialog (a download
//! on the web). OK runs `symbol.saveLibrary`. Fields: see [`super::save_library`].

use serde_json::Value;
use vectorz_engine::cmd::symbollib::SYMBOLS_EXT;

use super::save_library::{self, Library};
use crate::VectorzApp;

/// The dialog kind of Save Symbol Library.
pub const KIND: &str = "saveSymbolLibrary";

/// Open Save Symbol Library for the document's symbols (`names`: the one selected in the panel).
pub fn open(app: &mut VectorzApp, names: Vec<String>) -> Result<Value, String> {
    let library = Library {
        key: KIND,
        title: tl!("Save Symbol Library").to_string(),
        cmd: "symbol.saveLibrary",
        only: tl!("Selected Symbols Only").to_string(),
        ext: |_| SYMBOLS_EXT.to_string(),
        has_user_dir: app.session.symbol_libraries.user_dir().is_some(),
    };
    save_library::open(app, library, names, vec![])
}
