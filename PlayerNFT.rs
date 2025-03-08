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

