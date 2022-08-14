import type { CombatantSetup } from './types';
import { calculateVariations, sleep } from './utils';
import init, {
  simulate,
  type CombatantStats,
  type Stats,
} from './wasm/stacklands_combat_simulator.js';

export type MsgFromWorker =
  | { type: 'done' }
  | { type: 'cancelled' }
  | {
      type: 'progress';
      progress: number;
      newResult?: StatsWithSetup;
      replace?: boolean;
    };

export type MsgToWorker =
  | { type: 'cancel' }
  | {
      type: 'simulate';
      setup: SimulateSetup;
      findMonthStart: boolean;
      findMonthStartStep: number;
    };

export interface SimulateSetup {
  iterations: number;
  villagerSetup: CombatantSetup[];
  enemySetup: CombatantSetup[];
  monthStart: number;
  monthLength: number;
}

export type StatsWithSetup = Stats & SimulateSetup;

const initPromise = init();
let running = false;

const send = (msg: MsgFromWorker) => {
  postMessage(msg);
};

const STEP_ITERATIONS = 1000;

const addToResult = (old: StatsWithSetup, toAdd: Stats) => {
  old.iters += toAdd.iters;
  old.wins += toAdd.wins;
  old.total_length += toAdd.total_length;
  old.longest = Math.max(old.longest, toAdd.longest);
  for (const [i, v] of toAdd.enemy_hp.entries()) {
    old.enemy_hp[i] += v;
  }
  for (const [i, v] of toAdd.village_survivors.entries()) {
    old.village_survivors[i] += v;
  }
};

const sim = (
  setup: SimulateSetup,
  progressUnitOuter: number
): StatsWithSetup => {
  const doSim = (iters: number, setup: SimulateSetup) => {
    const res = simulate(
      iters,
      createSetup(setup.villagerSetup),
      createSetup(setup.enemySetup),
      setup.monthStart,
      setup.monthLength
    );
    return { ...res, ...setup };
  };

  const progressUnit =
    progressUnitOuter / Math.ceil(setup.iterations / STEP_ITERATIONS);

  const newResult = doSim(Math.min(setup.iterations, 1000), setup);
  send({
    type: 'progress',
    progress: progressUnit,
    newResult,
  });

  for (
    let iters = STEP_ITERATIONS;
    iters < setup.iterations;
    iters += STEP_ITERATIONS
  ) {
    const newPart = doSim(
      iters > setup.iterations ? iters - setup.iterations : STEP_ITERATIONS,
      setup
    );
    addToResult(newResult, newPart);
    send({
      type: 'progress',
      progress: progressUnit,
      newResult,
      replace: true,
    });
  }

  return newResult;
};

const createSetup = (combatants: CombatantSetup[]): CombatantStats[] => {
  const result = [];
  for (const c of combatants) {
    for (let i = 0; i < c.count; i++) result.push(c);
  }
  return result;
};

function* generateVariations(
  villagers: CombatantSetup[],
  enemies: CombatantSetup[],
  index = 0
): IterableIterator<void> {
  for (; index < villagers.length; index++) {
    const c = villagers[index];
    if (c.vary) {
      for (c.count = c.min_count!; c.count <= c.max_count!; c.count++) {
        for (const s of generateVariations(villagers, enemies, index + 1)) {
          yield s;
        }
      }
      return;
    }
  }
  for (; index - villagers.length < enemies.length; index++) {
    const c = enemies[index - villagers.length];
    if (c.vary) {
      for (c.count = c.min_count!; c.count <= c.max_count!; c.count++) {
        for (const s of generateVariations(villagers, enemies, index + 1)) {
          yield s;
        }
      }
      return;
    }
  }
  yield;
}

const simulateFindMonths = async (
  setup: SimulateSetup,
  findMonthStartStep: number,
  progressUnitOuter: number
): Promise<void> => {
  setup.monthStart = 0;
  const initialProgressUnit =
    progressUnitOuter / Math.floor(setup.monthLength / findMonthStartStep);
  const firstResult = sim(setup, initialProgressUnit);

  const first = Math.max(
    findMonthStartStep,
    findMonthStartStep *
      Math.floor((setup.monthLength - firstResult.longest) / findMonthStartStep)
  );
  const progressLeft = progressUnitOuter - initialProgressUnit;
  const stepsToDo = (setup.monthLength - first) / findMonthStartStep;
  const progressUnit = progressLeft / stepsToDo;

  for (
    setup.monthStart = first;
    setup.monthStart < setup.monthLength;
    setup.monthStart += findMonthStartStep
  ) {
    await sleep();
    if (!running) break;
    sim(setup, progressUnit);
  }
};

const startSimulation = async (
  setup: SimulateSetup,
  findMonthStart: boolean,
  findMonthStartStep: number
) => {
  await initPromise;

  running = true;

  if (
    findMonthStart ||
    setup.enemySetup.some((c) => c.vary) ||
    setup.villagerSetup.some((c) => c.vary)
  ) {
    const variations =
      calculateVariations(setup.villagerSetup) *
      calculateVariations(setup.enemySetup);
    const progressUnit = 100 / variations;
    for (const _ of generateVariations(setup.villagerSetup, setup.enemySetup)) {
      await sleep();
      if (!running) break;
      if (findMonthStart) {
        await simulateFindMonths(setup, findMonthStartStep, progressUnit);
      } else {
        sim(setup, progressUnit);
      }
    }
  } else {
    sim(setup, 100);
  }
  send({ type: 'done' });

  running = false;
};

onmessage = (e: MessageEvent<MsgToWorker>) => {
  const msg = e.data;

  switch (msg.type) {
    case 'cancel':
      running = false;
      send({ type: 'cancelled' });
      break;
    case 'simulate':
      startSimulation(msg.setup, msg.findMonthStart, msg.findMonthStartStep);
      break;
  }
};
