use std::rc::Rc;

use yew::{Html, Properties, function_component, html};

use crate::pie::{LargeSetPieChart, SetPieChart};

#[derive(Properties, PartialEq, Clone)]
pub struct SetsPageProps {
pub master_table: Rc<crate::data::MasterTable>,
}


#[function_component(SetsPage)]
pub fn sets_page(props: &SetsPageProps) -> Html {
    let master_table = props.master_table.clone();


    let set_charts: Html = (1..28)
    .map(|i| {
    html! {
    <SetPieChart
    master_table={master_table.clone()}
    partitions={vec![i]}
    top_n={12}
    chart_id={format!("topsets_{}", i)}
    width={500}
    height={300}
    />
    }
    })
    .collect();


    html! {
        <div style="width: 100%; display: flex; flex-direction: column; gap: 2rem; align-items: center; color: #fff;">
            <LargeSetPieChart
                master_table={master_table.clone()}
                partitions={vec![]}
                top_n={50}
                chart_id={"topsets_all".to_string()}
                width={1500}
                height={900}
            />
            <div style="font-size: 2rem; margin: 1rem; font-weight: bold; user-select: none; text-align: center;">{"Top 12 Most Frequently Used Parse Sets By Patch"}</div>
            <div style="display: flex; flex-wrap: wrap; gap: 1rem; justify-content: center; width: 100%;">
                { set_charts }
            </div>
        </div>
    }
}