use rand::Rng;
use serde::Serialize;

fn random_data<T>(size: usize, low: T, hight: T) -> Vec<T>
where
    T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd + Copy,
{
    let mut rng = rand::thread_rng();
    let mut data = vec![];
    for _ in 0..size {
        data.push(rng.gen_range(low..hight));
    }

    data
}

#[derive(Serialize)]
pub struct ChartData {
    labels: Vec<String>,
    datasets: Vec<ChartDataset>,
}

impl Default for ChartData {
    fn default() -> Self {
        let mut labels = vec![];
        for num in (10..=100).step_by(10) {
            labels.push(num);
        }

        Self {
            labels: labels.iter().map(|x| x.to_string()).collect(),
            datasets: vec![
                ChartDataset {
                    label: "First".into(),
                    data: random_data(10, 5, 20),
                    border_color: "#ffa600".into(),
                    background_color_top: "transparent".into(),
                    background_color_bottom: "#ffa600".into(),
                    ..Default::default()
                },
                ChartDataset {
                    label: "Second".into(),
                    data: random_data(10, 25, 100),
                    border_color: "#bc5090".into(),
                    background_color_top: "transparent".into(),
                    background_color_bottom: "#bc5090".into(),
                    ..Default::default()
                },
            ],
        }
    }
}

impl ChartData {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Serialize)]
struct ChartDataset {
    label: String,
    data: Vec<i32>,

    #[serde(rename = "borderColor")]
    border_color: String,
    #[serde(rename = "borderWidth")]
    border_width: i32,
    #[serde(rename = "pointRadius")]
    pointer_radius: i32,

    #[serde(rename = "backgroundColor")]
    background_color: String,
    #[serde(
        rename = "backgroundColorTop",
        skip_serializing_if = "String::is_empty"
    )]
    background_color_top: String,
    #[serde(
        rename = "backgroundColorBottom",
        skip_serializing_if = "String::is_empty"
    )]
    background_color_bottom: String,
    fill: bool,

    #[serde(rename = "lineTension")]
    line_tension: f32,
}

impl Default for ChartDataset {
    fn default() -> Self {
        Self {
            label: "".into(),
            data: vec![],

            border_color: "rgba(0, 0, 0, 1)".into(),
            border_width: 2,
            pointer_radius: 0,

            background_color: "rgba(0, 0, 0, 0.33)".into(),
            background_color_top: "".into(),
            background_color_bottom: "".into(),
            fill: true,

            line_tension: 0.35,
        }
    }
}
