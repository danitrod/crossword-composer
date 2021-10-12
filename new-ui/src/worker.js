import * as Comlink from 'comlink';

console.log('[worker] hello');

const mod = {
  counter: 0,
  test(a, b) {
    this.counter++;
    console.log(this.counter);
    return a + b;
  },
};

Comlink.expose(mod);
