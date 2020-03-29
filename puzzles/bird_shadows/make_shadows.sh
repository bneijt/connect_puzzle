#!/bin/bash
for i in *_fst.png; do
    echo convert 
    convert "$i" -colorize 255 -channel RGB -negate "${i::5}_snd.png"
done
