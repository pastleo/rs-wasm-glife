import { greet, Game } from "./wasm";

//greet();

const table = document.getElementById('universe');
const h1 = document.getElementById('h1');
const sizeStyle = document.getElementById('size-style');

let height = 80;
let width = 80;
let running, speed, game, cells;

const initTable = () => {
  cells = [];
  table.innerHTML = '';
  Array.from(Array(height)).forEach((_, y) => {
    const tr = table.insertRow(y);
    Array.from(Array(width)).forEach((_, i) => {
      cells.push(tr.insertCell(i));
    });
  });

}

// original toString implementation:
//initTable();
//const renderLoop = () => {
  //let i = 0;
  //Array.from(game.toString()).forEach(c => {
    //if(c === 'A') {
      //cells[i].classList.add('alive');
      //i++;
    //} else if(c === '_') {
      //cells[i].classList.remove('alive');
      //i++;
    //}
  //});
  //game.tick();
  //speed++;

  //requestAnimationFrame(renderLoop);
//}
//renderLoop();

const render = () => {
  cells.forEach((c, i) => {
    if(game.isChanged(i)) {
      c.classList.toggle('alive');
    }
  })
}

const init = () => {
  game = Game.new(height, width);
  running = false;
  speed = 0;

  initTable();
  render();
}

const calSpeed = () => {
  if(running) {
    h1.innerText = `speed: ${speed}`;
    speed = 0;
  } else {
    h1.innerText = `paused`;
  }
}
setInterval(calSpeed, 1000);

const renderLoop = () => {
  if(!running) { return }
  game.tick();
  render();
  speed++;

  requestAnimationFrame(renderLoop);
}

document.getElementById('run-stop').onclick = () => {
  running = !running;
  renderLoop();
}

document.getElementById('reset').onclick = () => {
  running = false;
  height = parseInt(document.getElementById('height').value) || 80;
  width = parseInt(document.getElementById('width').value) || 80;
  const cellSize = parseInt(document.getElementById('cell-size').value) || 10;
  document.getElementById('size-style').innerText = `
    table#universe tr td { width: ${cellSize}px; height: ${cellSize}px; }
  `
  init();
}

table.onmousedown = ({ target }) => {
  if(target.tagName === 'TD') {
    target.classList.toggle('alive');
    game.toggle(
      target.parentElement.rowIndex * width + target.cellIndex
    )
  }
}

init();
