const Player = require('..');
const player = new Player();

player.open('./fixtures/SampleAudio_0.4mb.mp3');
player.on('end', () => {
  console.log('end');
});

setTimeout(() => {
  console.log('play');
  player.play();
}, 2000);

setTimeout(() => {
  console.log('pause');
  player.pause();
}, 4000);

setTimeout(() => {
  console.log('play');
  player.play();
}, 6000);

setTimeout(() => {
  console.log('stop');
  player.stop();
}, 8000);
