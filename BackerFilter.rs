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
 unimplemented!() // Пока оставляем заглушку
}




    
