use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlayerNFT {
    pub owner: String,   // Владелец NFT
    pub abi: u32,        // Средний бай-ин турниров
    pub games: u32,      // Количество сыгранных турниров
    pub games_per_month: u32, // Среднее количество турниров в месяц
    pub roi: f64,        // ROI игрока
    pub dollars_per_tournament: f64, // Средний выигрыш с турнира
    pub afs: u32,        // Среднее количество участников в турнирах
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Storage};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlayerNFT {
    pub owner: String,
    pub abi: u32,     // Средний бай-ин турниров
    pub games: u32,   // Количество сыгранных турниров
    pub games_m: u32, // Турниры в месяц
    pub roi: i32,     // ROI в %
    pub dollars_t: u32, // Долларов с турнира
    pub afs: u32,     // Среднее число участников в турнирах
}

impl PlayerNFT {
    pub fn update_stats(
        &mut self,
        abi: u32,
        games: u32,
        games_m: u32,
        roi: i32,
        dollars_t: u32,
        afs: u32,
    ) {
        self.abi = abi;
        self.games = games;
        self.games_m = games_m;
        self.roi = roi;
        self.dollars_t = dollars_t;
        self.afs = afs;
    }
}

pub fn execute_update_stats(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    abi: u32,
    games: u32,
    games_m: u32,
    roi: i32,
    dollars_t: u32,
    afs: u32,
) -> StdResult<Response> {
    let mut player_nft: PlayerNFT = load_nft(deps.storage, &info.sender.to_string())?;
    player_nft.update_stats(abi, games, games_m, roi, dollars_t, afs);
    save_nft(deps.storage, &info.sender.to_string(), &player_nft)?;
    
    Ok(Response::new().add_attribute("action", "update_stats"))
}

fn load_nft(storage: &dyn Storage, owner: &str) -> StdResult<PlayerNFT> {
    // Заглушка: логика загрузки NFT из хранилища
    unimplemented!()
}

fn save_nft(storage: &mut dyn Storage, owner: &str, nft: &PlayerNFT) -> StdResult<()> {
    // Заглушка: логика сохранения NFT в хранилище
    unimplemented!()
}

const NFT_STORAGE: Map<&Addr, PlayerNFT> = Map::new("player_nfts");

pub fn load_nft(deps: Deps, owner: &Addr) -> StdResult<PlayerNFT> {
    NFT_STORAGE.load(deps.storage, owner)
}

pub fn save_nft(deps: DepsMut, owner: &Addr, nft: &PlayerNFT) -> StdResult<()> {
    NFT_STORAGE.save(deps.storage, owner, nft)
}

pub fn execute_mint_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
        let nft = PlayerNFT {
        owner: info.sender.clone(),
        abi: 0.0,
        games: 0,
        games_per_month: 0,
        roi: 0.0,
        dollars_per_tournament: 0.0,
        afs: 0.0,
    };

    save_nft(deps, &info.sender, &nft)?;
    
    Ok(Response::new()
        .add_attribute("action", "mint_nft")
        .add_attribute("owner", info.sender.to_string()))
}

pub fn execute_update_stats(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_abi: f64,
    new_games: u32,
    new_games_per_month: u32,
    new_roi: f64,
    new_dollars_per_tournament: f64,
    new_afs: f64,
) -> Result<Response, ContractError> {
    let mut nft = load_nft(deps.storage, &info.sender)?;

    nft.abi = new_abi;
    nft.games = new_games;
    nft.games_per_month = new_games_per_month;
    nft.roi = new_roi;
    nft.dollars_per_tournament = new_dollars_per_tournament;
    nft.afs = new_afs;

    save_nft(deps, &info.sender, &nft)?;

    Ok(Response::new()
        .add_attribute("action", "update_stats")
        .add_attribute("owner", info.sender.to_string()))
}

pub fn query_player_stats(deps: Deps, player: Addr) -> Result<Binary, ContractError> {
    let player_nft = load_nft(deps, &player)?;

    let response = json!({
        "owner": player_nft.owner,
        "abi": player_nft.abi,
        "games": player_nft.games,
        "games_per_month": player_nft.games_per_month,
        "roi": player_nft.roi,
        "dollars_per_tournament": player_nft.dollars_per_tournament,
        "afs": player_nft.afs
    })
    .to_string();

    Ok(Binary::from(response.as_bytes()))
}








