#!/bin/bash

perf record --call-graph dwarf target/release/t;
perf script | inferno-collapse-perf > stacks.folded;
cat stacks.folded | inferno-flamegraph > flamegraph.svg;

echo "Finish";
