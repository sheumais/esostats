use std::rc::Rc;
use charming::{
    Chart, WasmRenderer, component::{Axis, Legend, LegendType}, datatype::DataPointItem, element::{AxisLabel, Color, ItemStyle, JsFunction, Label, LabelAlign, LabelLine, LabelPosition, LineStyle, NameLocation, Orient, TextStyle, Tooltip, Trigger}, series::{Bar, Pie}
};
use yew::prelude::*;

use crate::{data::{Player, partition_to_name, partition_to_update_id}, players::hodor_name_to_html};

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

fn percent_tooltip() -> Tooltip {
    Tooltip::new()
        .trigger(Trigger::Item)
        .background_color("#282c38".to_string())
        .border_width(2)
        .formatter(JsFunction::new_with_args(
            "params",
            r#"
                return `
                <div style="color: #fff; font-family: "TF2Build";">
                    ${params.dataIndex + 1}. ${params.name}: ${params.value.toFixed(1)}%
                </div>
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

pub fn build_large_bar_graph_from_data(chart_data: Vec<(f64, String)>, chart_colors: Vec<Color>) -> Chart {
    let labels: Vec<String> = chart_data.iter().map(|(_, lbl)| lbl.clone()).collect();

    let data_points: Vec<DataPointItem> = chart_data
        .iter()
        .enumerate()
        .map(|(idx, (val, _))| {
            let color = chart_colors.get(idx).cloned().unwrap_or(Color::from("#000000"));
            DataPointItem::new(*val).item_style(ItemStyle::new().color(color))
        })
        .collect();

    let chart = Chart::new()
        .x_axis(Axis::new().data(labels).name("Set Name").name_location(NameLocation::Center).name_text_style(TextStyle::new().font_family("TF2Build").font_size(24).color("#fff")).axis_label(AxisLabel::new().show(false)))
        .y_axis(Axis::new().name("Percent of Parses Using The Set").name_location(NameLocation::Center).name_gap(45).name_text_style(TextStyle::new().font_family("TF2Build").font_size(24).color("#fff")).max_interval(10).max(100).axis_label(AxisLabel::new().color("#fff").font_family("TF2Build").font_size(16)))
        .series(Bar::new().data(data_points).label(Label::new().show(true).position(LabelPosition::Top).align(LabelAlign::Left).color("#fff").font_family("TF2Build").font_size(16).formatter("  {b}").rotate("35").offset((-10, 0))))
        .color(chart_colors)
        .tooltip(percent_tooltip())
        .legend(Legend::new().show(false));

    chart

}

pub fn build_large_bar_graph_skills_from_data(chart_data: Vec<(f64, String)>, chart_colors: Vec<Color>) -> Chart {
    let labels: Vec<String> = chart_data.iter().map(|(_, lbl)| lbl.clone()).collect();

    let data_points: Vec<DataPointItem> = chart_data
        .iter()
        .enumerate()
        .map(|(idx, (val, _))| {
            let color = chart_colors.get(idx).cloned().unwrap_or(Color::from("#000000"));
            DataPointItem::new(*val).item_style(ItemStyle::new().color(color))
        })
        .collect();

    let chart = Chart::new()
        .x_axis(Axis::new().data(labels).name("Skill Name").name_location(NameLocation::Center).name_text_style(TextStyle::new().font_family("TF2Build").font_size(24).color("#fff")).axis_label(AxisLabel::new().show(false)))
        .y_axis(Axis::new().name("Percent of Parses Using The Skill").name_location(NameLocation::Center).name_gap(45).name_text_style(TextStyle::new().font_family("TF2Build").font_size(24).color("#fff")).max_interval(10).max(100).axis_label(AxisLabel::new().color("#fff").font_family("TF2Build").font_size(16)))
        .series(Bar::new().data(data_points).label(Label::new().show(true).position(LabelPosition::Top).align(LabelAlign::Left).color("#fff").font_family("TF2Build").font_size(16).formatter("  {b}").rotate("35").offset((-10, 0))))
        .color(chart_colors)
        .tooltip(percent_tooltip())
        .legend(Legend::new().show(false));

    chart

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

    let name = compute_title(&partitions, "Top 75 Most Used Skills (All Patches)");

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

    let name = compute_title(&partitions, "Top 50 Most Used Sets (All Patches)");

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

    let name = compute_title(&partitions, "Top 75 Most Used Skills (All Patches)");
    let subtitle = format!("data from top 100 parses on every boss in every patch on esologs, normalised between patches");

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

    let name = compute_title(&partitions, "Top 50 Most Used Sets (All Patches)");
    let subtitle = format!("data from top 100 parses on every boss in every patch on esologs, normalised between patches");

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

#[function_component(LargeSetsBarGraph)]
pub fn large_bar_graph(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width;
    let height = props.height;

    let name = format!("Percentage of Boss Parses Using Each Set (U{})", partition_to_update_id(partitions[0]));
    let subtitle = format!("data from top 100 parses on every boss this patch");

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_sets_percentage_chart_vectors(&master_table, &partitions, top_n);

        let chart = build_large_bar_graph_from_data(chart_data, chart_colors);

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
            <div style={format!("font-size: {}px; margin-bottom: 1em;", (width as f32).sqrt().round() * 0.5)}>{subtitle}</div>
            <div style="margin:2em;" id={chart_id.clone()} />
        </div>
    }
}

#[function_component(LargeSkillsBarGraph)]
pub fn large_bar_graph(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width;
    let height = props.height;

    let name = format!("Percentage of Boss Parses Using Each Skill (U{})", partition_to_update_id(partitions[0]));
    let subtitle = format!("data from top 100 parses on every boss this patch");

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_skills_percentage_chart_vectors(&master_table, &partitions, top_n);

        let chart = build_large_bar_graph_skills_from_data(chart_data, chart_colors);

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
            <div style={format!("font-size: {}px; margin-bottom: 1em;", (width as f32).sqrt().round() * 0.5)}>{subtitle}</div>
            <div style="margin:2em;" id={chart_id.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TopPlayersProps {
    pub rows: Vec<(u64, Player)>,
}


#[function_component(TopPlayersTable)]
pub fn top_players_table(props: &TopPlayersProps) -> Html {
    html! {
        <table style="margin-left: auto; margin-right: auto; margin-top: 1em; font-size: 2em;">
            <thead style="color: #fff; text-align: center;">
                <tr>
                    // <th style="width: 25%">{"Rank"}</th>
                    <th style="width: 60%">{"Player"}</th>
                    <th style="width: 40%">{"#1 Rankings"}</th>
                </tr>
            </thead>
            <tbody style="color: #fff; text-align: center;">
                { for props.rows.iter().enumerate().map(|(i, (count, player))|
                    html! {
                        <tr>
                            // <td> { format!("{}.", i + 1) } </td>
                            <td> { hodor_name_to_html(if player.text.is_empty() {&player.name} else { &player.text }) } </td>
                            <td> { count } </td>
                        </tr>
                    }
                ) }
            </tbody>
        </table>
    }
}