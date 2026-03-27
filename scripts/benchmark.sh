#!/bin/bash
# Simple benchmark script for manual verification

set -e

echo "TaskGraph Performance Benchmark"
echo "==============================="
echo ""

# Create temp directory
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

# Generate test tasks
generate_tasks() {
    local count=$1
    echo "Generating $count tasks..."
    for i in $(seq 1 $count); do
        local deps=""
        if [ $i -gt 1 ]; then
            deps="depends_on: [task-$((i-1))]"
        else
            deps="depends_on: []"
        fi
        
        cat > "$TMPDIR/task-$i.md" <<EOF
---
id: task-$i
name: Task $i
status: pending
$deps
---

Description of task $i
EOF
    done
    echo "Done."
    echo ""
}

# Benchmark function
benchmark() {
    local name=$1
    shift
    echo "Benchmarking: $name"
    time cargo run --release -- -p "$TMPDIR" "$@" 2>/dev/null
    echo ""
}

# Run benchmarks for different sizes
for size in 50 500 1000; do
    echo "================================"
    echo "Testing with $size tasks"
    echo "================================"
    echo ""
    
    rm -rf "$TMPDIR"/*
    generate_tasks $size
    
    benchmark "list" list
    benchmark "validate" validate
    benchmark "topo" topo
    benchmark "cycles" cycles
    benchmark "critical" critical
    
    echo ""
done

echo "================================"
echo "Benchmark complete!"
echo "================================"
