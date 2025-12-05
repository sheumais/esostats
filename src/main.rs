use yew::{Html, function_component, html, use_memo};
use yew_icons::{Icon, IconId};

use crate::pie::SkillPieChart;

mod data;
mod pie;



#[function_component(App)]
fn app() -> Html {
    let master_table = use_memo((), |_| data::load_master_table());

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
        })
        .collect();

    html! {
        <div style=r#"display: flex; justify-content: center; align-items: center; flex-direction: row; flex-wrap: wrap; color: #fff;"#>
            <div style=r#"font-size: 4rem; margin: 1rem; font-weight: bold; user-select: none; width: 100%; text-align: center;"#>{"Syrup Stats - Get sticky with it"}</div>
            <SkillPieChart
                    master_table={master_table.clone()}
                    partitions={vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]}
                    top_n={50}
                    chart_id={format!("topskills_all")}
                    width={1500}
                    height={900}
            />
            {skill_charts}
            
            <div style="position: fixed; bottom: 1em; right: 1em; display: flex; gap: 1em; z-index: 999;">
                <a
                    href={"https://discord.gg/FjJjXHjUQ4"}
                    target="_blank"
                    rel="noopener noreferrer">
                    <Icon icon_id={IconId::BootstrapDiscord} style={r#"
                        width: 3em;
                        height: 3em;
                        color: #fff;
                        cursor: pointer;
                    "#} />
                </a>
                <a
                    href={"https://github.com/sheumais/"}
                    target="_blank"
                    rel="noopener noreferrer">
                    <Icon icon_id={IconId::BootstrapGithub} style={r#"
                        width: 3em;
                        height: 3em;
                        color: #fff;
                        cursor: pointer;
                    "#} />
                </a>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    // data::process_data_into_master_table_serialized()
}
