import * as Comlink from 'comlink';

console.log('hello');

async function init() {
  console.log('initing');
  const worker = Comlink.wrap(
    new Worker(new URL('./worker.js', import.meta.url), {
      type: 'module',
    })
  );

  console.log('counter', await worker.counter);

  console.log(await worker.test(13, 31));

  console.log(worker);
}

init();
