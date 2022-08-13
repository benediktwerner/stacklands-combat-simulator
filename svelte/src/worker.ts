import { sleep } from './utils';
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
  villagerSetup: CombatantStats[];
  enemySetup: CombatantStats[];
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
    setup.villagerSetup,
    setup.enemySetup,
    setup.monthStart,
    setup.monthLength
  );
  return { ...res, ...setup };
};

const startSimulation = async (
  setup: SimulateSetup,
  findMonthStart: boolean,
  findMonthStartStep: number
) => {
  await initPromise;

  let result: StatsWithSetup;
  running = true;

  if (findMonthStart) {
    setup.monthStart = 0;
    result = sim(setup);

    const first = Math.max(
      findMonthStartStep,
      findMonthStartStep *
        Math.floor((setup.monthLength - result.longest) / findMonthStartStep)
    );
    const progressUnit =
      (findMonthStartStep * 100) / (setup.monthLength - first + 1);
    let progress = progressUnit;

    send({
      type: 'progress',
      progress,
      newResult: result,
    });

    for (
      setup.monthStart = first;
      setup.monthStart < setup.monthLength;
      setup.monthStart += findMonthStartStep
    ) {
      await sleep();
      if (!running) break;

      const res = sim(setup);

      if (res.wins > result.wins) result = res;

      progress += progressUnit;
      send({
        type: 'progress',
        progress,
        newResult: res,
      });
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
