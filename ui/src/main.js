import Crossword from './Crossword.svelte';

console.log('creating crossword', document);

new Crossword({
  target: document.getElementById('app-root'),
  props: {},
});
