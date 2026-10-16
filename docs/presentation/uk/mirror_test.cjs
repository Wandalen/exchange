// Random sessions through the wasm build and src/mirror.js; exits 1 on the first answer that differs.
// Usage: node mirror_test.cjs [sessions]   (after build.py has built the wasm)
const { readFileSync } = require('node:fs');
const path = require('node:path');
const { createJsEngine } = require('./src/mirror.js');
const bytes = readFileSync(path.join(__dirname, 'engine/target/wasm32-unknown-unknown/release/deck_engine.wasm'));
const N = Number(process.argv[2] || 2000);

function wasmEngine() {
  const { exports: ex } = new WebAssembly.Instance(new WebAssembly.Module(bytes), {});
  const enc = new TextEncoder(), dec = new TextDecoder();
  return { call(cmd) {
    const b = enc.encode(cmd); const p = ex.io_input(b.length);
    new Uint8Array(ex.memory.buffer, p, b.length).set(b);
    const n = ex.io_run();
    return JSON.parse(dec.decode(new Uint8Array(ex.memory.buffer, ex.io_output(), n)));
  } };
}
let seed = 12345;
const rnd = (n) => { seed = (seed + 0x6D2B79F5) | 0; let t = Math.imul(seed ^ (seed >>> 15), 1 | seed); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return (((t ^ (t >>> 14)) >>> 0) % n); };
const pick = (a) => a[rnd(a.length)];
const norm = (o) => { const c = JSON.parse(JSON.stringify(o)); if (!c.ok) return { ok: false }; if (c.result.kind === 'hash') delete c.result.hash; return c; };
const tally = {};
let cmds = 0;
for (let run = 0; run < N; run++) {
  const w = wasmEngine(), j = createJsEngine();
  const bal = Array.from({ length: 4 }, () => [pick(['0', '5', '12.5', '40', '100', '100']), pick(['0', '3', '10', '25', '25'])]).flat();
  const log = ['reset ' + bal.join(' ')];
  let ids = 0;
  const len = 10 + rnd(50);
  for (let i = 0; i < len; i++) {
    const r = rnd(100);
    if (r < 75) {
      log.push(['place', 1 + rnd(4), pick(['buy', 'sell']), pick(['0.97', '0.98', '0.99', '1.00', '1.01', '1.02', '1.03', '1.005', '0', '-1']).replace(/^-1$/, rnd(10) ? '1.00' : '-1'),
        (rnd(20) ? String(1 + rnd(9)) : pick(['0', '1.5', '12'])), pick(['gtc', 'gtc', 'ioc', 'fok', 'post']), pick(['resting', 'incoming', 'both'])].join(' '));
      ids++;
    } else if (r < 92) log.push(`cancel ${1 + rnd(4)} ${rnd(ids + 1)}`);
    else log.push(rnd(6) ? pick(['resume', 'state', 'hash']) : 'halt');
  }
  for (const cmd of log) {
    cmds++;
    const a = norm(w.call(cmd)), b = norm(j.call(cmd));
    for (const e of (a.events || [])) { const ek = 'ev:' + e.kind + (e.cause ? ':' + e.cause : ''); tally[ek] = (tally[ek] || 0) + 1; }
    const k = a.ok ? (a.result.kind === 'rejected' ? 'rej:' + a.result.reason : a.result.kind + (a.result.stp ? '+stp' : '') + (a.result.tif_dropped ? '+tif' : '') + (a.result.trades ? '+trade' : '')) : 'error';
    tally[k] = (tally[k] || 0) + 1;
    if (JSON.stringify(a) !== JSON.stringify(b)) {
      console.log('MISMATCH run', run, 'cmd', cmd); console.log(log.slice(0, log.indexOf(cmd) + 1).join('\n'));
      console.log('wasm', JSON.stringify(a)); console.log('js  ', JSON.stringify(b)); process.exit(1);
    }
  }
}
console.log(`ok: ${N} runs, ${cmds} commands identical`); console.log(tally);
