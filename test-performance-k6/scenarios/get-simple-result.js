import http from 'k6/http';
import { sleep } from 'k6';
import { Trend, Rate, Counter } from 'k6/metrics';
import { check, fail } from 'k6';

export let BitonicDuration = new Trend('bitonic_duration');
export let BitonicFailRate = new Rate('bitonic_fail_rate');
export let BitonicSuccessRate = new Rate('bitonic_success_rate');
export let BitonicReqs = new Rate('bitonic_reqs');

export function GetSimpleResult() {
    const url = 'http://localhost:3000/bitonic?n=5&l=3&r=10';
    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    let res = http.get(url, params);
    BitonicDuration.add(res.timings.duration);
    BitonicReqs.add(1);
    BitonicFailRate.add(res.status !== 200);
    BitonicSuccessRate.add(res.status === 200);

    if(!check(res, {
        'is status 200': (r) => r.status === 200,
    })){
        fail('GetSimpleResult failed');
    };

    sleep(1);
} 
