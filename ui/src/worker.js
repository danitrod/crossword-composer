import init, * as cw from '../../pkg/crossword';

self.onmessage = ({ data }) => {
  init(data.wasm).then(() => {
    let solver = cw.Solver.new(data.wordlist);
    let result = solver.solve(data.grid);
    cw.initThreadPool(navigator.hardwareConcurrency).then(() => {
      postMessage(result);
    });
  });
};
