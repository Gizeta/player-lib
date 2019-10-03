const { Player } = require('../native');
const EventEmitter = require('events');

class Wrapper extends EventEmitter {
  constructor() {
    super();
    this.player = new Player();
  }

  open(path) {
    this.player.open(path, () => {
      this.emit('end');
    });
  }

  play() {
    this.player.play();
  }

  pause() {
    this.player.pause();
  }

  stop() {
    this.player.stop();
  }
}

module.exports = Wrapper;
