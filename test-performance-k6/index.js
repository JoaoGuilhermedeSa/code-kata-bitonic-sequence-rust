import { GetSimpleResult } from "./scenarios/get-simple-result.js";
import { GetRandomResult } from "./scenarios/get-random-inputs-result.js";
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

export function handleSummary(data) {
  return {
    "summary.html": htmlReport(data, {
      title: "Bitonic Sequence API Performance Test",
      theme: "bootswatch:darkly",
    }),
    stdout: textSummary(data, { indent: " ", enableColors: true }),
  };
}

export const options = {
  scenarios: {
    simple_test: {
      executor: 'per-vu-iterations',
      exec: 'simple',
      vus: 1,
      iterations: 1,
      maxDuration: '10s',
    },
    random_test: {
        executor: 'ramping-vus',
        exec: 'random',
        startVUs: 0,
        stages: [
            { duration: '5s', target: 0 }, // Warmup
            { duration: '15s', target: 1200 }, // Gradual ramp
            { duration: '30s', target: 1200 }, // Plateau
            { duration: '45s', target: 2400 }, // Step up
            { duration: '1m', target: 2400 }, // High load
        ],
        gracefulRampDown: '30s',
    },
  },
  thresholds: {
    'http_req_duration': ['p(95)<2000', 'p(99)<5000'],
    'http_req_failed': ['rate<0.01'],
  },
};

export function simple() {
  GetSimpleResult();
}

export function random() {
  GetRandomResult();
}
