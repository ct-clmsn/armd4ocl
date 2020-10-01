//  Copyright (c) 2020 Christopher Taylor
//
//  SPDX-License-Identifier: BSL-1.0
//  Distributed under the Boost Software License, Version 1.0. (See accompanying
//  file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
use armd4ocl::opencl_str;

#[test]
fn test_macro() {

    let script : &str = opencl_str! {
            int a;
            int b = 0;
            b = a + b;
        
            double e;
            e >>= 1;
            e++;
            ++e;
            e--;
            --e;
            e >> 1;

            double c;
            c = 0;

            local float c = 1;
            static double x = 0;
            global double d = 0;
            double * z = &d;

            z[0] = 1;
            z = 1;

            double a[10];
            double a[10+11];
            double a[a+b];

            double d = a + b;
            double e = a & b;
            e.x = e.x + 1;
            e.x += 1;
            double f = a & b + 1;
            e >> 1;
            e >>= 1;

            e.x <<= 1;
            e.w >>= 1;

            double e = (a / b) + a + (d / e) + ((a* (b + e) ) % e);

            for_(a = 1; a < 10; a+=1) {
                a = 1;
            }

            for_(a = 1; a < 10; a+=1) {
                a = 1;
                a = b + c;
                a = z[a] + c;
                a = z[0] + c;
                a.x = c;
            }

            for_(int a = 1; a < 10; a+=1) {
                a = 1;
            }

            for_(uint a = 1; a < 10; a+=1) {
                a = 1;
            }

            for_(uint a = 1; a < 10; a++) {
                a = 1;
            }

            for_(uint a = 1; a < 10; ++a) {
                a = 1;
            }

            for_(uint a = 1; a < 10; a<<=1) {
                a = 1;
            }

            for_(int a = 10; a > 0; a-=1) {
                a = 1;
            }

            for_(uint a = 10; a > 0; a-=1) {
                a = 1;
            }

            for_(uint a = 10; a > 0; a--) {
                a = 1;
            }

            for_(uint a = 10; a > 0; --a) {
                a = 1;
            }

            for_(uint a = 10; a > 0; a>>=1) {
                a = 1;
            }

            if_(a < b) {
                a = b;
            }

            if_(a < b) {
                a = b;
            }
            else_ {
                b = a;
            }

            if_(a < b) {
                a = b;
            }
            else_ {
                b = a;
            }
            else_ {
                b = a;
            }

            if_(a < b) {
                a = b;
            }
            else_if_(b > a) {
                b = a;
            }

            if_(a < b) {
                a = b;
            }
            else_if_(b > a) {
                b = a;
            }
            else_if_(b > a) {
                b = a;
            }

            while_(1) {
                a+=1;
            }

            while_(a == b) {
                a+=1;
            }

            uint8 f = (uint8)(1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0);
            double8 f = (double8)(1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0);
            double16 f = (double16)(1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0);

            float4 f = (float4)(1.0, 2.0, 3.0, 4.0);
            float3 f = (float3)(1.0, 2.0, 3.0);
            float2 f = (float2)(1.0, 2.0);

            union { float f; } a;
            union { float f; int c; } a;

            kernel void car(int a) {
                a += 1;
            }

            kernel void car(int a, int b) {
                a += 1;
            }
    };

    println!("{}", script);
}
