use std::collections::HashMap;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Args {
    /// Input city name for which you want to check the weather.
    #[arg(short, long)]
    city_name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct GeocodeList {
    list: Vec<Geocode>  // Jsonのリストが渡ってくるので、リストパースできるようにする
}

#[derive(Serialize, Deserialize, Debug)]
struct Geocode {
    name: String,
    local_names: HashMap<String, String>,   // キーと値をセットで保持している
    lat: f32,
    lon: f32,
    country: String
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    coord: Option<Coord>,
    #[serde(default)]
    weather: Vec<Weather>,
    #[serde(default)]
    base: String,
    main: Option<WeatherMain>,
    #[serde(default)]
    visibility: i64,
    wind: Option<Wind>,
    rain: Option<Rain>,
    clouds: Option<Clouds>,
    #[serde(default)]
    dt: i64,
    sys: Option<WeatherSys>,
    #[serde(default)]
    timezone: i64,
    #[serde(default)]
    id: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    cod: i64
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    #[serde(default)]
    lon: f64,
    #[serde(default)]
    lat: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    #[serde(default)]
    id: i64,
    #[serde(default)]
    main: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    icon: String
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherMain {
    #[serde(default)]
    temp: f64,
    #[serde(default)]
    feels_like: f64,
    #[serde(default)]
    temp_min: f64,
    #[serde(default)]
    temp_max: f64,
    #[serde(default)]
    pressure: i64,
    #[serde(default)]
    humidity: i64,
    #[serde(default)]
    sea_level: i64,
    #[serde(default)]
    grnd_level: i64
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    deg: i64,
    #[serde(default)]
    gust: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Rain {
    #[serde(default)]
    #[serde(alias = "1h")]
    one_hour_rain: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    #[serde(default)]
    all: i64
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherSys {
    #[serde(default)]
    #[serde(alias = "type")]
    weather_type: i64,
    #[serde(default)]
    id: i64,
    #[serde(default)]
    country: String,
    #[serde(default)]
    sunrise: i64,
    #[serde(default)]
    sunset: i64
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let args = Args::parse();

    // geocodingを取得するためのHTTPリクエストを作成
    // OpenWeatherAPIのIDを設定してからHTTPリクエスト。
    let app_id = "xxxxx"; // OpenWeatherAPIのAPIキーを入力してください。
    let request_geocoding = format!("http://api.openweathermap.org/geo/1.0/direct?q={0}&limit=1&appid={app_id}", args.city_name);

    // HTTPクライアントの作成
    let client = reqwest::Client::new();

    // 緯度、経度情報を取得するために、HTTP GETリクエストを投げる
    let geocode_list = client.get(request_geocoding)
            .send()
            .await?
            .json::<GeocodeList>()
            .await?;
    assert!(!geocode_list.list.is_empty() || geocode_list.list.len() == 1);

    let lat = geocode_list.list[0].lat;
    let lon = geocode_list.list[0].lon;
    let request_weather = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={app_id}");
    let weather_data = client.get(request_weather)
            .send()
            .await?
            .json::<WeatherData>()
            .await?;

    // コンソールに今日の天気を出力
    const DEFAULT_KELVIN: f64 = 273.15;
    println!("Today's weather is {}. temp_min: {:.2} °C. temp_max: {:.2} °C.", weather_data.weather[0].main, weather_data.main.as_ref().unwrap().temp_min - DEFAULT_KELVIN, weather_data.main.as_ref().unwrap().temp_max - DEFAULT_KELVIN);

    Ok(())
}