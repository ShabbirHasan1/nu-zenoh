#!/usr/bin/env nuze -X0

use std/assert

let expected = {
    keyexpr: "demo/**"
    parameters: [
        [key value];
        [_anyke ""]
        [limit "10"]
        [format json]
    ]
}

assert equal (zenoh parse selector "demo/**?_anyke;limit=10;format=json") $expected

let expected_duplicate_parameters = {
    keyexpr: "demo/**"
    parameters: [
        [key value];
        [tag a]
        [tag b]
        [limit "10"]
    ]
}

assert equal (zenoh parse selector "demo/**?tag=a;tag=b;limit=10") $expected_duplicate_parameters

let expected_empty_parameters = {
    keyexpr: "demo/**"
    parameters: []
}

assert equal (zenoh parse selector "demo/**") $expected_empty_parameters
