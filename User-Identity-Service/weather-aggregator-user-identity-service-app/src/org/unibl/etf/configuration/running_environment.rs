pub enum RunningEnvironment {
    Development,
    CI,
    CD,
    DevelopmentLocal
}

impl RunningEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "dev",
            Self::CI => "ci",
            Self::CD => "cd",
            Self::DevelopmentLocal => "dev_local",
        }
    }
}

impl TryFrom<String> for RunningEnvironment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Self::Development),
            "ci" => Ok(Self::CI),
            "cd" => Ok(Self::CD),
            "dev_local" => Ok(Self::DevelopmentLocal),
            other => Err(format!(
                "{} is not a valid value for `env`. Use either dev, ci, cd or dev_local",
                other
            )),
        }
    }
}