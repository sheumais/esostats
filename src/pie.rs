use std::rc::Rc;
use charming::{Chart, WasmRenderer, component::Legend, element::{Color, ItemStyle, JsFunction, Label, LabelLine, TextStyle, Tooltip, Trigger}, series::Pie};
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

#[function_component(SkillPieChart)]
pub fn pie_chart(props: &PieChartProps) -> Html {
    let master_table = props.master_table.clone();
    let partitions = props.partitions.clone();
    let top_n = props.top_n;
    let chart_id = props.chart_id.clone();
    let chart_id_clone = chart_id.clone();
    let width = props.width.clone();
    let height = props.height.clone();

    let name = if partitions.len() == 1 {
        partition_to_name(*partitions.first().unwrap())
    } else if partitions.len() > 1 && partitions.len() < 27 {
        format!("Update {} ->  Update {}", partition_to_update_id(*partitions.first().unwrap()), partition_to_update_id(*partitions.last().unwrap()))
    } else {
        "All Recorded Patches".to_string()
    };

    let render_task = yew_hooks::use_async(async move {
        let renderer = WasmRenderer::new(width, height);
        let (chart_data, chart_colors) =
            crate::data::top_n_skills_chart_vectors(&master_table, &partitions, top_n);

        let chart = Chart::new()
            .tooltip(Tooltip::new()
                .trigger(Trigger::Item)
                .background_color("#282c38".to_string())
                .border_width(2)
                .formatter(JsFunction::new_with_args(
                    "params",
                    r#"
                        return `
                        <div style="color: #fff; font-family: "TF2Build";">${params.name}: ${params.percent}%</div>
                        `
                    "#,
                )))
            .legend(Legend::new()
                .left("60%")
                .width("25%")
                .top("center")
                .height("100%")
                .text_style(TextStyle::new()
                    .color(Color::Value("#FFFFFF".to_string()))
                    .font_family("TF2Build")
                    .font_size(15))
                    .item_gap(3))
            .color(chart_colors)
            .series(
                Pie::new()
                    .center(vec!["30%", "50%"])
                    .radius(vec!["25%", "90%"])
                    .avoid_label_overlap(true)
                    .label(Label::new().show(false).color(Color::Value("#FFFFFF".to_string())))
                    .label_line(LabelLine::new().show(false))
                    .item_style(ItemStyle::new().border_color("#282c38").border_width(5).border_radius(8))
                    .data(chart_data),
            );

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
        <div style="display: flex; flex-direction: column; align-items: center; color: #fff; margin: 1em;">
            <div style={format!("font-size: {}px;", (width as f32).sqrt().round())}>{name}</div>
            <div id={chart_id.clone()} />
        </div>
    }
}