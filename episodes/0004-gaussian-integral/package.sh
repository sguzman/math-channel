#!/usr/bin/env bash
set -euo pipefail
# Original procedural ambient score; no samples, third-party tracks or voice.
v=out/example-0004-visual.mp4
duration=$(ffprobe -v error -show_entries format=duration -of csv=p=0 "$v")
fade=$(awk -v d="$duration" 'BEGIN{printf "%.3f",d-3}')
# Sustained additive harmonics with slow amplitude motion, gentle stereo separation.
ffmpeg -hide_banner -loglevel error -y -i "$v" -f lavfi -i "aevalsrc=0.030*sin(2*PI*130.8128*t)*(0.8+0.2*sin(2*PI*0.09*t))+0.020*sin(2*PI*196*t)*(0.8+0.2*sin(2*PI*0.13*t))+0.012*sin(2*PI*261.6256*t)|0.028*sin(2*PI*130.8128*t)*(0.8+0.2*sin(2*PI*0.11*t))+0.019*sin(2*PI*195.7*t)+0.012*sin(2*PI*329.6276*t):s=48000:d=$duration" -map 0:v -map 1:a -c:v copy -af "afade=t=in:d=2,afade=t=out:st=$fade:d=3" -c:a aac -b:a 192k -shortest -movflags +faststart out/example-0004.mp4
ffprobe -v error -show_format -show_streams out/example-0004.mp4 > out/example-0004-probe.txt
ffmpeg -v error -xerror -i out/example-0004.mp4 -f null -
ffmpeg -v error -y -i out/example-0004.mp4 -vf 'fps=1/4,scale=320:-2,tile=5x6:padding=4:margin=4' -frames:v 1 out/example-0004-storyboard.png
# Hero frame is sampled from the final theorem hold, before the closing fade.
hero=$(awk -v d="$duration" 'BEGIN{printf "%.3f",d-3.5}')
ffmpeg -v error -y -ss "$hero" -i out/example-0004.mp4 -frames:v 1 -vf scale=1280:720 out/example-0004-thumbnail.png
ffmpeg -v error -y -i out/example-0004.mp4 -vf scale=1280:720 -r 30 -c:v libx264 -preset fast -crf 21 -pix_fmt yuv420p -c:a aac -b:a 128k -movflags +faststart out/example-0004-preview.mp4
# Captions are on-screen exposition cues, not a claimed spoken transcript.
cp episodes/0004-gaussian-integral/captions.srt out/example-0004-captions.srt
cp episodes/0004-gaussian-integral/upload.md out/example-0004-upload.md
