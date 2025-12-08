use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs::{self, File}, io::Write, path::Path};
use bitcode::*;
use charming::{element::Color, series::{SankeyLink, SankeyNode}};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Row {
    pub zone: u32,
    pub boss: u32,
    pub partition: u32,
    pub row_id: String,
    pub name: String,
    pub at_name: String,
    pub dps: f32,
    pub timestamp: Option<String>,
    pub talents: Vec<String>,
    pub gear: Vec<GearItem>,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct GearItem {
    pub name: String,
    pub icon: String,
    pub id: u32,
}

fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        fields.push(current.trim().to_string());
    }

    fields
}

fn parse_talents(s: &str) -> Vec<String> {
    let trimmed = s.trim().trim_start_matches('[').trim_end_matches(']');
    trimmed
        .split(',')
        .map(|x| x.trim().trim_matches('\'').to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

fn parse_gear(s: &str) -> Vec<GearItem> {
    let mut gear = Vec::new();
    let current = s.trim().trim_start_matches('[').trim_end_matches(']').to_string();

    let parts: Vec<&str> = current.split("},").collect();
    for part in parts {
        let part = part.trim().trim_start_matches('{').trim_end_matches('}');

        let mut name = String::new();
        let mut icon = String::new();
        let mut id = 0;

        for kv in part.split(',') {
            let kv = kv.trim();
            let mut iter = kv.splitn(2, ':');
            let key = iter.next().unwrap_or("").trim().trim_matches('\'').trim_matches('"');
            let value = iter.next().unwrap_or("").trim().trim_matches('\'').trim_matches('"');

            match key {
                "name" => name = value.to_string(),
                "icon" => icon = value.to_string(),
                "id" => id = value.parse::<u32>().unwrap_or(0),
                _ => {}
            }
        }

        if !name.is_empty() && !icon.is_empty() {
            gear.push(GearItem { name, icon, id });
        }
    }

    gear
}

pub fn parse_csv_line(line: &str) -> Result<Row, String> {
    let fields = split_csv_line(line);
    if fields.len() < 10 {
        return Err("Not enough fields".to_string());
    }

    let zone: u32 = fields[0].parse().map_err(|e| format!("zone parse error: {}", e))?;
    let boss: u32 = fields[1].parse().map_err(|e| format!("boss parse error: {}", e))?;
    let partition: u32 = fields[2].parse().map_err(|e| format!("partition parse error: {}", e))?;
    let row_id = fields[3].clone();
    let name = fields[4].clone();
    let at_name = fields[5].clone();
    let dps: f32 = fields[6].parse().map_err(|e| format!("dps parse error: {}", e))?;
    let timestamp = if fields[7].is_empty() { None } else { Some(fields[7].clone()) };

    let talents = parse_talents(&fields[8]);
    let gear = parse_gear(&fields[9]);

    Ok(Row {
        zone,
        boss,
        partition,
        row_id,
        name,
        at_name,
        dps,
        timestamp,
        talents,
        gear,
    })
}

pub fn parse_csv_text(text: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if i == 0 { continue; }
        if line.trim().is_empty() { continue; }
        let row = parse_csv_line(line)?;
        rows.push(row);
    }
    Ok(rows)
}

pub fn boss_to_boss_name(boss_id: u8) -> String {
    let name = match boss_id {
        4 => "The Mage",
        8 => "The Warrior",
        12 => "The Serpent",
        15 => "Rakkhat",
        20 => "Assembly General",
        23 => "Saint Olms the Just",
        27 => "Z'Maja",
        43 => "Lokkestiiz",
        44 => "Yolnahkriin",
        45 => "Nahviintaas",
        46 => "Yandir the Butcher",
        47 => "Captain Vrol",
        48 => "Lord Falgravn",
        49 => "Oaxiltso",
        50 => "Flame-Herald Bahsei",
        51 => "Xalvakka",
        52 => "Lylanar and Turlassil",
        53 => "Reef Guardian",
        54 => "Tideborn Taleria",
        55 => "Exarchanic Yaseyla",
        56 => "Archwizard Twelvane and Chimera",
        57 => "Ansuul the Tormentor",
        58 => "Count Ryelaz and Zilyesset",
        59 => "Orphic Shattered Shard",
        60 => "Xoryn",
        61 => "Hall of Fleshcraft",
        62 => "Jynorah and Skorkhif",
        63 => "Overfiend Kazpian",
        _ => "Unknown Boss"
    };
    name.to_string()
}

pub fn partition_to_name(partition_id: u8) -> String {
    let partition_name = match partition_id {
        1 => "Elsweyr (Update 22)",
        2 => "Scalebreaker (Update 23)",
        3 => "Dragonhold (Update 24)",
        4 => "Harrowstorm (Update 25)",
        5 => "Greymoor (Update 26)",
        6 => "Stonethorn (Update 27)",
        7 => "Markarth (Update 28)",
        8 => "Flames of Ambition (Update 29)",
        9 => "Blackwood (Update 30)",
        10 => "Waking Flame (Update 31)",
        11 => "Deadlands (Update 32)",
        12 => "Ascending Tide (Update 33)",
        13 => "High Isle (Update 34)",
        14 => "Lost Depths (Update 35)",
        15 => "Firesong (Update 36)",
        16 => "Scribes of Fate (Update 37)",
        17 => "Necrom (Update 38)",
        18 => "Free Update (Update 39)",
        19 => "Infinite Archive (Update 40)",
        20 => "Scions of Ithelia (Update 41)",
        21 => "Gold Road (Update 42)",
        22 => "Home Tours (Update 43)",
        23 => "Golden Pursuits (Update 44)",
        24 => "Fallen Banners (Update 45)",
        25 => "Western Solstice (Update 46)",
        26 => "Feast of Shadows (Update 47)",
        27 => "Eastern Solstice (Update 48)",
        _ => "Unknown Partition",
    };
    partition_name.to_string()
}

pub fn partition_to_update_id(partition_id: u8) -> String {
    let partition_name = match partition_id {
        1 => "22",
        2 => "23",
        3 => "24",
        4 => "25",
        5 => "26",
        6 => "27",
        7 => "28",
        8 => "29",
        9 => "30",
        10 => "31",
        11 => "32",
        12 => "33",
        13 => "34",
        14 => "35",
        15 => "36",
        16 => "37",
        17 => "38",
        18 => "39",
        19 => "40",
        20 => "41",
        21 => "42",
        22 => "43",
        23 => "44",
        24 => "45",
        25 => "46",
        26 => "47",
        27 => "48",
        _ => "Unknown Partition",
    };
    partition_name.to_string()
}

pub fn parse_set_data_into_hashmap() -> HashMap<u16, &'static str> {
    let mut lookup_table = HashMap::new();
    let data = include_str!("../data/set_data.csv");
    for line in data.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            if let Ok(key) = parts[0].parse::<u16>() {
                lookup_table.insert(key, parts[1]);
            }
        }
    }

    lookup_table
}

pub fn parse_set_ids_into_hashmap() -> HashMap<u32, u16> {
    let mut lookup_table: HashMap<u32, u16> = HashMap::new();
    let data = include_str!("../data/set_ids.csv");

    for line in data.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.is_empty() {
            continue;
        }

        let value_str = parts[0];
        let value_num = value_str.parse::<u16>()
            .expect("Invalid value in CSV");

        for key_str in &parts[1..] {
            let key_num = key_str.parse::<u32>()
                .expect("Invalid key in CSV");

            lookup_table.insert(key_num, value_num);
        }
    }

    lookup_table
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Encode, Decode)]
pub struct TableRow {
    pub boss_id: u8,
    pub partition_id: u8,
    pub ranking: u8,
    /// reference into master.players by id
    pub player_id: u32,
    pub dps: u32,
    pub boss: bool,
    /// references into master.skills by id
    pub skills: Vec<u16>,
    pub armour: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Encode, Decode)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Encode, Decode)]
pub struct Skill {
    pub id: u16,
    pub name: String,

    pub class: Option<String>,
    pub tree: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Encode, Decode)]
pub struct SkillMetadata {
    pub fileName: String,
    pub skillName: String,
    pub skillClass: String,
    pub skillTree: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Encode, Decode)]
pub struct ItemSet {
    pub id: u16,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Encode, Decode)]
pub struct MasterTable {
    pub rows: Vec<TableRow>,
    pub players: Vec<Player>,
    pub skills: Vec<Skill>,
    pub sets: Vec<ItemSet>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HodorPlayer {
    name: String,
    text: String,
}

fn read_players_from_folder<P: AsRef<Path>>(dir: P) -> std::io::Result<Vec<HodorPlayer>> {
    let mut players = Vec::new();

    let re = Regex::new(r#"n\["@([^"]+)"\]\s*=\s*\{"[^"]*",\s*"([^"]+)"}"#).unwrap();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let content = fs::read_to_string(&path)?;

        for cap in re.captures_iter(&content) {
            let raw_name = cap[1].to_string();
            let text = cap[2].to_string();

            players.push(HodorPlayer {
                name: raw_name,
                text,
            });
        }
    }

    Ok(players)
}

pub fn process_data_into_master_table_serialized() {
    let set_id_hashmap = parse_set_ids_into_hashmap();

    // read input CSVs
    let csv_text = std::fs::read_to_string("data/total_dps.csv").unwrap();
    let csv_text_2 = std::fs::read_to_string("data/boss_dps.csv").unwrap();

    let rows_total_dps = parse_csv_text(&csv_text).unwrap();
    let rows_boss_dps = parse_csv_text(&csv_text_2).unwrap();

    let set_csv_text = std::fs::read_to_string("data/set_data.csv")
        .expect("failed to read ../data/set_data.csv");
    let mut sets_vec: Vec<ItemSet> = Vec::new();
    for (line_no, line) in set_csv_text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, ',');
        let id_str = parts.next().map(str::trim).unwrap_or("");
        let name = parts
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                panic!("missing set name on line {} in ../data/set_data.csv", line_no + 1)
            });

        let id: u16 = id_str.parse().unwrap_or_else(|_| {
            panic!(
                "invalid set id '{}' on line {} in ../data/set_data.csv",
                id_str,
                line_no + 1
            )
        });

        sets_vec.push(ItemSet { id, name });
    }

    let mut skill_id_map: HashMap<String, u16> = HashMap::new();
    let mut skills_vec: Vec<Skill> = Vec::new();

    let meta_text = std::fs::read_to_string("data/skills.json").unwrap();
    let raw_meta: HashMap<String, SkillMetadata> = serde_json::from_str(&meta_text).unwrap();

    let mut meta_lookup: HashMap<String, SkillMetadata> = HashMap::new();

    for (_key, meta) in raw_meta {
        let f = meta
            .fileName
            .strip_suffix(".png")
            .unwrap()
            .strip_prefix("ability_")
            .unwrap()
            .to_string();
        meta_lookup.insert(f, meta);
    }

    let mut get_skill_id = |name: String| -> u16 {
        if let Some(&id) = skill_id_map.get(&name) {
            return id;
        }

        let new_id = skills_vec.len() as u16 + 1;

        let meta = meta_lookup.get(&name);

        skills_vec.push(Skill {
            id: new_id,
            name: name.clone(),
            class: meta.map(|m| m.skillClass.clone()),
            tree: meta.map(|m| m.skillTree.clone()),
            display_name: meta.map(|m| m.skillName.clone()),
        });

        skill_id_map.insert(name, new_id);
        new_id
    };

    let mut player_id_map: HashMap<String, u32> = HashMap::new();
    let mut players_vec: Vec<Player> = Vec::new();

    let mut get_player_id = |name: String| -> u32 {
        if let Some(&id) = player_id_map.get(&name) {
            return id;
        }
        let new_id = players_vec.len() as u32 + 1;
        player_id_map.insert(name.clone(), new_id);

        players_vec.push(Player {
            id: new_id,
            name,
            text: "".into(),
        });

        new_id
    };

    let mut out_rows: Vec<TableRow> = Vec::with_capacity(rows_total_dps.len() + rows_boss_dps.len());

    let mut process_entry = |entry: &Row, boss: bool| {
        let (_, ranking_str) = entry.row_id.rsplit_once('-')
            .unwrap_or_else(|| panic!("invalid row_id format: {}", entry.row_id));
        let ranking: u8 = ranking_str.parse().unwrap_or_else(|_| {
            panic!("failed to parse ranking '{}' from row_id '{}'", ranking_str, entry.row_id)
        });

        let boss_id: u8 = entry.boss as u8;
        let partition_id: u8 = entry.partition as u8;

        let skill_ids: Vec<u16> = entry.talents.iter().map(|e| {
            let e = e.strip_suffix(".png").unwrap_or(e);
            let e = e.strip_prefix("ability_").unwrap_or(e);
            let skill_name = e.to_string();
            get_skill_id(skill_name)
        }).collect();

        let gear: Vec<u16> = entry.gear.iter()
            .map(|g| *set_id_hashmap.get(&g.id).unwrap_or(&0))
            .collect();

        let dps: u32 = entry.dps as u32;

        let canonical_name = entry.at_name.strip_prefix("@").unwrap_or(&entry.at_name).to_string();
        let player_id: u32 = get_player_id(canonical_name);

        let row = TableRow {
            boss_id,
            partition_id,
            ranking,
            player_id,
            dps,
            boss,
            skills: skill_ids,
            armour: gear,
        };

        out_rows.push(row);
    };

    for entry in &rows_total_dps {
        process_entry(entry, false);
    }
    for entry in &rows_boss_dps {
        process_entry(entry, true);
    }

    let hodor_dir = r"..\Elder Scrolls Online\live\AddOns\LibCustomNames\PC\names";
    let hodor_players = read_players_from_folder(hodor_dir)
        .expect("Failed reading Hodor player files");

    for hp in hodor_players {
        let name = hp.name.clone();

        if let Some(&pid) = player_id_map.get(&name) {
            if let Some(existing) = players_vec.iter_mut().find(|p| p.id == pid) {
                existing.text = hp.text.clone();
            }
        }
    }

    let master = MasterTable {
        rows: out_rows,
        players: players_vec,
        skills: skills_vec,
        sets: sets_vec,
    };

    let mut out_file = File::create("data/master_table.bin").expect("failed to create master_table.bin");
    // bincode::serde::encode_into_std_write(&master, &mut out_file, bincode::config::standard())
        // .expect("failed to serialize master table with bincode");
    let encoded = bitcode::encode(&master);
    out_file.write(&encoded).unwrap();

    let mut json_file = File::create("data/master_table.json").expect("failed to create master_table.json");
    let json_text = serde_json::to_string_pretty(&master).expect("failed to serialize to json");
    json_file.write_all(json_text.as_bytes()).expect("failed to write master_table.json");

    println!(
        "Wrote master_table.bin (binary) and master_table.json (readable). Rows: {}, Players: {}, Skills: {}, Sets: {}",
        master.rows.len(), master.players.len(), master.skills.len(), master.sets.len()
    );
}

const MASTER_TABLE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/master_table.bin"));

pub fn load_master_table() -> MasterTable {
    {
        bitcode::decode::<MasterTable>(MASTER_TABLE_BYTES)
    }
    .expect("Failed to decode master table")
}

pub fn top_n_skills_for_partitions(master: &MasterTable, partition_filter: &[u8], n: usize) -> Vec<(Skill, u32)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut freq: HashMap<u16, u32> = HashMap::new();
    let mut total_other_count = 0;

    for row in &master.rows {
        let include = match &filter {
            Some(set) => set.contains(&row.partition_id),
            None => true,
        };

        if include {
            for skill_id in &row.skills {
                *freq.entry(*skill_id).or_insert(0) += 1;
            }
        }
    }

    let mut skill_counts: Vec<(u16, u32)> = freq.into_iter().collect();
    skill_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut result = Vec::new();

    for (i, (id, count)) in skill_counts.iter().enumerate() {
        if i < n {
            if let Some(skill) = master.skills.iter().find(|s| s.id == *id) {
                result.push((skill.clone(), *count));
            }
        } else {
            total_other_count += count;
        }
    }
    
    if total_other_count > 0 {
        result.push((Skill {
            id: 999,
            name: "Other".to_string(),
            class: None,
            tree: None,
            display_name: Some("Other".to_string()),
        }, total_other_count));
    }

    result
}

pub fn top_n_skills_for_partitions_normalised(master: &MasterTable, partition_filter: &[u8], n: usize) -> Vec<(Skill, u32)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut rows_per_partition: HashMap<u8, usize> = HashMap::new();
    let mut per_partition_skill_counts: HashMap<u8, HashMap<u16, u32>> = HashMap::new();

    for row in &master.rows {
        let include = match &filter {
            Some(set) => set.contains(&row.partition_id),
            None => true,
        };

        if !include {
            continue;
        }

        *rows_per_partition.entry(row.partition_id).or_insert(0) += 1;

        let skill_map = per_partition_skill_counts
            .entry(row.partition_id)
            .or_insert_with(HashMap::new);

        for skill_id in &row.skills {
            *skill_map.entry(*skill_id).or_insert(0) += 1;
        }
    }

    if rows_per_partition.is_empty() {
        return Vec::new();
    }

    let min_partition_size = rows_per_partition
        .values()
        .cloned()
        .min()
        .unwrap_or(0);

    if min_partition_size == 0 {
        return Vec::new();
    }

    let mut normalized_counts: HashMap<u16, f64> = HashMap::new();

    for (&partition_id, skill_map) in &per_partition_skill_counts {
        let partition_size = *rows_per_partition.get(&partition_id).unwrap_or(&0) as f64;
        if partition_size <= 0.0 {
            continue;
        }
        let weight = (min_partition_size as f64) / partition_size;

        for (&skill_id, &count) in skill_map {
            let entry = normalized_counts.entry(skill_id).or_insert(0.0);
            *entry += (count as f64) * weight;
        }
    }

    let mut skill_counts: Vec<(u16, u32)> = normalized_counts
        .into_iter()
        .map(|(id, v)| {
            let rv = v.round();
            let rv_u32 = if rv < 0.0 { 0 } else { rv as u32 };
            (id, rv_u32)
        })
        .collect();

    skill_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut result: Vec<(Skill, u32)> = Vec::new();
    let mut total_other_count: u32 = 0;

    for (i, (id, count)) in skill_counts.iter().enumerate() {
        if i < n {
            if let Some(skill) = master.skills.iter().find(|s| s.id == *id) {
                result.push((skill.clone(), *count));
            } else {
                result.push((
                    Skill {
                        id: *id,
                        name: format!("Skill #{}", id),
                        class: None,
                        tree: None,
                        display_name: None,
                    },
                    *count,
                ));
            }
        } else {
            total_other_count = total_other_count.saturating_add(*count);
        }
    }

    if total_other_count > 0 {
        result.push((
            Skill {
                id: 999,
                name: "Other".to_string(),
                class: None,
                tree: None,
                display_name: Some("Other".to_string()),
            },
            total_other_count,
        ));
    }

    result
}

pub fn colour_from_skill(skill: &Skill) -> Color {
    let hex = match skill.class.as_deref() {
        Some("Arcanist")      => "#9ACD32",
        Some("Dragonknight")  => "#FF8C00",
        Some("Nightblade")    => "#AA0000",
        Some("Templar")       => "#FFD700",
        Some("Sorcerer")      => "#1E90FF",
        Some("Warden")        => "#228B22",
        Some("Necromancer")   => "#8A2BE2",
        Some("Weapon")        => "#FFE4C4",

        Some(_) => match skill.tree.as_deref() {
            Some("Vampire")        => "#8B0000",
            Some("Fighters Guild") => "#B73700",
            Some("Mages Guild")    => "#4682B4",
            Some("Psijic Order")   => "#008B8B",
            Some("Undaunted")      => "#70B04A",
            Some("Assault")        => "#FA8072",
            Some("Support")        => "#87CEFA",
            Some("Soul Magic")     => "#800080",
            _ => "#B2B2B2",
        },

        None => "#B2B2B2",
    };

    Color::Value(hex.to_string())
}

pub fn top_n_skills_chart_vectors(master: &MasterTable, partition_filter: &[u8], n: usize) -> (Vec<(i32, String)>, Vec<Color>) {
    let top_skills = top_n_skills_for_partitions(master, partition_filter, n);

    let mut data: Vec<(i32, String)> = Vec::with_capacity(top_skills.len());
    let mut colours: Vec<Color> = Vec::with_capacity(top_skills.len());

    for (skill, count) in top_skills {
        let name = skill.display_name.clone().unwrap_or_else(|| skill.name.clone());
        data.push(((count as i32).try_into().unwrap(), name));
        colours.push(colour_from_skill(&skill));
    }

    (data, colours)
}

pub fn top_n_skills_normalised_chart_vectors(master: &MasterTable, partition_filter: &[u8], n: usize) -> (Vec<(i32, String)>, Vec<Color>) {
    let top_skills = top_n_skills_for_partitions_normalised(master, partition_filter, n);

    let mut data: Vec<(i32, String)> = Vec::with_capacity(top_skills.len());
    let mut colours: Vec<Color> = Vec::with_capacity(top_skills.len());

    for (skill, count) in top_skills {
        let name = skill.display_name.clone().unwrap_or_else(|| skill.name.clone());
        data.push(((count as i32).try_into().unwrap(), name));
        colours.push(colour_from_skill(&skill));
    }

    (data, colours)
}

pub fn top_n_sets_for_partitions(master: &MasterTable, partition_filter: &[u8], n: usize) -> Vec<(ItemSet, u32)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut name_to_base_id: HashMap<String, u16> = HashMap::new();
    let mut canonical_id: HashMap<u16, u16> = HashMap::new();

    for s in &master.sets {
        let raw_name = &s.name;
        let normalized_name = raw_name.strip_prefix("Perfected ").unwrap_or(raw_name);

        match name_to_base_id.get(normalized_name) {
            Some(&existing_id) => {
                if !raw_name.starts_with("Perfected ") {
                    canonical_id.insert(existing_id, s.id);
                    name_to_base_id.insert(normalized_name.to_string(), s.id);
                }
                canonical_id.insert(s.id, name_to_base_id[normalized_name]);
            }
            None => {
                name_to_base_id.insert(normalized_name.to_string(), s.id);
                canonical_id.insert(s.id, s.id);
            }
        }
    }

    let mut freq: HashMap<u16, u32> = HashMap::new();

    for row in &master.rows {
        let include = match &filter {
            Some(set) => set.contains(&row.partition_id),
            None => true,
        };

        if !include {
            continue;
        }

        let unique_sets: HashSet<u16> = row.armour.iter().copied().collect();

        for original_id in unique_sets {
            if original_id == 0 {
                continue;
            }

            if let Some(&base_id) = canonical_id.get(&original_id) {
                *freq.entry(base_id).or_insert(0) += 1;
            }
        }
    }

    let mut set_counts: Vec<(u16, u32)> = freq.into_iter().collect();
    set_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let set_lookup: HashMap<u16, ItemSet> = master
        .sets
        .iter()
        .cloned()
        .map(|s| (s.id, s))
        .collect();

    let mut result: Vec<(ItemSet, u32)> = Vec::new();
    let mut total_other_count: u32 = 0;

    for (i, (id, count)) in set_counts.iter().enumerate() {
        if i < n {
            let set = set_lookup
                .get(id)
                .cloned()
                .unwrap_or_else(|| ItemSet {
                    id: *id,
                    name: format!("Unknown ({})", id),
                });
            result.push((set, *count));
        } else {
            total_other_count += count;
        }
    }

    if total_other_count > 0 {
        result.push((
            ItemSet {
                id: 999,
                name: "Other".to_string(),
            },
            total_other_count,
        ));
    }

    result
}

pub fn top_n_sets_for_partitions_normalised(master: &MasterTable, partition_filter: &[u8], n: usize) -> Vec<(ItemSet, u32)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut name_to_base_id: HashMap<String, u16> = HashMap::new();
    let mut canonical_id: HashMap<u16, u16> = HashMap::new();

    for s in &master.sets {
        let raw_name = &s.name;
        let normalized_name = raw_name.strip_prefix("Perfected ").unwrap_or(raw_name);

        match name_to_base_id.get(normalized_name) {
            Some(&existing_id) => {
                if !raw_name.starts_with("Perfected ") {
                    canonical_id.insert(existing_id, s.id);
                    name_to_base_id.insert(normalized_name.to_string(), s.id);
                }
                canonical_id.insert(s.id, name_to_base_id[normalized_name]);
            }
            None => {
                name_to_base_id.insert(normalized_name.to_string(), s.id);
                canonical_id.insert(s.id, s.id);
            }
        }
    }

    let mut rows_per_partition: HashMap<u8, usize> = HashMap::new();
    let mut per_partition_set_counts: HashMap<u8, HashMap<u16, u32>> = HashMap::new();

    for row in &master.rows {
        let include = match &filter {
            Some(set) => set.contains(&row.partition_id),
            None => true,
        };

        if !include {
            continue;
        }

        *rows_per_partition.entry(row.partition_id).or_insert(0) += 1;

        let set_map = per_partition_set_counts
            .entry(row.partition_id)
            .or_insert_with(HashMap::new);

        let unique_sets: HashSet<u16> = row.armour.iter().copied().collect();

        for s in unique_sets {
            if s == 0 {
                continue;
            }
            if let Some(&cid) = canonical_id.get(&s) {
                *set_map.entry(cid).or_insert(0) += 1;
            }
        }
    }

    if rows_per_partition.is_empty() {
        return Vec::new();
    }

    let min_partition_size = rows_per_partition.values().cloned().min().unwrap();

    let mut normalized_counts: HashMap<u16, f64> = HashMap::new();

    for (&partition_id, set_map) in &per_partition_set_counts {
        let size = rows_per_partition[&partition_id] as f64;
        if size == 0.0 {
            continue;
        }

        let weight = (min_partition_size as f64) / size;

        for (&set_id, &count) in set_map {
            *normalized_counts.entry(set_id).or_insert(0.0) += (count as f64) * weight;
        }
    }

    let mut set_counts: Vec<(u16, u32)> = normalized_counts
        .into_iter()
        .map(|(id, v)| (id, v.round().max(0.0) as u32))
        .collect();

    set_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let set_lookup: HashMap<u16, ItemSet> =
        master.sets.iter().cloned().map(|s| (s.id, s)).collect();

    let mut result: Vec<(ItemSet, u32)> = Vec::new();
    let mut total_other = 0;

    for (i, (id, count)) in set_counts.iter().enumerate() {
        if i < n {
            let set = set_lookup
                .get(id)
                .cloned()
                .unwrap_or_else(|| ItemSet {
                    id: *id,
                    name: format!("Unknown ({})", id),
                });
            result.push((set, *count));
        } else {
            total_other += count;
        }
    }

    if total_other > 0 {
        result.push((
            ItemSet {
                id: 999,
                name: "Other".to_string(),
            },
            total_other,
        ));
    }

    result
}

pub fn colour_from_set(set: &ItemSet) -> Color {
    let hex = match set.id {
        83 => "#CF6A32", // Elf Bane
        127 => "#476291", // Deadly Strike
        137 => "#D32CE6", // Berserking Warrior (Advancing Yokeda)
        // 147 => "#38F3AB", // Way of Martial Knowledge
        // 205 => "#4B69FF", // Willpower
        // 232 => "#38F3AB", // Roar of Alkosh
        292 => "#8650AC", // Mother's Sorrow
        304 => "#70B04A", // Medusa
        // 332 => "#AA0000", // Master Architect
        336 => "#A32C2E", // Pillar of Nirn
        338 => "#CF6A32", // Flame Blossom
        353 => "#4B69FF", // Mechanical Acuity
        389 | 393 => "#FFD700", // Arms of Relequen
        390 | 394 => "#F4A460", // Mantle of Siroria
        430 => "#DAA520", // Tzogvin's Warband
        444 | 449 => "#00BFFF", // False God's Devotion
        445 | 450 => "#B22222", // Tooth of Lokkestiiz
        455 => "#6B8E23", // Z'en's Redress
        456 => "#007FFF", // Azureblight Reaper
        470 => "#476291", // New Moon Acolyte
        475 => "#AA0000", // Aegis Caller
        570 => "#38F3AB", // Kinras's Wrath
        584 => "#48D1CC", // Diamond's Victory
        586 | 589 => "#70B04A", // Sul-Xan's Torment
        587 | 591 => "#50A7FC", // Bahsei's Mania
        646 | 653 => "#4B69FF", // Whorl of the Depths
        647 | 652 => "#96DA43", // Coral Riptide
        684 => "#FF4500", // Runecarver's Blaze
        702 | 707 => "#2F4F4F", // Ansuul's Torment
        764 => "#F4A460", // Highland Sentinel
        767 | 772 => "#E4AE33", // Slivers of the Null Arca
        777 => "#8847FF", // Corpseburster
        809 => "#8FBC8F", // Tide-Born Wildstalker
        168 | // Nerien'eth
        169 | // Valkyn Skoria
        170 | // Maw of the Infernal
        257 | // Velidreth
        273 | // Ilambris
        274 | // Iceheart
        275 | // Stormfist
        279 | // Selene
        280 | // Grothdarr
        342 | // Domihaus
        350 | // Zaan
        458 | // Grundwulf
        459 // Maarselok
        => "#B0C4DE", // Monster Sets
        270 => "#4D7942", // Slimecraw
        373 | 526 => "#99CCFF", // Crushing Wall
        369 | 522 => "#FFC0CB", // Merciless Charge
        372 | 525 // Thunderous Volley
        | 367 | 361 // Concentrated Force
        | 316 | 531 // Caustic Arrow
        | 413 | 425 // Spectral Cloak
        | 371 | 524 // Cruel Flurry
        => "#FFE4C4", // Arena Weapons
        501 | 503 | 505 | 519 | 520 | 521 | 575 | 576 | 593 | 594 | 596 | 597 | 625 | 626 | 627 | 654 | 655 | 656 | 657 | 658 | 674 | 675 | 676 | 691 | 692 | 693 | 694 | 760 | 761 | 762 | 811 | 812 | 813 | 845 => "#FF8200", // Mythics
        999 => "#B2B2B2",
        _ => "#B2B2B2",
    };

    Color::Value(hex.to_string())
}

pub fn top_n_sets_chart_vectors(master: &MasterTable, partition_filter: &[u8], n: usize) -> (Vec<(i32, String)>, Vec<Color>) {
    let top_sets = top_n_sets_for_partitions(master, partition_filter, n);

    let mut data: Vec<(i32, String)> = Vec::with_capacity(top_sets.len());
    let mut colours: Vec<Color> = Vec::with_capacity(top_sets.len());

    for (set, count) in top_sets {
        let name = set.name.clone();
        data.push(((count as i32).try_into().unwrap(), name));
        colours.push(colour_from_set(&set));
    }

    (data, colours)
}

pub fn top_n_sets_normalised_chart_vectors(master: &MasterTable, partition_filter: &[u8], n: usize) -> (Vec<(i32, String)>, Vec<Color>) {
    let top_sets = top_n_sets_for_partitions_normalised(master, partition_filter, n);

    let mut data: Vec<(i32, String)> = Vec::with_capacity(top_sets.len());
    let mut colours: Vec<Color> = Vec::with_capacity(top_sets.len());

    for (set, count) in top_sets {
        let name = set.name.clone();
        data.push(((count as i32).try_into().unwrap(), name));
        colours.push(colour_from_set(&set));
    }

    (data, colours)
}

pub fn top_n_players_by_average_rank(master: &MasterTable, partition_filter: &[u8], n: usize) -> Vec<(f64, String)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut agg: HashMap<u32, (u64, u64)> = HashMap::new();
    for row in &master.rows {
        if let Some(ref f) = filter {
            if !f.contains(&row.partition_id) {
                continue;
            }
        }
        let entry = agg.entry(row.player_id).or_insert((0u64, 0u64));
        entry.0 += row.ranking as u64;
        entry.1 += 1;
    }

    let id_to_name: HashMap<u32, String> = master
        .players
        .iter()
        .filter(|p| !p.name.is_empty())
        .map(|p| (p.id, p.name.clone()))
        .collect();

    let mut averages: Vec<(f64, u32)> = agg
        .into_iter()
        .filter(|(player_id, (_, count))| {
            *count >= 20 && id_to_name.contains_key(player_id)
        })
        .map(|(player_id, (sum, count))| ((sum as f64) / (count as f64), player_id))
        .collect();

    averages.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.1.cmp(&b.1))
    });

    averages
        .into_iter()
        .take(n)
        .map(|(avg, player_id)| {
            let name = id_to_name[&player_id].clone();
            (avg, name)
        })
        .collect()
}

pub fn top_n_players_by_top_k_count(master: &MasterTable, partition_filter: &[u8], n: usize, k: u32) -> Vec<(u64, Player)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut total_rows: HashMap<u32, u64> = HashMap::new();
    for row in &master.rows {
        if let Some(ref f) = filter {
            if !f.contains(&row.partition_id) {
                continue;
            }
        }
        *total_rows.entry(row.player_id).or_insert(0) += 1;
    }

    let mut counts_k: HashMap<u32, u64> = HashMap::new();
    for row in &master.rows {
        if let Some(ref f) = filter {
            if !f.contains(&row.partition_id) {
                continue;
            }
        }
        if (row.ranking as u32) <= k {
            *counts_k.entry(row.player_id).or_insert(0) += 1;
        }
    }

    let mut players: Vec<(u32, u64)> = counts_k
        .iter()
        .map(|(pid, c)| (*pid, *c))
        .filter(|(_, c)| *c > 0)
        .collect();

    players.sort_by(|a, b| b.1.cmp(&a.1));

    let cutoff_count = if players.len() >= n {
        players[n - 1].1
    } else {
        0
    };

    let tied_ids: Vec<u32> = players
        .iter()
        .filter(|(_, c)| *c == cutoff_count)
        .map(|(pid, _)| *pid)
        .collect();

    let mut counts_km1: HashMap<u32, u64> = HashMap::new();

    if k > 1 {
        let new_k = k - 1;

        for row in &master.rows {
            if tied_ids.contains(&row.player_id) {
                if let Some(ref f) = filter {
                    if !f.contains(&row.partition_id) {
                        continue;
                    }
                }
                if (row.ranking as u32) <= new_k {
                    *counts_km1.entry(row.player_id).or_insert(0) += 1;
                }
            }
        }
    }

    players.sort_by(|(pid_a, c_a), (pid_b, c_b)| {
        let ord = c_b.cmp(c_a);
        if ord != Ordering::Equal {
            return ord;
        }

        let tka = counts_km1.get(pid_a).copied().unwrap_or(0);
        let tkb = counts_km1.get(pid_b).copied().unwrap_or(0);
        let ord2 = tkb.cmp(&tka);
        if ord2 != Ordering::Equal {
            return ord2;
        }

        let ra = total_rows.get(pid_a).copied().unwrap_or(0);
        let rb = total_rows.get(pid_b).copied().unwrap_or(0);
        let ord3 = rb.cmp(&ra);
        if ord3 != Ordering::Equal {
            return ord3;
        }

        pid_a.cmp(pid_b)
    });

    players
        .into_iter()
        .take(n)
        .map(|(pid, count)| {
            let p = master.players[pid as usize - 1].clone();
            (count, p)
        })
        .filter(|(_, p)| !p.name.is_empty())
        .collect()
}

pub fn player_rows(master: &MasterTable, player_id: u32) -> Vec<TableRow> {
    let mut player_rows = Vec::new();
    for row in &master.rows {
        if row.player_id == player_id {
            player_rows.push(row.clone());
        }
    }
    return player_rows;
}

pub fn percent_players_with_sets(master: &MasterTable, partition_filter: &[u8]) -> Vec<(ItemSet, f64)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut name_to_ids: HashMap<String, Vec<(u16, bool)>> = HashMap::new();
    for s in &master.sets {
        let raw_name = &s.name;
        let normalized = raw_name.strip_prefix("Perfected ").unwrap_or(raw_name).to_string();
        let is_perfected = raw_name.starts_with("Perfected ");
        name_to_ids.entry(normalized).or_default().push((s.id, is_perfected));
    }

    let mut canonical_id: HashMap<u16, u16> = HashMap::new();
    for (_normalized_name, ids_vec) in name_to_ids.into_iter() {
        let base_id = ids_vec
            .iter()
            .find(|(_, is_perf)| !*is_perf)
            .map(|(id, _)| *id)
            .or_else(|| ids_vec.first().map(|(id, _)| *id))
            .expect("there should be at least one id for a name");
        for (id, _) in ids_vec {
            canonical_id.insert(id, base_id);
        }
    }

    let mut freq: HashMap<u16, u32> = HashMap::new();
    let mut total_rows_included: u32 = 0;
    let mut players_with_any_set: u32 = 0;

    for row in &master.rows {
        let include = match &filter {
            Some(set) => set.contains(&row.partition_id),
            None => true,
        };

        if !include {
            continue;
        }

        total_rows_included = total_rows_included.saturating_add(1);

        // unique non-zero armour ids for this player/row
        let unique_sets: HashSet<u16> = row.armour.iter().copied().filter(|&id| id != 0).collect();

        if unique_sets.is_empty() {
            continue;
        }

        players_with_any_set = players_with_any_set.saturating_add(1);

        for original_id in unique_sets {
            let base_id = canonical_id.get(&original_id).copied().unwrap_or(original_id);
            *freq.entry(base_id).or_insert(0) += 1;
        }
    }

    if players_with_any_set == 0 {
        return Vec::new();
    }

    let set_lookup: HashMap<u16, ItemSet> = master
        .sets
        .iter()
        .cloned()
        .map(|s| (s.id, s))
        .collect();

    let mut results: Vec<(ItemSet, f64)> = freq
        .into_iter()
        .map(|(id, count)| {
            let pct = (count as f64) * 100.0 / (players_with_any_set as f64);
            let set = set_lookup.get(&id).cloned().unwrap_or(ItemSet {
                id,
                name: format!("Unknown ({})", id),
            });
            (set, pct)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    results
}

pub fn top_n_sets_percentage_chart_vectors(
    master: &MasterTable,
    partition_filter: &[u8],
    n: usize,
) -> (Vec<(f64, String)>, Vec<Color>) {
    let top_sets = percent_players_with_sets(master, partition_filter);

    let mut data: Vec<(f64, String)> = Vec::new();
    let mut colours: Vec<Color> = Vec::new();

    for (set, pct) in top_sets.into_iter().take(n) {
        data.push((pct, set.name.clone()));
        colours.push(colour_from_set(&set));
    }

    (data, colours)
}

pub fn percent_players_with_skills(master: &MasterTable, partition_filter: &[u8]) -> Vec<(Skill, f64)> {
    let filter: Option<HashSet<u8>> = if partition_filter.is_empty() {
        None
    } else {
        Some(partition_filter.iter().copied().collect())
    };

    let mut freq: HashMap<u16, u32> = HashMap::new();
    let mut total_rows_included: u32 = 0;
    let mut players_with_any_skill: u32 = 0;

    for row in &master.rows {
        let include = match &filter {
            Some(set) => set.contains(&row.partition_id),
            None => true,
        };

        if !include {
            continue;
        }

        total_rows_included = total_rows_included.saturating_add(1);

        let unique_skills: HashSet<u16> = row.skills.iter().copied().collect();

        if unique_skills.is_empty() {
            continue;
        }

        players_with_any_skill = players_with_any_skill.saturating_add(1);

        for skill_id in unique_skills {
            *freq.entry(skill_id).or_insert(0) += 1;
        }
    }

    if players_with_any_skill == 0 {
        return Vec::new();
    }

    let skill_lookup: HashMap<u16, Skill> = master
        .skills
        .iter()
        .cloned()
        .map(|s| (s.id, s))
        .collect();

    let mut results: Vec<(Skill, f64)> = freq
        .into_iter()
        .map(|(id, count)| {
            let pct = (count as f64) * 100.0 / (players_with_any_skill as f64);
            let skill = skill_lookup.get(&id).cloned().unwrap_or(Skill {
                id,
                name: format!("Unknown ({})", id),
                class: None,
                tree: None,
                display_name: None,
            });
            (skill, pct)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    results
}

pub fn top_n_skills_percentage_chart_vectors(master: &MasterTable, partition_filter: &[u8], n: usize) -> (Vec<(f64, String)>, Vec<Color>) {
    let top_sets = percent_players_with_skills(master, partition_filter);

    let mut data: Vec<(f64, String)> = Vec::new();
    let mut colours: Vec<Color> = Vec::new();

    for (skill, pct) in top_sets.into_iter().take(n) {
        data.push((pct, skill.display_name.clone().unwrap_or_else(|| skill.name.clone())));
        colours.push(colour_from_skill(&skill));
    }

    (data, colours)
}
