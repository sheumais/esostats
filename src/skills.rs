use std::rc::Rc;

use yew::{Html, Properties, function_component, html};

use crate::graph::{LargeSkillPieChart, LargeSkillsBarGraph, SkillPieChart};

#[derive(Properties, PartialEq, Clone)]
pub struct SkillsPageProps {
pub master_table: Rc<crate::data::MasterTable>,
}


#[function_component(SkillsPage)]
pub fn skills_page(props: &SkillsPageProps) -> Html {
let master_table = props.master_table.clone();


    let skill_charts: Html = (1..28)
    .map(|i| {
        html! {
            <SkillPieChart
                master_table={master_table.clone()}
                partitions={vec![i]}
                top_n={12}
                chart_id={format!("topskills_{}", i)}
                width={500}
                height={300}
            />
        }
    }).rev().collect();


    html! {
        <div style="width: 100%; display: flex; flex-direction: column; gap: 2rem; align-items: center; color: #fff;">
            // <LargeSkillPieChart
            //     master_table={master_table.clone()}
            //     partitions={vec![27]}
            //     top_n={30}
            //     chart_id={"topskills_latest".to_string()}
            //     width={1500}
            //     height={900}
            // />

            <LargeSkillsBarGraph
                master_table={master_table.clone()}
                partitions={vec![27]}
                top_n={36}
                chart_id={"topskills_bar".to_string()}
                width={1500}
                height={900}
            />

            <div style="font-size: 2rem; margin: 1rem; font-weight: bold; user-select: none; text-align: center;">{"Top 12 Most Frequently Used Parse Skills By Patch"}</div>
            <div style="display: flex; flex-wrap: wrap; gap: 1rem; justify-content: center; width: 100%;">
                { skill_charts }
            </div>
            <LargeSkillPieChart
                master_table={master_table.clone()}
                partitions={vec![]}
                top_n={75}
                chart_id={"topskills_all".to_string()}
                width={1500}
                height={900}
            />
        </div>
    }
}