use serde::Serialize;

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
            datasets: vec![ChartDataset {
                label: "First".into(),
                data: vec![55, 125, 90, 220, 410, 250, 310, 290, 102, 117],
                border_color: "rgb(255, 192, 203)".into(),
                background_color_top: "rgba(255, 192, 203, 0.75)".into(),
                background_color_bottom: "rgba(55, 12, 203, 0.1)".into(),
                ..Default::default()
            }],
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
