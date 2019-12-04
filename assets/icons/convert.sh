#!/bin/bash

convert $1 -resize $2x$2 -quality 100 $1
