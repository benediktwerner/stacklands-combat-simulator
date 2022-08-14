import type { CombatantSetup } from './types';
import { calculateVariations, sleep } from './utils';
import init, {
  simulate,
  type CombatantStats,
  type Stats,
} from './wasm/stacklands_combat_simulator.js';

export type MsgFromWorker =
  | { type: 'done'; newResult?: StatsWithSetup }
  | { type: 'cancelled' }
  | { type: 'progress'; progress: number; newResult?: StatsWithSetup };

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

const sim = (setup: SimulateSetup): StatsWithSetup => {
  const res = simulate(
    setup.iterations,
    createSetup(setup.villagerSetup),
    createSetup(setup.enemySetup),
    setup.monthStart,
    setup.monthLength
  );
  return { ...res, ...setup };
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
  progress: number,
  progressUnitOuter: number
): Promise<number> => {
  setup.monthStart = 0;
  const firstResult = sim(setup);

  const first = Math.max(
    findMonthStartStep,
    findMonthStartStep *
      Math.floor((setup.monthLength - firstResult.longest) / findMonthStartStep)
  );
  const progressUnit =
    (findMonthStartStep * progressUnitOuter) /
    (setup.monthLength - first + findMonthStartStep);
  progress += progressUnit;

  send({
    type: 'progress',
    progress,
    newResult: firstResult,
  });

  for (
    setup.monthStart = first;
    setup.monthStart < setup.monthLength;
    setup.monthStart += findMonthStartStep
  ) {
    await sleep();
    if (!running) break;

    const res = sim(setup);

    progress += progressUnit;
    send({
      type: 'progress',
      progress,
      newResult: res,
    });
  }

  return progress;
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
    let progress = 0;
    for (const _ of generateVariations(setup.villagerSetup, setup.enemySetup)) {
      await sleep();
      if (!running) break;
      if (findMonthStart) {
        progress = await simulateFindMonths(
          setup,
          findMonthStartStep,
          progress,
          progressUnit
        );
      } else {
        progress += progressUnit;
        send({
          type: 'progress',
          progress,
          newResult: sim(setup),
        });
      }
    }
    send({ type: 'done' });
  } else {
    send({
      type: 'done',
      newResult: sim(setup),
    });
  }

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
