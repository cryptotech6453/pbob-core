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

