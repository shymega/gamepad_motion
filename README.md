# gamepad_motion

Rust bindings for the [GamepadMotionHelpers](https://github.com/JibbSmart/GamepadMotionHelpers/tree/main) C++ library.

## Usage
```rust
use gamepad_motion::GamepadMotion;

let mut gm = GamepadMotion::new();
loop {
	// read gyro, accelerometer, elapsed time...
	xy = gm.process(g, a, dt).gyro_player_space(None);
	// update position using xy...
}
```
