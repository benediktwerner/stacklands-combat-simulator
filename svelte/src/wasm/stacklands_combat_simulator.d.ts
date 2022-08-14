/* tslint:disable */
/* eslint-disable */
/**
* @param {number} iters
* @param {CombatantStats[]} villager_setup
* @param {CombatantStats[]} enemy_setup
* @param {number} start_time
* @param {number} month_length
* @returns {Stats}
*/
export function simulate(iters: number, villager_setup: CombatantStats[], enemy_setup: CombatantStats[], start_time: number, month_length: number): Stats;
export interface Stats {
    iters: number;
    wins: number;
    timeouts: number;
    total_length: number;
    longest: number;
    village_survivors: number[];
    enemy_hp: [number, number, number, number, number, number, number, number, number, number];
}

export interface CombatantStats {
    hp: number;
    attack_speed: number;
    hit_chance: number;
    min_damage: number;
    max_damage: number;
}


export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly simulate: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
