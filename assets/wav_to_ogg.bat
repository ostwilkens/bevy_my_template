@echo off
FOR %%A IN (%*) DO ffmpeg -i "%%A" -q:a 6 "%%~nA.ogg"
