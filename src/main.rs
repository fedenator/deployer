#[macro_use]
extern crate clap;

mod configuration;
mod deploy;

use std::{fs};

use crate::configuration::{Config, DeployConfig};

fn clean_webapp_folder(config: &Config) {
	let paths = fs::read_dir(&config.webapps_folder).unwrap();

	for path in paths {
		let str_path = path.unwrap().path().display().to_string();
		println!("Borrando: {}", str_path);
		if fs::metadata(&str_path).unwrap().is_file() {
			fs::remove_file(&str_path).unwrap();
		} else {
			fs::remove_dir_all(&str_path).unwrap();
		}
	}
}

fn ask_user(text: &str) -> String {
	let mut line = String::new();

	println!("{}", text);
	std::io::stdin().read_line(&mut line).unwrap();

	return line.trim().to_owned();
}

fn main() {

	let matches = clap_app!( Deployer =>
		(version: "0.0.2")
		(author : "Federico Palacios <fedenator7@gmail.com>")
		(about  : "Utilidad para deployear aplicaciones web rapido")
		(@arg FOLDER: -w --web_apps_folder +takes_value
			"Carpeta target para deployear las applicaciones web")
		(@subcommand clean =>
			(about: "Limpia la carpeta target")
		)
		(@subcommand deploy =>
			(about: "Deployea los archivos dados")
			(@arg FILE:      -f --file      +multiple +takes_value "Archivos para deployear")
			(@arg EXTENCION: -e --extencion           +takes_value "Extencion de los empaquetados a deployear (default: war)")
		)
		(@subcommand quick =>
			(about: "Deploy rapido")
			(@arg EXTENCION: -e --extencion           +takes_value "Extencion de los empaquetados a deployear (default: war)")
		)
		(@subcommand test1 =>
			(about: "Test1")
		)
		(@subcommand test2 =>
			(about: "Test2")
		)

		// (@arg FOLDER   : -rp --root_path "Carpeta ")
	).get_matches();

	let config = Config {
		webapps_folder: matches.value_of("web-apps-folder").unwrap_or("/u01/tomcat/tomcat/webapps").to_owned(),
	};

	if let Some(_) = matches.subcommand_matches("clean") {
		clean_webapp_folder(&config);
	}

	if let Some(matches) = matches.subcommand_matches("deploy") {
		let deploy_config = DeployConfig {
			config   : &config,
			extencion: matches.value_of("extencion").unwrap_or("war").to_owned()
		};

		let files: Vec<&str> = matches.values_of("FILE").unwrap().collect();

		for file in files {
			let file_name = deploy::name_of_file(file);

			let mut webapp_name = ask_user( &format!("Nombre para el archivo <{}> (vacio para usar el mismo nombre)", file_name) );
			if webapp_name == "" {
				webapp_name = file_name;
			}

			deploy::deploy_webapp(deploy_config.config, file, &webapp_name)
		}
	}

	if let Some(_) = matches.subcommand_matches("quick") {
		clean_webapp_folder(&config);

		let deploy_config = DeployConfig {
			config   : &config,
			extencion: matches.value_of("extencion").unwrap_or("war").to_owned()
		};

		let wars = deploy::find_wars( std::env::current_dir().unwrap().to_str().unwrap() );

		for war in wars {
			deploy::deploy_webapp(&deploy_config.config, &war, &deploy::name_of_file(&war) );
		}
	}

	if let Some(_) = matches.subcommand_matches("test1") {
		println!("TEST1 encontroado");
	}

	if let Some(_) = matches.subcommand_matches("test2") {
		println!("TEST2 encontrado");
	}
}
