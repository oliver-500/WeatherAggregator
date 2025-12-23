use crate::org::unibl::etf::configuration::running_environment::RunningEnvironment;
use crate::org::unibl::etf::configuration::settings::Settings;

pub mod settings;
pub mod running_environment;

pub fn get_configuration(config_path : &str) -> Result<Settings, config::ConfigError> {
    for (key, value) in std::env::vars() {
        println!("{} = {}", key, value);
    }

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join(config_path);

    let environment: RunningEnvironment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "prod".into())
        .try_into()
        .expect("Failed to parse required environment variable with name APP_ENV.");

    let environment_filename = format!("configuration_{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
                configuration_directory.join("configuration_base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;

    let s1 = settings.try_deserialize::<Settings>();
    println!("{:?}", s1); //remove later
    s1
}