// Crate linkage metadata
#[link(name = "xml", vers = "0.1", author = "DanielFath", package_id="xml")];

//Metadata
#[desc = "XML pull parser for rust"];
#[licence = "MIT"];
#[author = "DanielFath"];
#[crate_type = "lib"];


// Forbidden things
#[forbid(non_camel_case_types)];
#[forbid(non_uppercase_statics)];
#[forbid(unreachable_code)];


// Warn on missing docs
#[warn(unnecessary_qualification)];
#[warn(managed_heap_memory)];
#[warn(missing_doc)];
//#[warn(owned_heap_memory)];

pub use xml_parser::*;
pub use xml_node::*;
pub use std::io::*;
pub use xml_lexer::*;

pub mod xml_parser;
pub mod xml_node;
pub mod util;
pub mod xml_lexer;

fn main() {
    error!("This is an error log");
    warn!("This is a warn log");
    info!("this is an info log");
    debug!("This is a debug log");
}