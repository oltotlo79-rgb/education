import { startRest, stopRest } from './h.mjs';
import { runTs } from './run.mjs';
await startRest();
runTs('q6-reading.ts');
stopRest();
