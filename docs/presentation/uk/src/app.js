(() => {
'use strict';

const NAMES = { 1: 'Аліса', 2: 'Богдан', 3: 'Віра', 4: 'Гліб' };
const INITIAL = { 1: 'А', 2: 'Б', 3: 'В', 4: 'Г' };
const TIF_UA = { gtc: 'GTC', ioc: 'IOC', fok: 'FOK', post: 'Post-only' };
const REASON_UA = {
  InsufficientFunds: (o) => o && o.side === 'sell' ? 'недостатньо вільних яблук' : 'недостатньо вільних коштів',
  Halted: () => 'торги зупинено',
  PostOnlyWouldTake: () => 'post-only торгувала б одразу',
  PriceOffTick: () => 'ціна не кратна кроку 0,01 ₴',
  QuantityOffLot: () => 'кількість має бути цілою',
  ZeroQuantity: () => 'нульова кількість',
  NegativePrice: () => 'від’ємна ціна',
};
const reasonUa = (r, o) => (REASON_UA[r] ? REASON_UA[r](o) : r);

const cents = (s) => Math.round(Number(s) * 100);
const fmtC = (c) => (c / 100).toFixed(2).replace('.', ',');
const money = (s) => fmtC(cents(s));
const icon = (id) => `<svg class="ico" aria-hidden="true"><use href="#i-${id}"/></svg>`;
const person = (a) => `<span class="person" data-t="${a}">${NAMES[a]}</span>`;
const oblig = (o) => (o.cash != null ? `${money(o.cash)} ₴` : `${o.apples} шт.`);
const verb = (side) => (side === 'buy' ? 'купує' : 'продає');
const placeCmd = (o, stp) => `place ${o.acct} ${o.side} ${o.price} ${o.qty} ${o.tif} ${stp}`;
const orderNo = (id) => `№${id + 1}`;

/* ---------- engine: Rust via WebAssembly, JS mirror as fallback ---------- */
let wasmModule = null;
async function bootEngine() {
  try {
    const bin = atob(document.getElementById('wasm-b64').textContent.trim());
    const bytes = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
    wasmModule = await WebAssembly.compile(bytes);
    const probe = await newEngine();
    if (!probe.call('reset 0 0 0 0 0 0 0 0').ok) throw new Error('probe');
  } catch (e) {
    wasmModule = null;
  }
  const badge = document.getElementById('engine-badge');
  if (wasmModule) {
    badge.textContent = 'Rust → WebAssembly';
    badge.classList.add('wasm');
    badge.title = 'Стакан рахує exchange_core, скомпільований у wasm32 і вбудований у сторінку';
  } else {
    badge.textContent = 'JS-дзеркало рушія';
    badge.title = 'WebAssembly недоступний у цьому вікні. Працює JS-копія правил, звірена з Rust на 177 тис. команд. Відкрийте HTML-файл локально, щоб запустити Rust.';
  }
}
async function newEngine() {
  if (!wasmModule) return createJsEngine();
  const inst = await WebAssembly.instantiate(wasmModule, {});
  const ex = inst.exports, enc = new TextEncoder(), dec = new TextDecoder();
  let dead = false;
  return {
    kind: 'wasm',
    call(cmd) {
      if (dead) return { ok: false, error: 'рушій зупинився' };
      try {
        const b = enc.encode(cmd);
        const p = ex.io_input(b.length);
        new Uint8Array(ex.memory.buffer, p, b.length).set(b);
        const n = ex.io_run();
        return JSON.parse(dec.decode(new Uint8Array(ex.memory.buffer, ex.io_output(), n)));
      } catch (e) {
        dead = true;
        return { ok: false, error: 'рушій зупинився' };
      }
    },
  };
}

/* ---------- order book ladder ---------- */
function rowsOf(book) {
  const out = [];
  for (const side of ['sell', 'buy']) for (const l of book[side]) out.push({ side, p: cents(l.price), orders: l.orders });
  return out;
}

function renderBook(el, opt) {
  const { prev, cur, events = [], prices = [], clickable = false, selected = null, still = false, note = '' } = opt;
  const hits = new Map();
  for (const e of events) if (e.kind === 'trade') hits.set(e.maker, (hits.get(e.maker) || 0) + Number(e.qty));
  const rows = new Map();
  const row = (side, p) => {
    const k = `${side}:${p}`;
    if (!rows.has(k)) rows.set(k, { side, p, prev: [], cur: [] });
    return rows.get(k);
  };
  for (const r of rowsOf(prev.book)) row(r.side, r.p).prev = r.orders;
  for (const r of rowsOf(cur.book)) row(r.side, r.p).cur = r.orders;
  const used = new Set([...rows.values()].map((r) => r.p));
  for (const p of prices) if (!used.has(p)) { row('idle', p); used.add(p); }
  const rank = { sell: 0, idle: 1, buy: 2 };
  const list = [...rows.values()].sort((a, b) => b.p - a.p || rank[a.side] - rank[b.side]);

  let maxQty = 1;
  for (const r of list) {
    const curIds = new Map(r.cur.map((o) => [o.id, o]));
    const prevIds = new Set(r.prev.map((o) => o.id));
    r.chips = [];
    for (const o of r.prev) {
      const c = curIds.get(o.id);
      if (c) r.chips.push({ o: c, st: hits.has(o.id) ? 'hit' : 'kept', delta: Number(o.qty) - Number(c.qty) });
      else r.chips.push({ o, st: 'gone' });
    }
    for (const o of r.cur) if (!prevIds.has(o.id)) r.chips.push({ o, st: 'new' });
    r.sum = r.cur.reduce((s, o) => s + Number(o.qty), 0);
    maxQty = Math.max(maxQty, r.chips.reduce((s, c) => s + Number(c.o.qty), 0));
  }
  const track = Math.max(120, (el.clientWidth || 520) - 230);
  const unit = Math.max(5, Math.min(30, (track - 4 * 6) / maxQty));

  const bestAsk = cur.book.sell.length ? cents(cur.book.sell[0].price) : null;
  const bestBid = cur.book.buy.length ? cents(cur.book.buy[0].price) : null;
  let spread;
  if (bestAsk != null && bestBid != null) spread = `спред ${fmtC(bestAsk - bestBid)} ₴`;
  else if (bestAsk != null) spread = 'покупців немає';
  else if (bestBid != null) spread = 'продавців немає';
  else spread = 'стакан порожній';
  const lastSell = list.map((r) => r.side).lastIndexOf('sell');
  const firstBuy = list.findIndex((r) => r.side === 'buy');
  const divAt = lastSell >= 0 ? lastSell + 1 : firstBuy >= 0 ? firstBuy : Math.floor(list.length / 2);

  const chipHtml = (c, side, p) => {
    const o = c.o, q = Number(o.qty);
    const tag = clickable && c.st !== 'gone' ? 'button' : 'span';
    const cls = ['chip', c.st === 'kept' ? '' : c.st, selected === o.id ? 'sel' : ''].filter(Boolean).join(' ');
    const title = `${NAMES[o.acct]} ${verb(side)} ${o.qty} × ${fmtC(p)} ₴ · ${orderNo(o.id)}${o.tif !== 'gtc' ? ' · ' + TIF_UA[o.tif] : ''}${c.st === 'gone' ? ' · знято зі стакану' : ''}`;
    const delta = c.delta > 0 ? `<span class="delta">−${c.delta}</span>` : '';
    const tg = o.tif === 'post' ? '<span class="tg">post</span>' : '';
    return `<${tag} class="${cls}" data-t="${o.acct}" data-id="${o.id}" style="--q:${q}" title="${title}" aria-label="${title}"${tag === 'button' ? ' type="button"' : ''}><span class="who">${INITIAL[o.acct]}</span><span class="q">${q}</span>${tg}${delta}</${tag}>`;
  };

  let html = `<div class="book-head"><b>Стакан · яблука за ₴</b><span>${note || 'зліва перший у черзі'}</span></div>`;
  html += `<div class="ladder${still ? ' still' : ''}" style="--u:${unit.toFixed(1)}px">`;
  html += `<div class="side-tag ask-tag"><span>Продаж</span><span>к-сть</span></div>`;
  list.forEach((r, i) => {
    if (i === divAt) html += `<div class="spread">${spread}</div>`;
    const cls = r.side === 'sell' ? 'ask' : r.side === 'buy' ? 'bid' : 'idle';
    html += `<div class="lrow ${cls}"><span class="px">${fmtC(r.p)}</span><div class="track">${r.chips.map((c) => chipHtml(c, r.side, r.p)).join('')}</div><span class="sum">${r.sum || ''}</span></div>`;
  });
  if (divAt >= list.length) html += `<div class="spread">${spread}</div>`;
  html += `<div class="side-tag bid-tag"><span>Купівля</span><span></span></div></div>`;
  el.innerHTML = html;
}

/* ---------- balances, tape, footer ---------- */
function renderBalances(el, prev, cur, still) {
  const cols = [['cash_free', '₴', false], ['cash_locked', '₴', true], ['apples_free', 'шт.', false], ['apples_locked', 'шт.', true]];
  let html = `<table class="bal${still ? ' still' : ''}"><thead><tr><th>Хто</th><th>₴ вільно</th><th>${icon('lock')} ₴</th><th>${icon('apple')} вільно</th><th>${icon('lock')} ${icon('apple')}</th></tr></thead><tbody>`;
  for (const a of cur.accounts) {
    const p = prev.accounts.find((x) => x.id === a.id) || a;
    html += `<tr><td>${person(a.id)}</td>`;
    for (const [k, , locked] of cols) {
      const isCash = k.startsWith('cash');
      const v = isCash ? cents(a[k]) : Number(a[k]);
      const d = v - (isCash ? cents(p[k]) : Number(p[k]));
      const shown = isCash ? fmtC(v) : String(v);
      const dl = d && !still ? `<span class="dl">${d > 0 ? '+' : '−'}${isCash ? fmtC(Math.abs(d)) : Math.abs(d)}</span>` : '';
      const cls = [locked ? 'lock' : '', locked && v ? 'has' : '', d && !still ? 'chg' : ''].filter(Boolean).join(' ');
      html += `<td class="${cls}">${shown}${dl}</td>`;
    }
    html += '</tr>';
  }
  el.innerHTML = html + '</tbody></table>';
}

function eventLine(e, ctx) {
  const n = orderNo(e.order);
  switch (e.kind) {
    case 'accepted':
      return ['acc', `Прийнято ${n}: ${person(e.acct)} ${verb(e.side)} ${e.qty} × ${money(e.price)} ₴ · заблоковано ${oblig(e.reserved)}`];
    case 'trade': {
      const buyer = e.taker_side === 'buy' ? e.acct : e.maker_acct, seller = e.taker_side === 'buy' ? e.maker_acct : e.acct;
      return ['trade', `Угода: ${person(seller)} → ${person(buyer)} · ${e.qty} шт. × ${money(e.price)} ₴ = ${fmtC(cents(e.price) * Number(e.qty))} ₴`];
    }
    case 'cancelled': {
      if (e.cause === 'request') return ['cxl', `Скасовано ${n} на запит ${person(e.acct)}: ${e.qty} шт., повернуто ${oblig(e.released)}`];
      if (e.cause === 'self_match') return ['cxl', `Self-trade: знято ${n} (${person(e.acct)}), ${e.qty} шт., повернуто ${oblig(e.released)}`];
      return ['cxl', `Залишок ${n} скасовано за часом життя: ${e.qty} шт., повернуто ${oblig(e.released)}`];
    }
    case 'rejected':
      return ['rej', `Відхилено ${n} (${person(e.acct)}): ${reasonUa(e.reason, ctx)}`];
    default:
      return ['', e.kind];
  }
}
const tapeItem = (e, ctx) => {
  const [cls, html] = eventLine(e, ctx);
  return `<li class="${cls}"><span class="sq">#${e.seq}</span><span>${html}</span></li>`;
};

function renderFoot(el, cur) {
  let cash = 0, apples = 0;
  for (const a of cur.accounts) { cash += cents(a.cash_free) + cents(a.cash_locked); apples += Number(a.apples_free) + Number(a.apples_locked); }
  const ok = cur.conserved ? '<span class="ok">✓ баланс зійшовся</span>' : '<span class="bad">✗ баланс не зійшовся</span>';
  el.innerHTML = `<span>Σ ${fmtC(cash)} ₴ · Σ ${apples} шт.</span>${ok}<span>перевірка exchange_conserve</span>${cur.halted ? '<span class="badge bad">торги зупинено</span>' : ''}`;
}

/* ---------- the "incoming" card ---------- */
function outcomeBadges(order, ans) {
  if (!ans.ok) return [['bad', `Помилка: ${ans.error}`]];
  const r = ans.result;
  if (r.kind === 'rejected') return [['bad', `Відхилено: ${reasonUa(r.reason, order)}`]];
  if (r.kind === 'cancelled') return [['ok', `Скасовано ${r.qty} шт.`]];
  if (r.kind === 'not_resting') return [['bad', 'Такої заявки в стакані немає']];
  if (r.kind === 'halted') return [['bad', 'Нові заявки відхиляються']];
  if (r.kind === 'resumed') return [['ok', 'Торги відновлено']];
  if (r.kind !== 'placed') return [];
  const trades = ans.events.filter((e) => e.kind === 'trade');
  const filled = trades.reduce((s, e) => s + Number(e.qty), 0);
  const out = [];
  if (filled) out.push(['ok', `${order.side === 'buy' ? 'Куплено' : 'Продано'} ${filled} з ${order.qty}`]);
  if (order.side === 'buy' && filled) {
    const back = trades.reduce((s, e) => s + (cents(order.price) - cents(e.price)) * Number(e.qty), 0);
    if (back > 0) out.push(['mid', `Різниця ${fmtC(back)} ₴ повернулась`]);
  }
  if (Number(r.resting)) out.push(['mid', `У стакані: ${r.resting} шт.`]);
  if (r.tif_dropped) {
    const c = ans.events.find((e) => e.kind === 'cancelled' && e.cause === 'tif');
    out.push(['bad', filled ? `Залишок ${c.qty} шт. скасовано` : `${TIF_UA[order.tif]}: скасовано цілком`]);
  }
  for (const e of ans.events.filter((x) => x.kind === 'cancelled' && x.cause === 'self_match'))
    out.push(['bad', e.order === r.order ? 'Self-trade: нову заявку скасовано' : `Self-trade: знято ${orderNo(e.order)}`]);
  if (!out.length) out.push(['mid', 'Нічого не виконано']);
  return out;
}

function renderIncoming(el, { tag, order, cmd, ans, idle }) {
  let body;
  if (idle) body = `<div class="inc-order">${idle}</div>`;
  else if (order) {
    const lim = order.side === 'buy' ? 'до' : 'від';
    body = `<div class="inc-order">${person(order.acct)} <span class="side-${order.side}">${verb(order.side)}</span> <span class="mono">${order.qty}</span> шт. ${lim} <span class="mono">${money(order.price)}</span> ₴ <span class="badge">${TIF_UA[order.tif]}</span></div>`;
  } else body = `<div class="inc-order">${cmd}</div>`;
  const badges = ans ? `<div class="outcome">${outcomeBadges(order, ans).map(([c, t]) => `<span class="badge ${c}">${t}</span>`).join('')}</div>` : '';
  el.innerHTML = `<div class="inc-head"><span class="tag">${tag}</span></div>${body}${badges}`;
  el.classList.remove('flash');
  void el.offsetWidth;
  el.classList.add('flash');
}

/* ---------- scripted demo ---------- */
const demo = {
  s: 0, v: 0, k: 0, cache: new Map(), engine: null,
  el: {},
  variants(sc) { return sc.variants || [{ key: '-', label: '', steps: sc.steps }]; },
  prices(sc) {
    const ps = new Set();
    const add = (o) => o && ps.add(cents(o.price));
    (sc.setup || []).forEach(add);
    for (const v of this.variants(sc)) v.steps.forEach((st) => add(st.order));
    return [...ps];
  },
  timeline(sc, v) {
    const key = `${sc.id}/${v.key}`;
    if (this.cache.has(key)) return this.cache.get(key);
    const call = (c) => this.engine.call(c);
    const stp = v.stp || 'incoming';
    let ans = call('reset ' + sc.reset.join(' '));
    for (const o of sc.setup || []) ans = call(placeCmd(o, stp));
    const states = [ans], cmds = [];
    for (const st of v.steps) {
      let cmd = st.cmd;
      if (st.order) cmd = placeCmd(st.order, stp);
      if (st.cancelOwn) {
        const o = [...ans.book.sell, ...ans.book.buy].flatMap((l) => l.orders).find((x) => x.acct === st.cancelOwn);
        cmd = `cancel ${st.cancelOwn} ${o ? o.id : 999}`;
      }
      ans = call(cmd);
      states.push(ans);
      cmds.push(cmd);
    }
    const t = { states, cmds };
    this.cache.set(key, t);
    return t;
  },
  init() {
    const $ = (id) => document.getElementById(id);
    this.el = { list: $('scn-list'), seg: $('variant-seg'), rule: $('scn-rule'), say: $('scn-say'), dots: $('step-dots'), prev: $('step-prev'), next: $('step-next'), inc: $('demo-inc'), book: $('demo-book'), bal: $('demo-bal'), tape: $('demo-tape'), foot: $('demo-foot') };
    this.el.list.innerHTML = SCENARIOS.map((sc, i) => `<button class="scn" type="button" data-i="${i}"><span class="k">${i + 1}</span><span>${sc.title}</span></button>`).join('');
    this.el.list.addEventListener('click', (e) => { const b = e.target.closest('.scn'); if (b) this.pick(Number(b.dataset.i), 0, 0); });
    this.el.seg.addEventListener('click', (e) => { const b = e.target.closest('button'); if (b) this.pick(this.s, Number(b.dataset.i), 0); });
    this.el.prev.addEventListener('click', () => this.back());
    this.el.next.addEventListener('click', () => this.forward());
  },
  pick(s, v, k) { this.s = s; this.v = v; this.k = k; this.render(); },
  steps() { return this.variants(SCENARIOS[this.s])[this.v].steps.length; },
  forward() {
    if (this.k < this.steps()) { this.k++; this.render(); return true; }
    const sc = SCENARIOS[this.s];
    if (this.v + 1 < this.variants(sc).length) { this.pick(this.s, this.v + 1, 0); return true; }
    if (this.s + 1 < SCENARIOS.length) { this.pick(this.s + 1, 0, 0); return true; }
    return false;
  },
  back() {
    if (this.k > 0) { this.k--; this.render(); return true; }
    if (this.v > 0) { const v = this.v - 1; this.pick(this.s, v, this.variants(SCENARIOS[this.s])[v].steps.length); return true; }
    if (this.s > 0) { const s = this.s - 1, vs = this.variants(SCENARIOS[s]); this.pick(s, vs.length - 1, vs[vs.length - 1].steps.length); return true; }
    return false;
  },
  render(still = false) {
    if (!this.engine) return;
    const sc = SCENARIOS[this.s], vs = this.variants(sc), v = vs[this.v], k = this.k;
    const t = this.timeline(sc, v);
    const cur = t.states[k], prev = k > 0 ? t.states[k - 1] : cur, step = k > 0 ? v.steps[k - 1] : null;
    [...this.el.list.children].forEach((b, i) => b.setAttribute('aria-pressed', String(i === this.s)));
    this.el.seg.hidden = !sc.variants;
    if (sc.variants) this.el.seg.innerHTML = vs.map((x, i) => `<button type="button" data-i="${i}" aria-pressed="${i === this.v}">${x.label}</button>`).join('');
    this.el.rule.textContent = sc.rule;
    this.el.say.textContent = step ? step.say : (sc.setup ? 'У стакані вже є заявки. Натисніть «Далі».' : 'Стакан порожній. Натисніть «Далі».');
    this.el.dots.innerHTML = v.steps.map((_, i) => `<i class="${i < k - 1 ? 'done' : i === k - 1 ? 'on' : ''}"></i>`).join('');
    this.el.prev.disabled = this.s === 0 && this.v === 0 && k === 0;
    const policy = v.stp ? ` · self-trade: ${v.label.toLowerCase()}` : '';
    const tag = step ? `Крок ${k} з ${v.steps.length}${policy}` : `Початковий стан${policy}`;
    if (!step) renderIncoming(this.el.inc, { tag, idle: sc.setup ? 'Стартовий стакан' : 'Порожній стакан' });
    else if (step.order) renderIncoming(this.el.inc, { tag, order: step.order, ans: cur });
    else {
      const what = step.cmd === 'halt' ? 'Оператор зупиняє торги' : step.cmd === 'resume' ? 'Оператор відновлює торги' : `${person(step.cancelOwn)} скасовує свою заявку`;
      renderIncoming(this.el.inc, { tag, cmd: what, ans: cur });
    }
    const events = step ? cur.events : [];
    renderBook(this.el.book, { prev, cur, events, prices: this.prices(sc), still: still || !step });
    renderBalances(this.el.bal, prev, cur, still || !step);
    this.el.tape.innerHTML = events.length ? events.map((e) => tapeItem(e, step && step.order)).join('') : '<li><span></span><span class="empty">Подій ще немає</span></li>';
    renderFoot(this.el.foot, cur);
  },
};

/* ---------- sandbox ---------- */
const SB_RESET = ['500', '20', '500', '20', '500', '20', '60', '0'];
const SB_SETUP = [
  { acct: 1, side: 'sell', price: '10.10', qty: 5, tif: 'gtc' },
  { acct: 3, side: 'sell', price: '10.20', qty: 8, tif: 'gtc' },
  { acct: 3, side: 'sell', price: '10.30', qty: 6, tif: 'gtc' },
  { acct: 2, side: 'buy', price: '9.90', qty: 6, tif: 'gtc' },
  { acct: 3, side: 'buy', price: '9.90', qty: 3, tif: 'gtc' },
  { acct: 1, side: 'buy', price: '9.80', qty: 10, tif: 'gtc' },
];
const sb = {
  engine: null, log: [], cur: null, prev: null, events: [], tape: [], last: null, selected: null, form: { acct: 2, side: 'buy', tif: 'gtc' },
  el: {},
  async reset() {
    this.engine = await newEngine();
    this.log = [];
    this.tape = [];
    this.selected = null;
    this.exec('reset ' + SB_RESET.join(' '));
    for (const o of SB_SETUP) this.exec(placeCmd(o, 'incoming'));
    this.prev = this.cur;
    this.events = [];
    this.last = null;
    this.el.replayOut.innerHTML = '';
    renderIncoming(this.el.inc, { tag: 'Пісочниця', idle: 'Поставте заявку ліворуч або натисніть на заявку в стакані, щоб скасувати' });
    this.render(true);
  },
  exec(cmd, ctx) {
    const ans = this.engine.call(cmd);
    if (ans.ok) {
      this.log.push(cmd);
      this.prev = this.cur || ans;
      this.cur = ans;
      this.events = ans.events;
      for (const e of ans.events) this.tape.unshift(tapeItem(e, ctx));
      this.tape.length = Math.min(this.tape.length, 60);
    }
    return ans;
  },
  init() {
    const $ = (id) => document.getElementById(id);
    this.el = { form: $('sb-form'), who: $('sb-who'), side: $('sb-side'), tif: $('sb-tif'), price: $('sb-price'), qty: $('sb-qty'), stp: $('sb-stp'), cost: $('sb-cost'), halt: $('sb-halt'), replay: $('sb-replay'), replayOut: $('sb-replay-out'), resetBtn: $('sb-reset'), inc: $('sb-inc'), book: $('sb-book'), sel: $('sb-selected'), bal: $('sb-bal'), tape: $('sb-tape'), foot: $('sb-foot'), count: $('sb-count') };
    this.el.who.innerHTML = [1, 2, 3, 4].map((a) => `<button type="button" data-v="${a}">${NAMES[a]}</button>`).join('');
    const seg = (el, key, cast) => el.addEventListener('click', (e) => {
      const b = e.target.closest('button[data-v]');
      if (!b) return;
      this.form[key] = cast(b.dataset.v);
      this.syncForm();
    });
    seg(this.el.who, 'acct', Number);
    seg(this.el.side, 'side', String);
    seg(this.el.tif, 'tif', String);
    this.el.form.addEventListener('click', (e) => {
      const b = e.target.closest('button[data-d], button[data-q]');
      if (!b) return;
      if (b.dataset.d) this.el.price.value = Math.max(0, cents(this.el.price.value.replace(',', '.')) / 100 + Number(b.dataset.d)).toFixed(2);
      else this.el.qty.value = String(Math.max(1, (parseInt(this.el.qty.value, 10) || 0) + Number(b.dataset.q)));
      this.syncForm();
    });
    this.el.price.addEventListener('input', () => this.syncForm());
    this.el.qty.addEventListener('input', () => this.syncForm());
    this.el.form.addEventListener('submit', (e) => { e.preventDefault(); this.place(); });
    this.el.halt.addEventListener('click', () => this.toggleHalt());
    this.el.replay.addEventListener('click', () => this.replay());
    this.el.resetBtn.addEventListener('click', () => this.reset());
    this.el.book.addEventListener('click', (e) => {
      const c = e.target.closest('button.chip');
      if (!c) return;
      const id = Number(c.dataset.id);
      this.selected = this.selected === id ? null : id;
      this.render(true);
    });
    this.el.sel.addEventListener('click', (e) => { if (e.target.closest('[data-cancel]')) this.cancelSelected(); });
    this.syncForm();
  },
  orderFromForm() {
    const price = this.el.price.value.trim().replace(',', '.');
    const qty = this.el.qty.value.trim();
    return { acct: this.form.acct, side: this.form.side, price, qty, tif: this.form.tif };
  },
  syncForm() {
    const f = this.form;
    for (const [el, v] of [[this.el.who, String(f.acct)], [this.el.side, f.side], [this.el.tif, f.tif]])
      [...el.querySelectorAll('button[data-v]')].forEach((b) => b.setAttribute('aria-pressed', String(b.dataset.v === v)));
    const o = this.orderFromForm();
    const valid = /^\d+(\.\d+)?$/.test(o.price) && /^\d+$/.test(o.qty);
    this.el.form.querySelector('#sb-place').disabled = !valid;
    if (!valid || !this.cur) { this.el.cost.textContent = valid ? '' : 'Ціна — число на кшталт 10.20, кількість — ціле число.'; return; }
    const a = this.cur.accounts.find((x) => x.id === o.acct);
    this.el.cost.innerHTML = o.side === 'buy'
      ? `Буде заблоковано ${fmtC(Math.round(Number(o.price) * 100) * Number(o.qty))} ₴ · вільно ${money(a.cash_free)} ₴`
      : `Буде заблоковано ${o.qty} шт. · вільно ${a.apples_free} шт.`;
  },
  place() {
    const o = this.orderFromForm();
    const ans = this.exec(placeCmd(o, this.el.stp.value), o);
    this.last = o;
    renderIncoming(this.el.inc, { tag: ans.ok && ans.result.order != null ? `Заявка ${orderNo(ans.result.order)}` : 'Заявка', order: o, ans });
    this.render();
  },
  cancelSelected() {
    const o = this.findOrder(this.selected);
    if (!o) return;
    const ans = this.exec(`cancel ${o.acct} ${o.id}`);
    renderIncoming(this.el.inc, { tag: `Скасування ${orderNo(o.id)}`, cmd: `${person(o.acct)} скасовує свою заявку`, ans });
    this.selected = null;
    this.render();
  },
  toggleHalt() {
    const cmd = this.cur.halted ? 'resume' : 'halt';
    const ans = this.exec(cmd);
    renderIncoming(this.el.inc, { tag: 'Оператор', cmd: cmd === 'halt' ? 'Оператор зупиняє торги' : 'Оператор відновлює торги', ans });
    this.render();
  },
  async replay() {
    const h1 = this.engine.call('hash').result.hash;
    const second = await newEngine();
    for (const c of this.log) second.call(c);
    const h2 = second.call('hash').result.hash;
    const same = h1 === h2;
    this.el.replayOut.innerHTML = `<span>Прогін 1: ${h1}</span><span>Прогін 2: ${h2}</span><span class="${same ? 'ok' : 'bad'}">${same ? '✓ однаковий хеш' : '✗ хеші різні'} · ${this.log.length} команд, окремий екземпляр рушія</span>`;
  },
  findOrder(id) {
    if (id == null || !this.cur) return null;
    for (const side of ['sell', 'buy']) for (const l of this.cur.book[side]) for (const o of l.orders) if (o.id === id) return { ...o, side, price: l.price };
    return null;
  },
  render(still = false) {
    if (!this.cur) return;
    const prices = this.last && /^\d+(\.\d+)?$/.test(this.last.price) ? [cents(this.last.price)] : [];
    renderBook(this.el.book, { prev: still ? this.cur : this.prev, cur: this.cur, events: still ? [] : this.events, prices, clickable: true, selected: this.selected, still, note: 'натисніть на заявку, щоб скасувати' });
    renderBalances(this.el.bal, still ? this.cur : this.prev, this.cur, still);
    this.el.tape.innerHTML = this.tape.length ? this.tape.join('') : '<li><span></span><span class="empty">Подій ще немає</span></li>';
    this.el.count.textContent = `${this.cur.stats.fills} угод · ${this.cur.stats.rejects} відмов`;
    renderFoot(this.el.foot, this.cur);
    this.el.halt.textContent = this.cur.halted ? 'Відновити торги' : 'Зупинити торги';
    const o = this.findOrder(this.selected);
    this.el.sel.innerHTML = o ? `<span>Обрано ${orderNo(o.id)}: ${person(o.acct)} ${verb(o.side)} ${o.qty} × ${money(o.price)} ₴</span><button class="btn" type="button" data-cancel>Скасувати від імені ${NAMES[o.acct] === 'Аліса' ? 'Аліси' : NAMES[o.acct] === 'Віра' ? 'Віри' : NAMES[o.acct] === 'Богдан' ? 'Богдана' : 'Гліба'}</button>` : '';
    this.syncForm();
  },
};

/* ---------- static books on the title and glossary slides ---------- */
let glossState = null;
async function buildGloss() {
  const e = await newEngine();
  e.call('reset 100 10 300 0 0 5 100 10');
  for (const o of [[4, 'sell', '10.20', 10], [1, 'sell', '10.00', 10], [3, 'sell', '10.00', 5], [2, 'buy', '9.90', 6], [4, 'buy', '9.80', 4]])
    glossState = e.call(placeCmd({ acct: o[0], side: o[1], price: o[2], qty: o[3], tif: 'gtc' }, 'incoming'));
}
function renderStatic() {
  if (!glossState) return;
  for (const id of ['title-book', 'gloss-book']) {
    const el = document.getElementById(id);
    if (el.offsetParent !== null) renderBook(el, { prev: glossState, cur: glossState, still: true });
  }
}

/* ---------- speaker notes and the full script ---------- */
const NOTES = {
  title: { t: '0:00', text: 'Добрий день! Наш проєкт називається exchange. Це ядро біржі, написане на Rust: заявки заходять, угоди виходять, і жодна копійка при цьому не з’являється і не зникає.' },
  exchange: { t: '0:15', text: 'Коротко, як працює біржа. Заявка — це «купити або продати стільки-то яблук за таку-то ціну». Ті, що не знайшли пари, чекають у стакані: продавці зверху, покупці знизу. Черга проста: спершу краща ціна, а за однакової ціни — хто прийшов раніше. І біржа дає чотири обіцянки: чесна черга, точні гроші, жодної угоди без покриття і відтворюваний день.' },
  demo: { t: '0:40', text: 'Те, що ви бачите, — не анімація: цей стакан рахує наш крейт exchange_core, скомпільований у WebAssembly. Гліб прийшов першим, але просить дорожче. Аліса й Віра просять менше, тож їхня черга перша. Богдан купує дванадцять: десять бере в Аліси, два — у Віри, бо Аліса прийшла раніше. Платить він ціну продавця: заблоковано 122,40, сплачено 120. Тепер одна й та сама заявка з різним часом життя. GTC — залишок чекає в стакані. IOC — залишок скасовано. FOK — все або нічого, і стакан навіть не змінився. Post-only — відмова, бо вона торгувала б одразу. І торгівля із собою: Аліса натрапляє на власну заявку. Між собою вони не торгують ніколи, а оператор обирає, яку з двох скасувати.',
    alt: 'Клавіша → гортає кроки, наприкінці сценарію переходить до наступного. Якщо на значку внизу написано «JS-дзеркало рушія», скажіть: «цей стакан рахує копія нашого рушія, звірена з Rust».' },
  sandbox: { t: '—', text: 'Під час виступу цей слайд можна пропустити (Shift + →). Після питань запропонуйте організаторам поставити заявку самим. Кнопка «Прогнати день ще раз» проганяє всі команди в окремому екземплярі рушія й показує той самий хеш.' },
  arch: { t: '1:40', text: 'Тепер про інженерію. Заявки приходять з багатьох потоків, а стан біржі змінює рівно один записувач — як у справжніх рушіях. Між ними наше lock-free кільце: без м’ютексів і навіть без атомарних read-modify-write, лише два release-записи на весь обмін, а курсори лежать на різних кеш-лініях. Порядок пам’яті перевіряє loom — він перебирає всі переплетення потоків. На великому кільці воно на 37% швидше за rtrb.' },
  decisions: { t: '2:10', text: 'Далі — кожна заявка є транзакцією без коду відкату: блокування, зведення й розрахунок спершу проходять на копіях стакану й рахунків, і лише потім ті самі чисті функції повторюються на справжньому стані. Гроші — на рівні типів: додати гроші до кількості чи змішати масштаби просто не скомпілюється, а округлення явне в кожній операції. І кожна зміна стану — це подія з номером без пропусків, а кожна угода в подвійному записі сходиться в нуль. Тому той самий день, прогнаний двічі, дає той самий хеш.',
    alt: 'Якщо спитають про авторство: ring і exact — окремі репозиторії тієї самої родини, exchange їх використовує.' },
  next: { t: '2:45', text: 'Чого ще немає: відновлення після перезапуску — це наш наступний пріоритет. Ще немає зміни заявки, комісій і другої торгової пари.' },
  end: { t: '2:55', text: 'Двадцять чотири крейти, жодної залежності з crates.io, триста двадцять сім тестів. Дякуємо! Будемо раді питанням.' },
};

/* ---------- deck ---------- */
const slides = [...document.querySelectorAll('.slide')];
let cur = 0;
const notesEl = document.getElementById('notes');
const notesBtn = document.getElementById('notes-btn');
const sectionsEl = document.getElementById('sections');
sectionsEl.innerHTML = slides.map((s, i) => `<button type="button" data-i="${i}"><span>${s.dataset.label}</span></button>`).join('');
sectionsEl.addEventListener('click', (e) => { const b = e.target.closest('button'); if (b) show(Number(b.dataset.i)); });

function renderNotes() {
  const id = slides[cur].dataset.id, n = NOTES[id];
  notesEl.innerHTML = n ? `<div class="nt"><div class="nt-head"><span>${slides[cur].dataset.label} · з ${n.t}</span><span>N — сховати</span></div><p>${n.text}</p>${n.alt ? `<p class="alt">${n.alt}</p>` : ''}</div>` : '';
}
function show(i) {
  cur = Math.max(0, Math.min(slides.length - 1, i));
  slides.forEach((s, j) => s.classList.toggle('is-active', j === cur));
  [...sectionsEl.children].forEach((b, j) => { b.setAttribute('aria-current', String(j === cur)); b.classList.toggle('done', j < cur); });
  document.getElementById('counter').textContent = `${cur + 1}/${slides.length}`;
  renderNotes();
  const id = slides[cur].dataset.id;
  try { history.replaceState(null, '', '#' + id); } catch (e) { /* hash is optional */ }
  requestAnimationFrame(() => {
    renderStatic();
    if (id === 'demo') demo.render(true);
    if (id === 'sandbox') sb.render(true);
  });
}
function next() {
  if (slides[cur].dataset.id === 'demo' && demo.forward()) return;
  show(cur + 1);
}
function prev() {
  if (slides[cur].dataset.id === 'demo' && demo.back()) return;
  show(cur - 1);
}
document.getElementById('next').addEventListener('click', () => show(cur + 1));
document.getElementById('prev').addEventListener('click', () => show(cur - 1));

function toggleNotes(force) {
  const on = force ?? notesEl.hidden;
  notesEl.hidden = !on;
  notesBtn.setAttribute('aria-pressed', String(on));
}
notesBtn.addEventListener('click', () => toggleNotes());
const scriptEl = document.getElementById('script');
document.getElementById('script-body').innerHTML = slides.map((s) => {
  const n = NOTES[s.dataset.id];
  return n ? `<section class="seg-t"><span class="tm">${n.t} · ${s.dataset.label}</span><p>${n.text}</p></section>` : '';
}).join('');
const toggleScript = (on = scriptEl.hidden) => { scriptEl.hidden = !on; };
document.getElementById('script-btn').addEventListener('click', () => toggleScript(true));
document.getElementById('script-close').addEventListener('click', () => toggleScript(false));

// timer
const timerEl = document.getElementById('timer');
let tStart = null, tAcc = 0, tTick = null;
function drawTimer() {
  const ms = tAcc + (tStart ? Date.now() - tStart : 0), s = Math.floor(ms / 1000);
  timerEl.textContent = `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
  timerEl.classList.toggle('over', s >= 180);
}
function toggleTimer() {
  if (tStart) { tAcc += Date.now() - tStart; tStart = null; clearInterval(tTick); }
  else { tStart = Date.now(); tTick = setInterval(drawTimer, 250); }
  drawTimer();
}
function resetTimer() { tStart = null; tAcc = 0; clearInterval(tTick); drawTimer(); }
timerEl.addEventListener('click', toggleTimer);
timerEl.addEventListener('dblclick', resetTimer);

document.addEventListener('keydown', (e) => {
  if (e.target.closest('input, select, textarea')) return;
  if ((e.key === ' ' || e.key === 'Enter') && e.target.closest('button')) return;
  if (!scriptEl.hidden) { if (e.key === 'Escape' || e.code === 'KeyS') toggleScript(false); return; }
  const k = e.key;
  if ((k === 'ArrowRight' && e.shiftKey)) { show(cur + 1); }
  else if (k === 'ArrowLeft' && e.shiftKey) { show(cur - 1); }
  else if (k === 'ArrowRight' || k === 'PageDown' || k === ' ') { if (!tStart && tAcc === 0) toggleTimer(); next(); }
  else if (k === 'ArrowLeft' || k === 'PageUp') prev();
  else if (k === 'Home') show(0);
  else if (k === 'End') show(slides.length - 1);
  else if (/^[1-5]$/.test(k) && slides[cur].dataset.id === 'demo') demo.pick(Number(k) - 1, 0, 0);
  else if (e.code === 'KeyN') toggleNotes();
  else if (e.code === 'KeyS') toggleScript(true);
  else if (e.code === 'KeyT') { if (e.shiftKey) resetTimer(); else toggleTimer(); }
  else if (e.code === 'KeyF') { (document.fullscreenElement ? document.exitFullscreen() : document.documentElement.requestFullscreen?.())?.catch?.(() => {}); }
  else if (e.key === 'Escape') toggleNotes(false);
  else return;
  e.preventDefault();
});

let resizeT = null;
window.addEventListener('resize', () => {
  clearTimeout(resizeT);
  resizeT = setTimeout(() => { renderStatic(); const id = slides[cur].dataset.id; if (id === 'demo') demo.render(true); if (id === 'sandbox') sb.render(true); }, 120);
});

/* ---------- boot ---------- */
(async () => {
  demo.init();
  sb.init();
  await bootEngine();
  demo.engine = await newEngine();
  await buildGloss();
  await sb.reset();
  const start = slides.findIndex((s) => '#' + s.dataset.id === location.hash);
  show(start >= 0 ? start : 0);
})();
})();
