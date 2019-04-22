pub struct Config {
    pub webapps_folder: String,
}

pub struct DeployConfig<'config> {
    pub config   : &'config Config,
    pub extencion: String
}
