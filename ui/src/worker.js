import init, * as cw from '../../pkg/crossword';

self.onmessage = ({ data }) => {
  console.log('[worker] message received');
  init(data.wasm).then(() => {
    console.log('initialized');
    cw.initThreadPool(navigator.hardwareConcurrency).then(() => {
      console.log('thread pool initialized');
      let solver = cw.Solver.new(data.wordlist);
      let result = solver.solve(data.grid);
      postMessage(result);
    });
  });
};
