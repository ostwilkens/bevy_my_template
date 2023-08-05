@echo off
FOR %%A IN (%*) DO ffmpeg -y -i "%%A" -q:v 75 "%%~nA.webp"
