#!/bin/bash
babel -w *.js --out-dir compiled --presets es2015 --plugins transform-es2015-modules-amd --source-maps
