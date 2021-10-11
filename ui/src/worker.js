import init, * as cw from '../../pkg/crossword';

self.onmessage = ({ data }) => {
  console.log('got message');
  init(data.wasm).then(() => {
    console.log('wasm initialized');
    cw.initThreadPool(navigator.hardwareConcurrency)
      .then(() => {
        console.log('thread pool initialized');
        let solver = cw.Solver.new(data.wordlist);
        console.log('starting to solve with', data.grid);
        let result = solver.solve(data.grid);
        postMessage(result);
      })
      .catch((err) => {
        console.log('errorred initializing thread pool on', err);
      });
  });
};
