import { greet, Universe } from "./wasm";

greet();
const universe = Universe.new_square(10, 10);
const pre = document.getElementById('universe');

const renderLoop = () => {
  pre.innerText = universe.render();
  universe.tick();

  //requestAnimationFrame(renderLoop);
}

setInterval(renderLoop, 1000);
//renderLoop();
