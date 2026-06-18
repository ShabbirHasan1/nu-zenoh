#!/usr/bin/env nuze -X0

use std/assert

let expected = {
    protocol: tcp
    address: "127.0.0.1:7447"
    metadata: {
        prio: high
        rel: "1"
    }
}

assert equal (zenoh parse locator "tcp/127.0.0.1:7447?prio=high;rel=1") $expected

let expected_empty_metadata = {
    protocol: udp
    address: "224.0.0.224:7446"
    metadata: {}
}

assert equal (zenoh parse locator "udp/224.0.0.224:7446") $expected_empty_metadata
