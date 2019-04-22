use std::{ fs };
use crate::configuration::{ Config };

/// Obtiene el path a todos los .war recursivamente desde el directorio dado
///
/// ### Argguments
/// * `root_path` - Path al directorio para empezar a buscar
pub fn find_wars(root_path: &str) -> Vec<String> {
	let mut wars = Vec::new();

 	let paths = fs::read_dir(root_path).unwrap();

 	for path in paths {
 		let entry    = path.unwrap();
 		let str_path = entry.path().display().to_string();

 		if fs::metadata(&str_path).unwrap().is_file() {

 			if str_path.ends_with(".war") {
 				wars.push(str_path);
 			}

 		} else {
 			wars.append( &mut find_wars(&str_path) );
 		}
 	}

 	return wars;
}

pub fn deploy_webapp(config: &Config, path: &str, nombre: &str) {
	let to = format!("{0}/{1}", &config.webapps_folder, nombre);
	println!("Copiando from: <{0}> to: <{1}>", path, to);
	fs::copy(path, to).unwrap();
}

pub fn name_of_file(full_path: &str) -> String {
	let last_slash = full_path.rfind('/');
	return full_path.split_at( last_slash.unwrap() + 1 ).1.to_string();
}
