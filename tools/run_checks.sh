#!/bin/bash

CHECKS=$(ls checks/check-*.js)

for CHECK in $CHECKS; do

    echo "Running $CHECK..."
    node $CHECK

done
