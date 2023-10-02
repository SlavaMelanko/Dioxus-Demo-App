use dioxus::prelude::*;

use crate::chart::ChartData;

const CHART_JS_4_4_0_MIN: &str = include_str!("../../chart.js.4.4.0.min.js");

const UTILS_FOR_CHART_JS: &str = r#"
    function getGradient(ctx, chartArea, topColor, bottomColor = "transparent") {
        let gradient = ctx.createLinearGradient(0, chartArea.bottom, 0, chartArea.top);
        gradient.addColorStop(1, bottomColor);
        gradient.addColorStop(0, topColor);
        return gradient;
    }

    function updateChartData(chartData) {
        for (let i = 0; i < chartData.datasets.length; i++) {
            let topColor = chartData.datasets[i].backgroundColorTop;
            let bottomColor = chartData.datasets[i].backgroundColorBottom;
            if (topColor && bottomColor) {
                chartData.datasets[i].backgroundColor = function (context) {
                    const chart = context.chart;
                    const { ctx, chartArea } = chart;
                    if (!chartArea) return; // happens on the initial chart load
                    return getGradient(ctx, chartArea, topColor, bottomColor);
                };
            }
        }
    }
"#;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let eval_provider = use_eval(cx);
    let _future = use_future(cx, (), |_| {
        to_owned![eval_provider];
        async move {
            let eval = eval_provider(
                r#"
                    let chartData = await dioxus.recv();

                    updateChartData(chartData)

                    let theChart = new Chart(document.getElementById("line-chart"), {
                        type: "line",
                        data: chartData,
                        options: {},
                    });

                    return true;
                "#,
            )
            .unwrap();

            eval.send(ChartData::default().to_json()).unwrap();

            match eval.await {
                Ok(v) => info!("Chart created: {:?}", v),
                Err(e) => error!("Chart creation failed: {:?}", e),
            }
        }
    });

    cx.render(rsx! {
        canvas {
            id: "line-chart",
            width: 620, // TODO: Use absolute position?
        }
        script {
            "{CHART_JS_4_4_0_MIN}",
        }
        script {
            "{UTILS_FOR_CHART_JS}",
        }
    })
}
