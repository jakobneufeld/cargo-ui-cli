#![feature(decl_macro)]
#![feature(with_options)]
use rocket::*;
use toml::*;
use toml::Value;
use std::collections::HashMap;
use toml_edit::*;
use std::fs::*;
use std::io::Write;
mod cors;
use rocket::response::content;

#[get("/")]
fn index() -> String {
    String::from("hey")
}

#[get("/addpkg/<pkgname>")]
fn addpkg(pkgname: String) ->  String {
    let mut doc = read_to_string("Cargo.toml").unwrap().parse::<Document>().unwrap();
    doc["dependencies"][pkgname.as_str()] = value("*");
    let mut cargo = std::fs::File::with_options();
    cargo.write(true);
    let mut cargo_file = cargo.open("Cargo.toml");
    cargo_file.unwrap().write(doc.to_string().as_bytes());
    String::from("Ok")
   
}
#[get("/readpkg")]
fn readpkg() -> content::Json<String> {
    let cargotoml =  read_to_string("Cargo.toml").unwrap();
    
 
let mut  value = cargotoml.parse::<Value>().unwrap();
let mut hmap = Vec::<HashMap::<String, String>>::new();
let depenciestable = & mut value["dependencies"].as_table_mut().unwrap().into_iter();
for dep in depenciestable {
    match dep.1 {
        toml::Value::String(s) => {
        let mut valmap = HashMap::new();
        valmap.insert(dep.0.clone(), s.clone());
        hmap.push(valmap);
        }
        toml::Value::Table(t)  => {
        let mut valmap = HashMap::new();
        valmap.insert(dep.0.clone(), t.get("version").unwrap().as_str().unwrap().to_string().clone());
        hmap.push(valmap );
        }
        _ => {
            
        }
    }
  }
  let a =  serde_json::ser::to_string(&hmap).unwrap();
content::Json(a)

}
#[get("/editpkg/<pkgname>/<versiona>")]
fn editpkg(pkgname: String, versiona: String) ->  String {
    let mut doc = read_to_string("Cargo.toml").unwrap().parse::<Document>().unwrap();
    doc["dependencies"][pkgname.as_str()] = toml_edit::value( versiona.as_str());
    let mut cargo = std::fs::File::with_options();
    cargo.write(true);
    let mut cargo_file = cargo.open("Cargo.toml");
    cargo_file.unwrap().write(doc.to_string().as_bytes());
    String::from("Ok")
   
}



fn main() {
    rocket::ignite().mount("/", routes!(index,addpkg, readpkg, editpkg)).attach(cors::CORS()).launch();
}
