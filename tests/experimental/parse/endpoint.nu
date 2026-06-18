#!/usr/bin/env nuze -X0

use std/assert

let expected = {
    protocol: tcp
    address: "127.0.0.1:7447"
    metadata: {
        rel: "1"
    }
    config: {
        connect_timeout: "3s"
        iface: lo0
    }
}

assert equal (zenoh parse endpoint "tcp/127.0.0.1:7447?rel=1#connect_timeout=3s;iface=lo0") $expected

let expected_empty_metadata_and_config = {
    protocol: udp
    address: "224.0.0.224:7446"
    metadata: {}
    config: {}
}

assert equal (zenoh parse endpoint "udp/224.0.0.224:7446") $expected_empty_metadata_and_config
