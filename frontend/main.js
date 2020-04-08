import init, { run_app } from './pkg/p2pc.js';
async function main() {
   await init('/app/p2pc_bg.wasm');
   run_app();
}
main();
