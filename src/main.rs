use yew::{Callback, Html, function_component, html, use_memo, use_state};
use yew_icons::{Icon, IconId};

use crate::{players::PlayersPage, sets::SetsPage, skills::SkillsPage};

mod data;
mod graph;
mod players;
mod skills;
mod sets;

enum Page {
    Players,
    Sets,
    Skills
}

#[function_component(App)]
pub fn app() -> Html {
    let master_table = use_memo((), |_| data::load_master_table());
    let page = use_state(|| Page::Players);

    let on_nav = {
        let page = page.clone();
        Callback::from(move |p| page.set(p))
    };

    let content = match *page {
        Page::Skills => html! { <SkillsPage master_table={master_table.clone()} /> },
        Page::Sets => html! { <SetsPage master_table={master_table.clone()} /> },
        Page::Players => html! { <PlayersPage master_table={master_table.clone()} /> },
    };

    let button_style = "background-color: #15171fff; font-family: 'TF2Build'; color: #fff; border: none; text-decoration: none; padding: 0.25em; outline: none; font-size: 2.5em; border-radius: 0.25em; cursor: pointer;".to_string();

    html! {
        <div style="display: flex; align-items: center; flex-direction: column; min-height: 100vh;">
            <div style="top: 0px;">
                <div style="display: flex; gap: 1rem; align-items: center; justify-content: center; width: 100vw; margin-top: 0.5em;">
                    <div style="font-size: 2.5rem; font-weight: 700; color: #fff;">{"ESO Raid Stats"}</div>
                    <div style="display: flex; gap: 0.5rem;">
                        <button onclick={Callback::from({let on_nav = on_nav.clone(); move |_| on_nav.emit(Page::Players)})} style={button_style.clone()}>{"Players"}</button>
                        <button onclick={Callback::from({let on_nav = on_nav.clone(); move |_| on_nav.emit(Page::Skills)})} style={button_style.clone()}>{"Skills"}</button>
                        <button onclick={Callback::from({let on_nav = on_nav.clone(); move |_| on_nav.emit(Page::Sets)})} style={button_style.clone()}>{"Sets"}</button>
                    </div>
                </div>
            </div>


            <div>
                { content }
            </div>
            <div style="position: fixed; bottom: 1em; right: 1em; display: flex; gap: 1em; z-index: 999;">
                <a href={"https://discord.gg/FjJjXHjUQ4"} target="_blank" rel="noopener noreferrer">
                    <Icon icon_id={IconId::BootstrapDiscord} style={r#"width: 3em; height: 3em; color: #fff; cursor: pointer;"#} />
                </a>
                <a href={"https://github.com/sheumais/esostats/"} target="_blank" rel="noopener noreferrer">
                    <Icon icon_id={IconId::BootstrapGithub} style={r#"width: 3em; height: 3em; color: #fff; cursor: pointer;"#} />
                </a>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    // data::process_data_into_master_table_serialized()
}
