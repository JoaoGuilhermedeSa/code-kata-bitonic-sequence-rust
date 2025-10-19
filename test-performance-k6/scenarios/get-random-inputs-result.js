import http from 'k6/http';
import { check, sleep } from 'k6';
import { randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';
import { Trend, Rate, Counter } from 'k6/metrics';

export let RandomTestDuration = new Trend('random_test_duration');
export let RandomTestFailRate = new Rate('random_test_fail_rate');
export let RandomTestSuccessRate = new Rate('random_test_success_rate');
export let RandomTestReqs = new Counter('random_test_reqs');

export function GetRandomResult() {
  const l = randomIntBetween(1, 499);
  const r = randomIntBetween(l + 1, 500);
  const n = randomIntBetween(5, 40);

  const res = http.get(`http://127.0.0.1:3000/bitonic?n=${n}&l=${l}&r=${r}`, {
    headers: {
      'Accept': 'application/json',
    },
    tags: {
      name: '/bitonic',
    },
  });

  RandomTestDuration.add(res.timings.duration);
  RandomTestReqs.add(1);
  RandomTestFailRate.add(res.status !== 200);
  RandomTestSuccessRate.add(res.status === 200);

  check(res, {
    'status is 200': (r) => r.status === 200,
    'response body is present': (r) => r.body.length > 0,
  });

  sleep(randomIntBetween(0.05, 0.2)); // Think time
}