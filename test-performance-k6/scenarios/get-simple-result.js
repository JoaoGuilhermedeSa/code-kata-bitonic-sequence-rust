import http from 'k6/http';
import { sleep } from 'k6';
import { Trend, Rate, Counter } from 'k6/metrics';
import { check, fail } from 'k6';

export let GetCustomerDuration = new Trend('get_customer_duration');
export let GetCustomerFailRate = new Rate('get_customer_fail_rate');
export let GetCustomerSuccessRate = new Rate('get_customer_success_rate');
export let GetCustomerReqs = new Rate('get_customer_reqs');

export default function GetSimpleResult() {
    const url = 'http://localhost:8000/bitonic?numbers=1,3,8,12,4,2';
    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    let res = http.get(url, params);
    GetCustomerDuration.add(res.timings.duration);
    GetCustomerReqs.add(1);
