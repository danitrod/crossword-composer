import init, * as cw from '../../pkg/crossword';

self.onmessage = ({ data }) => {
  init(data.wasm).then(() => {
    cw.initThreadPool(4).then(() => {
      let solver = cw.Solver.new(data.wordlist);
      let result = solver.solve(data.grid);
      postMessage(result);
    });
  });
};
