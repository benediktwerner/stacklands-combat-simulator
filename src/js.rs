use wasm_bindgen::{prelude::*, JsCast};

use crate::CombatantStats;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CombatantStats[]")]
    pub type JsCombatantStatsArray;
    #[wasm_bindgen(typescript_type = "Stats")]
    pub type JsStats;
}

#[wasm_bindgen]
pub fn simulate(
    iters: u32,
    villager_setup: JsCombatantStatsArray,
    enemy_setup: JsCombatantStatsArray,
    start_time: u32,
    month_length: u32,
) -> Result<JsStats, JsValue> {
    let villager_setup: Vec<CombatantStats> =
        villager_setup.into_serde().map_err(|e| e.to_string())?;
    let enemy_setup: Vec<CombatantStats> = enemy_setup.into_serde().map_err(|e| e.to_string())?;
    let stats = crate::simulate(
        iters,
        &villager_setup,
        &enemy_setup,
        start_time,
        month_length,
    );
    JsValue::from_serde(&stats)
        .map(JsValue::unchecked_into)
        .map_err(|e| e.to_string().into())
}
