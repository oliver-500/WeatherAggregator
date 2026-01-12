
#[derive(Clone, Debug)]
pub struct RetrieveCurrentWeatherCacheRequest{
    pub location_name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub state: Option<String>,

}