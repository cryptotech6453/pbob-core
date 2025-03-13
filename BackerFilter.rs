use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BackerFilter {
    pub min_abi: u32,  // Минимальный ABI
    pub min_games: u32,  // Минимальное количество турниров
    pub min_games_per_month: u32,  // Минимум турниров в месяц
    pub min_roi: f64,  // Минимальный ROI
    pub min_dollars_per_tournament: f64, // Минимальный выигрыш с турнира
    pub afs_range: (u32, u32),  // Диапазон игроков в турнирах
}

pub fn execute_set_backing_filter(
    deps: DepsMut,
    info: MessageInfo,
    min_abi: f64,
    min_games: u32,
    min_games_per_month: u32,
    min_roi: f64,
    min_dollars_per_tournament: f64,
    min_afs: f64,
) -> Result<Response, ContractError> {
    let filter = BackerFilter {
        backer: info.sender.clone(),
        min_abi,
        min_games,
        min_games_per_month,
        min_roi,
        min_dollars_per_tournament,
        min_afs,
    };

    save_backer_filter(deps, &info.sender, &filter)?;

    Ok(Response::new()
        .add_attribute("action", "set_backing_filter")
        .add_attribute("backer", info.sender.to_string()))
}

pub fn query_is_player_eligible(
    deps: Deps,
    player: Addr,
    backer: Addr,
) -> Result<Binary, ContractError> {
    let player_nft = load_nft(deps, &player)?;
    let backer_filter = load_backer_filter(deps, &backer)?;

    let is_eligible = player_nft.abi >= backer_filter.min_abi
        && player_nft.games >= backer_filter.min_games
        && player_nft.games_per_month >= backer_filter.min_games_per_month
        && player_nft.roi >= backer_filter.min_roi
        && player_nft.dollars_per_tournament >= backer_filter.min_dollars_per_tournament
        && player_nft.afs >= backer_filter.min_afs;
    
    let response = json!({ "eligible": is_eligible }).to_string();
    Ok(Binary::from(response.as_bytes()))
}



    
