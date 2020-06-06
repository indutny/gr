'use strict';

const MAX_RADIUS = 10000;
const STEP = 0.0000001;
const EPSILON = 1e-23;

function compute(restCentralDensity, n) {
  const GAMMA = 1 + 1 / n;

  let P = restCentralDensity ** GAMMA;
  let m = 0;

  let r;
  for (r = 0; r < MAX_RADIUS; r += STEP) {
    const rho0 = P ** (1 / GAMMA);
    const rho = rho0 + n * P;

    const dm = 4 * Math.PI * (r ** 2) * rho;

    let dP = -rho * m / Math.max(EPSILON, r ** 2);
    dP *= (1 + P / Math.max(EPSILON, rho));
    dP *= (1 + 4 * Math.PI * P * (r ** 3) / Math.max(EPSILON, m));
    dP /= Math.max(EPSILON, 1 - 2 * m / Math.max(EPSILON, r));

    m += dm * STEP;
    P += dP * STEP;

    if (P <= 0) {
      break;
    }
  }

  return { r, m, P };
}

for (let d = 0; d < 1.25; d += 0.01) {
  const rcd = (Math.sqrt(1 + 4 * d) - 1)/ 2;
  console.log(`${d},${compute(rcd, 1).m}`);
}
