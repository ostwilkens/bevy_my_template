# bevy_my_template

## version of wasm-server-runner which uses local index.html
`cargo install --git https://github.com/squ1dd13/wasm-server-runner/ wasm-server-runner`

## trunk version that supports polling on windows
`cargo install --git https://github.com/ctron/trunk --branch feature/add_poll_1 trunk`
`trunk serve --poll --poll-interval 1s --address 0.0.0.0`

todo:
- single-file windows builds
- offline web build deploy
- click sound effect
- hover sound effect
- basic sound effects (click, blip etc)
- score resource
- mute button
- handle resource
- disable play button for 1s after entering menu to prevent accidental click
- wav -> ogg(6) script
- fix your timestep: "you maintain the FixedUpdate calculations as canonical and on the normal update schedule you LERP using the accumulated() method on the FixedTime resource from the FixedUpdate calculations"
- my own font
- online highscore
