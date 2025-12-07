use std::rc::Rc;
use charming::{
    Chart, WasmRenderer,
    component::{Legend, LegendType},
    element::{Color, ItemStyle, JsFunction, Label, LabelLine, LineStyle, Orient, TextStyle, Tooltip, Trigger},
    series::Pie,
};
use yew::prelude::*;

use crate::data::{partition_to_name, partition_to_update_id};

#[derive(Properties, PartialEq, Clone)]
pub struct PieChartProps {
    pub master_table: Rc<crate::data::MasterTable>,
    pub partitions: Vec<u8>,
    pub top_n: usize,
    pub chart_id: String,
    pub width: u32,
    pub height: u32,
}

fn compute_title(partitions: &Vec<u8>, default_all_title: &str) -> String {
    if partitions.len() == 1 {
        partition_to_name(*partitions.first().unwrap())
    } else if partitions.len() > 1 && partitions.len() < 27 {
        format!(
            "Update {} ->  Update {}",
            partition_to_update_id(*partitions.first().unwrap()),
            partition_to_update_id(*partitions.last().unwrap())
        )
    } else {
        default_all_title.to_string()
    }
}

fn default_tooltip() -> Tooltip {
    Tooltip::new()
        .trigger(Trigger::Item)
        .background_color("#282c38".to_string())
        .border_width(2)
        .formatter(JsFunction::new_with_args(
            "params",
            r#"
                return `
                <div style="color: #fff; font-family: "TF2Build";">${params.dataIndex+1}. ${params.name}: ${params.percent}%</div>
                `
            "#,
        ))
}

fn default_legend() -> Legend {
    Legend::new()
        .left("58%")
        .width("20%")
        .top("center")
        .height("100%")
        .type_(LegendType::Scroll)
        .orient(Orient::Vertical)
        .text_style(
            TextStyle::new()
                .color(Color::Value("#FFFFFF".to_string()))
                .font_family("TF2Build")
                .font_size(15),
        )
        .item_gap(3)
}

fn default_pie_base() -> Pie {
    Pie::new()
        .center(vec!["28%", "50%"])
        .radius(vec!["25%", "90%"])
        .avoid_label_overlap(true)
        .label(Label::new().show(false).color(Color::Value("#FFFFFF".to_string())))
        .label_line(LabelLine::new().show(false))
        .item_style(ItemStyle::new().border_color("#282c38").border_width(4).border_radius(8))
}

fn build_chart_from_data(chart_data: Vec<(i32, String)>, chart_colors: Vec<Color>) -> Chart {
    let pie = default_pie_base().data(chart_data);
    Chart::new()
        .tooltip(default_tooltip())
        .legend(default_legend())
        .color(chart_colors)
        .series(pie)
}

fn large_tooltip() -> Tooltip {
    default_tooltip()
}

fn large_legend() -> Legend {
    Legend::new()
        .left("90%")
        .width("10%")
        .top("center")
        .height("90%")
        .type_(LegendType::Scroll)
        .orient(Orient::Vertical)
        .text_style(
            TextStyle::new()
                .color(Color::Value("#FFFFFF".to_string()))
                .font_family("TF2Build")
                .font_size(18),
        )
        .item_gap(4)
}

fn large_pie_base() -> Pie {
    Pie::new()
        .center(vec!["50%", "50%"])
        .radius(vec!["25%", "95%"])
        .avoid_label_overlap(true)
        .label(Label::new().show(true).color(Color::Value("#FFFFFF".to_string())).font_family("TF2Build").distance(10).font_size(16))
        .label_line(LabelLine::new().show(true).show_above(true).length(40).line_style(LineStyle::new().width(2.5)))
        .item_style(ItemStyle::new().border_color("#282c38").border_width(4).border_radius(16))
}

fn build_large_chart_from_data(chart_data: Vec<(i32, String)>, chart_colors: Vec<Color>) -> Chart {
    let pie = large_pie_base().data(chart_data);
    Chart::new()
        .tooltip(large_tooltip())
        // .legend(large_legend())
        .color(chart_colors)
        .series(pie)
}

#[function_component(SkillPieChart)]
pub fn skill_pie_chart(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width;
    let height = props.height;

    let name = compute_title(&partitions, "Top 75 Most Used Skills (All Recorded Patches)");

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_skills_chart_vectors(&master_table, &partitions, top_n);

        let chart = build_chart_from_data(chart_data, chart_colors);

        renderer.render(&chart_id_clone, &chart).unwrap();
        Ok::<(), ()>(())
    });

    {
        let render_task = render_task.clone();
        use_effect_with((), move |_| {
            render_task.run();
            || ()
        });
    }

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; color: #fff; margin: 1em; user-select: none;">
            <div style={format!("font-size: {}px; margin-bottom: 1em;", (width as f32).sqrt().round())}>{name}</div>
            <div id={chart_id.clone()} />
        </div>
    }
}

#[function_component(SetPieChart)]
pub fn set_pie_chart(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width;
    let height = props.height;

    let name = compute_title(&partitions, "Top 50 Most Used Sets (All Recorded Patches)");

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_sets_chart_vectors(&master_table, &partitions, top_n);

        let chart = build_chart_from_data(chart_data, chart_colors);

        renderer.render(&chart_id_clone, &chart).unwrap();
        Ok::<(), ()>(())
    });

    {
        let render_task = render_task.clone();
        use_effect_with((), move |_| {
            render_task.run();
            || ()
        });
    }

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; color: #fff; margin: 1em; user-select: none;">
            <div style={format!("font-size: {}px; margin-bottom: 0.5em;", (width as f32).sqrt().round())}>{name}</div>
            <div id={chart_id.clone()} />
        </div>
    }
}

#[function_component(LargeSkillPieChart)]
pub fn large_skill_pie_chart(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width;
    let height = props.height;

    let name = compute_title(&partitions, "Top 75 Most Used Skills");
    let subtitle = format!("taken from top 100 parses on every boss each patch, normalised between patches");

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_skills_normalised_chart_vectors(&master_table, &partitions, top_n);

        let chart = build_large_chart_from_data(chart_data, chart_colors);

        renderer.render(&chart_id_clone, &chart).unwrap();
        Ok::<(), ()>(())
    });

    {
        let render_task = render_task.clone();
        use_effect_with((), move |_| {
            render_task.run();
            || ()
        });
    }

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; color: #fff; margin: 1em; user-select: none;">
            <div style={format!("font-size: {}px; margin-bottom: 0.25em;", (width as f32).sqrt().round() * 1.2)}>{name}</div>
            <div style={format!("font-size: {}px; margin-bottom: 2em;", (width as f32).sqrt().round() * 0.5)}>{subtitle}</div>
            <div style="margin:2em;" id={chart_id.clone()} />
        </div>
    }
}

#[function_component(LargeSetPieChart)]
pub fn large_set_pie_chart(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width;
    let height = props.height;

    let name = compute_title(&partitions, "Top 50 Most Used Sets");
    let subtitle = format!("taken from top 100 parses on every boss each patch, normalised between patches");

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_sets_normalised_chart_vectors(&master_table, &partitions, top_n);

        let chart = build_large_chart_from_data(chart_data, chart_colors);

        renderer.render(&chart_id_clone, &chart).unwrap();
        Ok::<(), ()>(())
    });

    {
        let render_task = render_task.clone();
        use_effect_with((), move |_| {
            render_task.run();
            || ()
        });
    }

    html! {
        <div style="display: flex; flex-direction: column; align-items: center; color: #fff; margin: 1em; user-select: none;">
            <div style={format!("font-size: {}px; margin-bottom: 0.25em;", (width as f32).sqrt().round() * 1.2)}>{name}</div>
            <div style={format!("font-size: {}px; margin-bottom: 2em;", (width as f32).sqrt().round() * 0.5)}>{subtitle}</div>
            <div style="margin:2em;" id={chart_id.clone()} />
        </div>
    }
}
