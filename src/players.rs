use std::rc::Rc;

use yew::prelude::*;
use web_sys::{KeyboardEvent, InputEvent};


use crate::{data::{self, MasterTable, TableRow, boss_to_boss_name, partition_to_name, top_n_players_by_top_k_count}, graph::TopPlayersTable};

fn create_player_row(_master: &MasterTable, row: TableRow) -> Html {
    let dps = row.dps;
    let boss = boss_to_boss_name(row.boss_id);
    let ranking = row.ranking;
    let aoe_st = if row.boss {"ST"} else {"AOE"};    

    html!{
        <div style="display: flex; gap: 10px; flex-direction: row; width: max-content; font-size: 1.5em;">
            <span>{format!("{}.", ranking)}</span>
            <span>{format!("{:.1}k {}", dps as f32 / 1000.0, aoe_st)}</span>
            <span>{boss}</span>
        </div>
    }
}

pub fn hodor_name_to_html(input: &str) -> Html {
    let mut chars = input.chars().peekable();
    let mut current_color: Option<String> = None;

    let mut spans: Vec<Html> = Vec::new();

    while let Some(c) = chars.next() {
        if c == '|' {
            match chars.peek() {
                Some('c') => {
                    chars.next();
                    let mut hex = String::new();
                    for _ in 0..6 {
                        if let Some(h) = chars.next() {
                            hex.push(h);
                        }
                    }
                    current_color = Some(hex);
                }

                Some('r') => {
                    chars.next();
                    current_color = None;
                }

                Some('L') => {
                    chars.next();

                    loop {
                        match chars.next() {
                            Some('|') => {
                                if let Some('l') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                            Some(_) => continue,
                            None => break,
                        }
                    }
                }

                Some('t') => {
                    chars.next();

                    loop {
                        match chars.next() {
                            Some('|') => {
                                if let Some('t') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                            Some(_) => continue,
                            None => break,
                        }
                    }
                }

                Some(_) => {
                    chars.next();
                }
                None => break,
            }
        } else {
            let style = current_color
                .as_ref()
                .map(|hex| format!("color: #{}", hex))
                .unwrap_or_default();

            spans.push(html! {
                <span style={style.clone()}>{ c }</span>
            });
        }
    }

    html! { <>{ for spans }</> }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PlayersPageProps {
    pub master_table: Rc<crate::data::MasterTable>,
}

#[function_component(PlayersPage)]
pub fn players_page(props: &PlayersPageProps) -> Html {
    let master_table = props.master_table.clone();

    let selected_player = use_state(|| None::<u32>);
    let search_open = use_state(|| false);
    let search_query = use_state(|| String::new());

    let matches_ref = use_mut_ref(|| Vec::<(usize, String)>::new());

    let matches = {
        let query = (*search_query).to_lowercase().trim_matches('@').to_owned();
        if query.len() >= 3 {
            master_table
                .players
                .iter()
                .enumerate()
                .filter(|(_, player)| player.name.to_lowercase().contains(&query))
                .map(|(i, player)| (i, player.name.clone()))
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    };
    *matches_ref.borrow_mut() = matches.clone();

    let oninput_search = {
        let search_query = search_query.clone();
        let search_open = search_open.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                search_query.set(val.clone());
                search_open.set(val.len() >= 3);
            }
        })
    };

    let on_select = {
        let search_query = search_query.clone();
        let master_table = master_table.clone();
        let search_open = search_open.clone();
        let selected_player = selected_player.clone();

        Callback::from(move |idx: usize| {
            if let Some(player) = master_table.players.get(idx) {
                selected_player.set(Some(player.id.clone()));
            }
            search_open.set(false);
            search_query.set(String::new());
        })
    };


    let onkeydown_search = {
        let matches_ref = matches_ref.clone();
        let on_select = on_select.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let matches = matches_ref.borrow();
                if let Some((idx, _)) = matches.first() {
                    on_select.emit(*idx);
                }
            }
        })
    };


    let player_data: Html = if let Some(player_id) = *selected_player {
        let mut rows: Vec<&data::TableRow> = master_table
            .rows
            .iter()
            .filter(|p| p.player_id == player_id)
            .collect();


        rows.sort_by(|a, b| {
            b.partition_id
            .cmp(&a.partition_id)
            .then(a.ranking.cmp(&b.ranking))
            .then(b.dps.cmp(&a.dps))
        });


        let mut last_partition: Option<u8> = None;
        let mut children: Vec<Html> = Vec::new();


        let player_text = &master_table.players[player_id as usize - 1].text;
        let player_name_html = if player_text.is_empty() {
            html! { &master_table.players[player_id as usize - 1].name }
        } else {
            hodor_name_to_html(&player_text)
        };


        for r in rows {
            if r.ranking > 25 { continue; }
            if Some(r.partition_id) != last_partition {
                last_partition = Some(r.partition_id);
                let partition_label = partition_to_name(r.partition_id);


                children.push(
                    html! {
                        <span style="font-size: 2em; margin: 0.5em;">{ partition_label }</span>
                    }
                );
            }


            children.push(create_player_row(&master_table, r.clone()));
        }


        if children.is_empty() {
            children.push(html!{<div style="font-size: 1.5em;">{"No Results"}</div>})
        }


        html! {
            <div style="display: flex; flex-direction: column; align-items: center; gap: 5px; color: #fff;">
            <div style="font-size: 5em;">{ player_name_html }</div>
                { for children }
            </div>
        }
    } else {
        html! { "" }
    };

    html! {
        <div>
            <div style="width: max-content; color: #fff; width: max-content; color: #fff; display: flex; flex-direction: column; align-items: center; margin: auto;">
            <div style="font-size: 3rem; margin: 1rem; font-weight: bold; user-select: none; text-align: left; color: #fff;">{"Search DPS Players"}</div>
                <input
                    type="text"
                    placeholder="@username"
                    value={(*search_query).clone()}
                    oninput={oninput_search}
                    onkeydown={onkeydown_search}
                    autofocus=true
                    style="width: 70.5%; font-family: 'TF2Build'; background-color: #15171fff; color: #fff; border: none; text-decoration: none; padding: 0.25em; outline: none; font-size: 2.5em; border-radius: 0.25em;"
                />
                if *search_open {
                    <ul style="height: max-content; max-height: 16em; overflow-y: auto; margin-top: 0.25em; padding: 0.25em; list-style: none; display: flex; flex-direction: column; overflow-x: hidden; font-size: 1.5em; color: #fff; z-index: 999; position: relative; background-color: #15171fdd; width: 16.25em; border-radius: 0.5em;">
                        { 
                            for matches.iter().map(|(i, label)| {
                                let label_clone = label.clone();
                                let idx = *i;
                                let on_click = {
                                    let on_select = on_select.clone();
                                    Callback::from(move |_| on_select.emit(idx))
                                };
                                html! {
                                    <li onclick={on_click} style="padding: 0.25em 0; cursor: pointer; width: 100%;">
                                        { label_clone }
                                    </li>
                                }
                            }) 
                        }
                    </ul>
                }
            </div>
            if let Some(_) = *selected_player {
                <div style="flex: 1 1 30em;">
                    { player_data }
                </div>
            } else {
                <TopPlayersTable
                    rows={top_n_players_by_top_k_count(&master_table, &[], 101, 1)}
                />
            }
        </div>
    }
}