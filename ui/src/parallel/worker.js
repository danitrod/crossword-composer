import init, { parSolve } from '../../../pkg/crossword';

self.onmessage = ({ data }) => {
  init(data.wasm).then(() => {
    let result = parSolve(data.wordlist, data.grid, data.id, data.numThreads);
    postMessage(result);
  });
};
