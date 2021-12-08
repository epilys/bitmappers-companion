#!/bin/sh
convert $1  -density 38 -units PixelsPerCentimeter $1
optipng -keep -o7 -zm1-9 $1
