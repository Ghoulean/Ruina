/***
 * Exposes the following as public:
 *
 * pub const ABNO_PAGES: [AbnoPage; 156];
 * pub const BATTLE_SYMBOLS: [BattleSymbol; 200];
 * pub const COMBAT_PAGES: [CombatPage; 1613];
 * pub const KEY_PAGES: [KeyPage; 625];
 * pub const PASSIVES: [Passive; 808];
 */
include!(concat!(env!("OUT_DIR"), "/", env!("OUT_FILE")));