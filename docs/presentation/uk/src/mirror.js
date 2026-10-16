// JS port of engine/src/lib.rs's protocol, used only when WebAssembly is unavailable.
// mirror_test.cjs holds it to the wasm build answer for answer.
// Money in integer cents, quantities in whole units (tick 0.01, lot 1).
function createJsEngine() {
  let S = null;

  const fmt = (c) => {
    const neg = c < 0, a = Math.abs(c), w = Math.floor(a / 100), f = a % 100;
    const s = f === 0 ? String(w) : f % 10 === 0 ? `${w}.${f / 10}` : `${w}.${String(f).padStart(2, '0')}`;
    return neg ? '-' + s : s;
  };
  const cents = (t) => {
    if (!/^-?\d+(\.\d+)?$/.test(t)) return null;
    const neg = t.startsWith('-'), [w, f = ''] = t.replace('-', '').split('.');
    const frac = f.replace(/0+$/, '');
    if (frac.length > 2) return { offTick: true, neg };
    const v = Number(w) * 100 + Number((frac + '00').slice(0, 2));
    return { v: neg ? -v : v, neg };
  };
  const units = (t) => {
    if (!/^\d+(\.\d+)?$/.test(t)) return null;
    const [w, f = ''] = t.split('.');
    if (/[1-9]/.test(f)) return { offLot: true };
    return { v: Number(w) };
  };

  function reset(args) {
    if (args.length !== 8) return 'reset takes cash and apples for four accounts';
    const accounts = {};
    for (let n = 0; n < 4; n++) {
      const c = cents(args[2 * n]), q = units(args[2 * n + 1]);
      if (!c || c.offTick || !q || q.offLot) return 'bad balance';
      accounts[n + 1] = { cashFree: c.v, cashLocked: 0, applesFree: q.v, applesLocked: 0 };
    }
    S = { accounts, sell: [], buy: [], resv: new Map(), events: [], seen: 0, seq: 0, nextId: 0, halted: false,
      stats: { rests: 0, fills: 0, rejects: 0, cancels: 0 } };
    return null;
  }

  const emit = (order, acct, kind) => S.events.push({ seq: S.seq++, order, acct, ...kind });
  // A side's levels, best first: [{price, orders:[{id, acct, qty, total, tif}]}]
  const crosses = (side, limit, rest) => side === 'buy' ? rest <= limit : rest >= limit;
  const opposite = (side) => side === 'buy' ? 'sell' : 'buy';
  const clone = (levels) => levels.map((l) => ({ price: l.price, orders: l.orders.map((o) => ({ ...o })) }));

  function insert(side, price, order) {
    const levels = S[side];
    const better = (a, b) => side === 'buy' ? a > b : a < b;
    let i = 0;
    while (i < levels.length && better(levels[i].price, price)) i++;
    if (i < levels.length && levels[i].price === price) levels[i].orders.push(order);
    else levels.splice(i, 0, { price, orders: [order] });
  }
  function removeFront(levels) {
    levels[0].orders.shift();
    if (!levels[0].orders.length) levels.shift();
  }

  // The crossing loop on `levels` (mutated). Returns {steps, remaining}.
  function crossInner(levels, inc, policy) {
    let remaining = inc.qty;
    const steps = [];
    while (remaining > 0 && levels.length) {
      const best = levels[0].orders[0], price = levels[0].price;
      if (!crosses(inc.side, inc.price, price)) break;
      if (best.acct === inc.acct) {
        if (policy === 'resting') { removeFront(levels); steps.push({ cancel: best.id, acct: best.acct, qty: best.qty }); continue; }
        if (policy === 'incoming') { steps.push({ cancel: inc.id, acct: inc.acct, qty: remaining }); break; }
        removeFront(levels);
        steps.push({ cancel: best.id, acct: best.acct, qty: best.qty });
        steps.push({ cancel: inc.id, acct: inc.acct, qty: remaining });
        break;
      }
      const taken = Math.min(remaining, best.qty);
      steps.push({ trade: true, maker: best.id, makerAcct: best.acct, price, qty: taken });
      best.qty -= taken;
      if (best.qty === 0) removeFront(levels);
      remaining -= taken;
    }
    return { steps, remaining };
  }

  function obligationOf(r) {
    return r.side === 'buy' ? { cash: fmt(r.limit * r.unfilled) } : { apples: String(r.unfilled) };
  }
  function release(id) {
    const r = S.resv.get(id), a = S.accounts[r.acct];
    if (r.side === 'buy') { const amt = r.limit * r.unfilled; a.cashLocked -= amt; a.cashFree += amt; }
    else { a.applesLocked -= r.unfilled; a.applesFree += r.unfilled; }
    S.resv.delete(id);
    return obligationOf(r);
  }
  function settle(inc, t) {
    const taker = S.resv.get(inc.id), maker = S.resv.get(t.maker);
    const [buy, sell] = inc.side === 'buy' ? [taker, maker] : [maker, taker];
    const buyer = S.accounts[buy.acct], seller = S.accounts[sell.acct];
    const paid = t.price * t.qty, freed = buy.limit * t.qty;
    buyer.cashLocked -= freed; buyer.cashFree += freed - paid; buyer.applesFree += t.qty;
    seller.applesLocked -= t.qty; seller.cashFree += paid;
    for (const r of [buy, sell]) { r.unfilled -= t.qty; }
    for (const [id, r] of [[inc.id, taker], [t.maker, maker]]) if (r.unfilled === 0) S.resv.delete(id);
  }

  function reject(id, acct, reason) {
    S.stats.rejects++;
    emit(id, acct, { kind: 'rejected', reason });
    return { kind: 'rejected', reason };
  }

  function place(w) {
    const [, acctT, side, priceT, qtyT, tif, policy] = w;
    const acct = Number(acctT);
    if (!(acct >= 1 && acct <= 4 && /^\d+$/.test(acctT))) return { err: `bad account: ${acctT}` };
    if (side !== 'buy' && side !== 'sell') return { err: `bad side: ${side}` };
    if (!['gtc', 'ioc', 'fok', 'post'].includes(tif)) return { err: `bad tif: ${tif}` };
    if (!['resting', 'incoming', 'both'].includes(policy)) return { err: `bad stp: ${policy}` };
    const p = cents(priceT), q = units(qtyT);
    if (!p) return { err: 'price' };
    if (!q) return { err: 'qty' };
    const id = S.nextId++;
    if (!q.offLot && q.v === 0) return reject(id, acct, 'ZeroQuantity');
    if (p.neg && (p.offTick || p.v < 0)) return reject(id, acct, 'NegativePrice');
    if (S.halted) return reject(id, acct, 'Halted');
    if (p.offTick) return reject(id, acct, 'PriceOffTick');
    if (q.offLot) return reject(id, acct, 'QuantityOffLot');
    const inc = { id, acct, side, price: p.v, qty: q.v, tif };
    const opp = S[opposite(side)];
    if (tif === 'post' && opp.length && crosses(side, inc.price, opp[0].price)) return reject(id, acct, 'PostOnlyWouldTake');
    const a = S.accounts[acct];
    if (side === 'buy' ? a.cashFree < inc.price * inc.qty : a.applesFree < inc.qty) return reject(id, acct, 'InsufficientFunds');

    // Reserve, then cross (FOK probes a clone first).
    if (side === 'buy') { a.cashFree -= inc.price * inc.qty; a.cashLocked += inc.price * inc.qty; }
    else { a.applesFree -= inc.qty; a.applesLocked += inc.qty; }
    S.resv.set(id, { acct, side, limit: inc.price, unfilled: inc.qty });
    emit(id, acct, { kind: 'accepted', side, price: fmt(inc.price), qty: String(inc.qty),
      reserved: side === 'buy' ? { cash: fmt(inc.price * inc.qty) } : { apples: String(inc.qty) } });

    let crossing;
    if (tif === 'fok' && crossInner(clone(opp), inc, policy).remaining !== 0) crossing = { steps: [], remaining: inc.qty };
    else crossing = crossInner(opp, inc, policy);

    let trades = 0, incCancelled = false;
    for (const s of crossing.steps) {
      if (s.trade) {
        trades++; S.stats.fills++;
        settle(inc, s);
        emit(id, acct, { kind: 'trade', maker: s.maker, maker_acct: s.makerAcct, taker_side: side, price: fmt(s.price), qty: String(s.qty) });
      } else {
        const released = release(s.cancel);
        emit(s.cancel, s.acct, { kind: 'cancelled', cause: 'self_match', qty: String(s.qty), released });
        if (s.cancel === id) incCancelled = true; else S.stats.cancels++;
      }
    }
    const tifDropped = !incCancelled && (tif === 'ioc' || tif === 'fok') && crossing.remaining > 0;
    if (tifDropped) {
      const released = release(id);
      emit(id, acct, { kind: 'cancelled', cause: 'tif', qty: String(crossing.remaining), released });
    }
    const resting = incCancelled || tifDropped ? 0 : crossing.remaining;
    if (resting > 0) {
      insert(side, inc.price, { id, acct, qty: resting, total: inc.qty, tif });
      S.stats.rests++;
    }
    return { kind: 'placed', order: id, trades, resting: String(resting), stp: incCancelled, tif_dropped: tifDropped };
  }

  function cancel(w) {
    const acct = Number(w[1]), id = Number(w[2]);
    if (!(acct >= 1 && acct <= 4 && /^\d+$/.test(w[1]))) return { err: `bad account: ${w[1]}` };
    if (!/^\d+$/.test(w[2])) return { err: `bad order id: ${w[2]}` };
    for (const side of ['sell', 'buy']) {
      for (const [li, level] of S[side].entries()) {
        const oi = level.orders.findIndex((o) => o.id === id && o.acct === acct);
        if (oi < 0) continue;
        const o = level.orders[oi];
        const released = release(id);
        level.orders.splice(oi, 1);
        if (!level.orders.length) S[side].splice(li, 1);
        emit(id, acct, { kind: 'cancelled', cause: 'request', qty: String(o.qty), released });
        S.stats.cancels++;
        return { kind: 'cancelled', qty: String(o.qty) };
      }
    }
    return { kind: 'not_resting' };
  }

  function fnv(text) {
    let h = 0xcbf29ce484222325n;
    for (const b of new TextEncoder().encode(text)) { h ^= BigInt(b); h = (h * 0x100000001b3n) & 0xffffffffffffffffn; }
    return '0x' + h.toString(16).padStart(16, '0');
  }

  function answer(result) {
    const events = S.events.slice(S.seen);
    S.seen = S.events.length;
    const side = (levels) => levels.map((l) => ({ price: fmt(l.price),
      orders: l.orders.map((o) => ({ id: o.id, acct: o.acct, qty: String(o.qty), total: String(o.total), tif: o.tif })) }));
    const accounts = [1, 2, 3, 4].map((n) => {
      const a = S.accounts[n];
      return { id: n, cash_free: fmt(a.cashFree), cash_locked: fmt(a.cashLocked), apples_free: String(a.applesFree), apples_locked: String(a.applesLocked) };
    });
    return { ok: true, result, events, book: { sell: side(S.sell), buy: side(S.buy) }, accounts, conserved: true, halted: S.halted, stats: { ...S.stats } };
  }

  function call(command) {
    const w = command.trim().split(/\s+/);
    if (w[0] === 'reset') {
      const e = reset(w.slice(1));
      return e ? { ok: false, error: e } : answer({ kind: 'reset' });
    }
    if (!S) return { ok: false, error: 'reset first' };
    let r;
    if (w[0] === 'place' && w.length === 7) r = place(w);
    else if (w[0] === 'cancel' && w.length === 3) r = cancel(w);
    else if (w[0] === 'halt' && w.length === 1) r = S.halted ? { err: 'already halted' } : (S.halted = true, { kind: 'halted' });
    else if (w[0] === 'resume' && w.length === 1) r = !S.halted ? { err: 'not halted' } : (S.halted = false, { kind: 'resumed' });
    else if (w[0] === 'state' && w.length === 1) r = { kind: 'state' };
    else if (w[0] === 'hash' && w.length === 1) r = { kind: 'hash', hash: fnv(JSON.stringify(S.events)) };
    else r = { err: `unknown command: ${command}` };
    return r.err ? { ok: false, error: r.err } : answer(r);
  }

  return { call, kind: 'js' };
}

if (typeof module !== 'undefined') module.exports = { createJsEngine };
