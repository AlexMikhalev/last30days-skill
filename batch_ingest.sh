#!/bin/bash
set -e

PREFIXES=(
    "sciences-core/agents"
    "sciences-core/algorithms"
    "sciences-core/cdma and gsm and umts"
    "sciences-core/chaos and fractals"
    "sciences-core/coding & error correction"
    "sciences-core/complexity and information"
    "sciences-core/compression"
    "sciences-core/control"
    "sciences-core/crlb"
    "sciences-core/cryptology"
    "sciences-core/discrete math"
    "sciences-core/doppler"
    "sciences-core/dsp"
    "sciences-core/fading"
    "sciences-core/feature_extraction"
    "sciences-core/frequency"
    "sciences-core/fuzzy NN ai"
    "sciences-core/genetic algorithms"
    "sciences-core/hough"
    "sciences-core/kalman filter and estimation theory"
    "sciences-core/math reference"
    "sciences-core/monte-carlo & bootstrap"
    "sciences-core/networking"
    "sciences-core/non-bayesian"
    "sciences-core/optical flow"
    "sciences-core/optimisation"
    "sciences-core/particle filter"
    "sciences-core/probability and stats"
    "sciences-core/propagation"
    "sciences-core/radon"
    "sciences-core/recognition and clustering"
    "sciences-core/segmentation"
    "sciences-core/svm_packet_classificaion"
    "sciences-core/svm_text"
    "sciences-core/symmetry"
    "sciences-core/timing"
    "sciences-core/tracking"
    "sciences-core/wavelets"
    "sciences-core/wireless communication"
)

cd /home/alex/clawd/ingestor

echo "Starting batch ingest at $(date)"
echo "Total prefixes to process: ${#PREFIXES[@]}"

for i in "${!PREFIXES[@]}"; do
    prefix="${PREFIXES[$i]}"
    num=$((i + 1))
    echo ""
    echo "[$num/${#PREFIXES[@]}] Processing: $prefix"
    uv run ingestor --config config.yaml run --mode filer --bucket books --prefix "$prefix" --max-files 1000
done

echo ""
echo "Batch ingest completed at $(date)"
