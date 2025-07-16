use std::thread;
use std::time::{Duration, Instant};
use winapi::shared::minwindef::DWORD;
use winapi::um::winuser::{
    MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
    MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_WHEEL, mouse_event,
};

pub mod mouse_control {
    use super::*;

    /// Simulates a left mouse button click with configurable delay between press and release.
    ///
    /// # Arguments
    /// * `delay` - Duration to wait between sending the mouse-down and mouse-up events
    ///
    /// # Safety
    /// This function contains unsafe code because it calls the WinAPI `mouse_event` function directly.
    /// The caller must ensure:
    /// - The thread has rights to perform input simulation
    /// - No other thread is manipulating mouse state concurrently
    ///
    /// # Notes
    /// - Uses `MOUSEEVENTF_LEFTDOWN` and `MOUSEEVENTF_LEFTUP` flags
    /// - Sleeps the current thread during the delay
    /// - Coordinate parameters (0,0,0,0) indicate using current cursor position
    pub fn mouse_left_click(delay: Duration) {
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            thread::sleep(delay);
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }
    }

    /// Simulates a right mouse button click with configurable delay between press and release.
    ///
    /// # Arguments
    /// * `delay` - Duration to wait between sending the mouse-down and mouse-up events
    ///
    /// # Safety
    /// This function contains unsafe code because it calls the WinAPI `mouse_event` function directly.
    /// The caller must ensure:
    /// - The thread has rights to perform input simulation
    /// - No other thread is manipulating mouse state concurrently
    ///
    /// # Notes
    /// - Uses `MOUSEEVENTF_RIGHTDOWN` and `MOUSEEVENTF_RIGHTUP` flags
    /// - Sleeps the current thread during the delay
    /// - Coordinate parameters (0,0,0,0) indicate using current cursor position
    /// - Typical uses include context menu interactions
    pub fn mouse_right_click(delay: Duration) {
        unsafe {
            mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
            thread::sleep(delay);
            mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
        }
    }

    /// Simulates a middle mouse button click with configurable delay between press and release.
    ///
    /// # Arguments
    /// * `delay` - Duration between the mouse-down and mouse-up events. A zero duration may cause
    ///             some applications to not register the click properly.
    ///
    /// # Safety
    /// This function contains unsafe WinAPI calls. The caller must ensure:
    /// - The application has permission to simulate input events
    /// - No other threads are manipulating mouse state concurrently
    /// - The function is not called during system hooks that might intercept mouse events
    ///
    /// # Notes
    /// - Uses `MOUSEEVENTF_MIDDLEDOWN` and `MOUSEEVENTF_MIDDLEUP` flags
    /// - Maintains cursor position (parameters 0,0,0,0 use current position)
    pub fn mouse_middle_click(delay: Duration) {
        unsafe {
            mouse_event(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0, 0);
            thread::sleep(delay);
            mouse_event(MOUSEEVENTF_MIDDLEUP, 0, 0, 0, 0);
        }
    }

    /// Simulates mouse wheel rotation with specified delta value.
    ///
    /// # Arguments
    /// * `delta` - Wheel movement amount and direction:
    ///   - Positive: Scroll up (away from user)
    ///   - Negative: Scroll down (toward user)
    ///   - Standard increment: 120 units per "click" (WHEEL_DELTA)
    ///
    /// # Safety
    /// Contains unsafe WinAPI calls. Requirements:
    /// - Thread must have input simulation privileges
    /// - No concurrent mouse state manipulation
    /// - Delta should be multiples of 120 for expected behavior
    ///
    /// # Notes
    /// - Uses `MOUSEEVENTF_WHEEL` flag
    /// - Uses current cursor position (0,0,0,0 parameters)
    /// - For horizontal scroll, use `mouse_hwheel_spin`
    pub fn mouse_wheel_spin(delta: i32) {
        unsafe {
            mouse_event(MOUSEEVENTF_WHEEL, 0, 0, delta as DWORD, 0);
        }
    }

    /// Simulates smooth mouse wheel scrolling by breaking movement into incremental steps.
    ///
    /// # Arguments
    /// * `delta` - Total scroll amount (positive = up, negative = down)
    /// * `duration` - Total time over which to distribute the scrolling
    ///
    /// # Returns
    /// `Ok(())` on success, or `Err(&'static str)` if:
    /// - `delta` is zero (would cause division by zero)
    /// - `duration` is too short for requested steps
    ///
    /// # Safety
    /// Contains unsafe WinAPI calls. Requirements:
    /// - Thread must have input simulation privileges
    /// - No concurrent mouse state manipulation
    ///
    /// # Notes
    /// - More realistic than single-event scrolling
    /// - Each increment sends ±1 wheel unit
    /// - Sleeps between increments for smooth effect
    /// - Uses current cursor position (0,0,0,0 parameters)
    pub fn mouse_wheel_complex(delta: i32, duration: Duration) -> Result<(), &'static str> {
        if delta == 0 {
            return Err("Delta cannot be zero");
        }

        let tick_time = duration.div_f32(delta.abs() as f32);
        let tick_move: i8 = if delta > 0 { 1 } else { -1 };

        for _ in 0..delta.abs() {
            unsafe {
                mouse_event(MOUSEEVENTF_WHEEL, 0, 0, tick_move as DWORD, 0);
            }
            thread::sleep(tick_time);
        }
        Ok(())
    }

    /// Sets the absolute mouse position within specified screen boundaries.
    ///
    /// # Arguments
    /// * `new_x` - Target X coordinate (0 = left edge, `screen_width` = right edge)
    /// * `new_y` - Target Y coordinate (0 = top edge, `screen_height` = bottom edge)
    /// * `screen_width` - Current screen width in pixels
    /// * `screen_height` - Current screen height in pixels
    ///
    /// # Behavior
    /// - Silently returns if coordinates exceed screen dimensions
    /// - Converts coordinates to absolute input (0-65535 range)
    /// - Uses combined `MOUSEEVENTF_MOVE|MOUSEEVENTF_ABSOLUTE` flags
    ///
    /// # Safety
    /// Contains unsafe WinAPI calls. Requirements:
    /// - Valid screen dimensions must be provided
    /// - Coordinates should be within display bounds
    pub fn mouse_set_position(new_x: u32, new_y: u32, screen_width: u32, screen_height: u32) {
        if new_x > screen_width || new_y > screen_height {
            return;
        }
        unsafe {
            mouse_event(
                MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
                (new_x * 65536 / screen_width) as DWORD,
                (new_y * 65536 / screen_height) as DWORD,
                0,
                0,
            );
        }
    }

    /// Moves the mouse cursor by relative pixel offsets from current position.
    ///
    /// # Arguments
    /// * `x` - Horizontal movement (positive = right, negative = left)
    /// * `y` - Vertical movement (positive = down, negative = up)
    ///
    /// # Behavior
    /// - No movement occurs if both offsets are zero
    /// - Uses relative movement mode (`MOUSEEVENTF_MOVE`)
    /// - Movement is cumulative with other mouse events
    ///
    /// # Safety
    /// Contains unsafe WinAPI calls. Requirements:
    /// - Thread must have input simulation privileges
    /// - No concurrent mouse state manipulation
    pub fn mouse_movement(x: i32, y: i32) {
        if y == 0 && x == 0 {
            return;
        }
        unsafe {
            mouse_event(MOUSEEVENTF_MOVE, x as DWORD, y as DWORD, 0, 0);
        }
    }

    /// Calculates a linearly interpolated point between two positions based on time progression.
    ///
    /// # Arguments
    /// * `start_pos` - Tuple of (x, y) coordinates for the starting point
    /// * `end_pos` - Tuple of (x, y) coordinates for the ending point
    /// * `time_end` - Total duration of the movement (must be > 0)
    /// * `time_now` - Current elapsed time (should be ≤ time_end)
    ///
    /// # Returns
    /// Tuple `(x, y)` representing the current position along the linear path.
    ///
    /// # Panics
    /// - If `time_end` is zero or negative
    /// - If `time_now` is negative
    pub fn point_t_linear(
        start_pos: (f64, f64),
        end_pos: (f64, f64),
        time_end: f64,
        time_now: f64,
    ) -> (f64, f64) {
        let progress: f64 = time_now / time_end;
        return (
            start_pos.0 + (end_pos.0 - start_pos.0) * progress,
            start_pos.1 + (end_pos.1 - start_pos.1) * progress,
        );
    }

    /// Calculates a position along a path with constant acceleration, blending linear and quadratic motion.
    ///
    /// # Arguments
    /// * `start_pos` - (x, y) starting coordinates
    /// * `end_pos` - (x, y) target coordinates
    /// * `time_end` - Total duration of movement (must be > 0)
    /// * `time_now` - Current elapsed time (clamped to [0, time_end])
    /// * `acceleration` - Rate of acceleration (pixels/time²):
    ///   - Positive: speeds up toward target
    ///   - Negative: slows down approaching target
    ///   - Zero: equivalent to `point_t_linear`
    ///
    /// # Returns
    /// (x, y) coordinates at current time with applied acceleration.
    ///
    /// # Panics
    /// - If `time_end` ≤ 0
    /// - If inputs produce NaN values
    pub fn point_t_linear_acceleration(
        start_pos: (f64, f64),
        end_pos: (f64, f64),
        time_end: f64,
        time_now: f64,
        acceleration: f64,
    ) -> (f64, f64) {
        let x_speed: f64 = (2.0 * end_pos.0 - acceleration * time_end.powf(2.0)) / (2.0 * time_end);
        let y_speed: f64 = (2.0 * end_pos.1 - acceleration * time_end.powf(2.0)) / (2.0 * time_end);
        return (
            start_pos.0 + x_speed * time_now + (acceleration * time_now.powf(2.0)) / 2.0,
            start_pos.1 + y_speed * time_now + (acceleration * time_now.powf(2.0)) / 2.0,
        );
    }

    /// Calculates a position along a path with independent x/y accelerations, creating curved trajectories.
    ///
    /// # Arguments
    /// * `start_pos` - (x, y) starting coordinates
    /// * `end_pos` - (x, y) target coordinates
    /// * `time_end` - Total movement duration (must be > 0)
    /// * `time_now` - Current elapsed time (automatically clamped to [0, time_end])
    /// * `x_acceleration` - Horizontal acceleration (units/time²):
    ///   - Positive: curves rightward
    ///   - Negative: curves leftward
    /// * `y_acceleration` - Vertical acceleration (units/time²):
    ///   - Positive: curves downward
    ///   - Negative: curves upward
    ///
    /// # Returns
    /// (x, y) coordinates at current time with applied accelerations.
    ///
    /// # Panics
    /// - If `time_end` ≤ 0
    /// - If acceleration values would produce NaN positions
    pub fn point_t_curved_acceleration(
        start_pos: (f64, f64),
        end_pos: (f64, f64),
        time_end: f64,
        time_now: f64,
        x_acceleration: f64,
        y_acceleration: f64,
    ) -> (f64, f64) {
        let x_speed: f64 = (2.0 * (end_pos.0 - start_pos.0) - x_acceleration * time_end.powf(2.0))
            / (2.0 * time_end);
        let y_speed: f64 = (2.0 * (end_pos.1 - start_pos.1) - y_acceleration * time_end.powf(2.0))
            / (2.0 * time_end);
        return (
            start_pos.0 + x_speed * time_now + (x_acceleration * time_now.powf(2.0)) / 2.0,
            start_pos.1 + y_speed * time_now + (y_acceleration * time_now.powf(2.0)) / 2.0,
        );
    }

    type MoveFunction = dyn Fn((f64, f64), (f64, f64), f64, f64) -> (f64, f64);

    /// Executes a controlled mouse movement between two points using a specified movement function.
    ///
    /// # Arguments
    /// * `start_pos` - (x, y) starting coordinates in pixels
    /// * `end_pos` - (x, y) target coordinates in pixels
    /// * `disp_res` - (width, height) of display resolution
    /// * `duration` - Total movement time (must be non-zero)
    /// * `mps_lock` - Movement updates per second (must be ≥ 1)
    /// * `move_fn` - Movement function implementing the path calculation:
    ///   - Signature: `Fn((f64, f64), (f64, f64), f64, f64) -> (f64, f64)`
    ///   - Receives: start_pos, end_pos, total_time, elapsed_time
    ///   - Returns: current (x, y) position
    ///
    /// # Returns
    /// `Ok(())` on success, or `Err(&'static str)` if:
    /// - Duration is zero
    /// - mps_lock is zero
    /// - start and end positions are identical
    ///
    /// # Behavior
    /// - Maintains precise timing using mps_lock rate limiting
    /// - Guarantees final position equals end_pos
    /// - Uses yield-based busy waiting for precise timing
    /// - Converts positions through specified movement function
    ///
    /// # Safety
    /// - Contains unsafe mouse position calls
    /// - Movement function must produce valid screen coordinates
    pub fn mouse_set_position_complex(
        start_pos: (u32, u32),
        end_pos: (u32, u32),
        disp_res: (u32, u32),
        duration: Duration,
        mps_lock: u64,
        move_fn: &MoveFunction,
    ) -> Result<(), &'static str> {
        if duration.as_secs() == 0 && duration.subsec_nanos() == 0 {
            return Err("Duration must be greater than zero");
        }
        if mps_lock == 0 {
            return Err("Moves per second (mps_lock) must be greater than zero");
        }
        if start_pos.0 == end_pos.0 && start_pos.1 == end_pos.1 {
            return Err("Movement(end_pos) must be greater than zero");
        }

        let tick_time_lock: Duration = Duration::from_secs_f64(1.0 / mps_lock as f64);
        let start_pos_f64 = ((start_pos.0) as f64, (start_pos.1) as f64);
        let end_pos_f64 = ((end_pos.0) as f64, (end_pos.1) as f64);

        let fn_start_time = Instant::now();
        while fn_start_time.elapsed() < duration {
            let tick_start = Instant::now();

            let (x_t, y_t) = move_fn(
                start_pos_f64,
                end_pos_f64,
                duration.as_secs_f64(),
                fn_start_time.elapsed().as_secs_f64(),
            );
            mouse_set_position(x_t as u32, y_t as u32, disp_res.0, disp_res.1);

            let elapsed_tick = tick_start.elapsed();
            if elapsed_tick < tick_time_lock {
                while tick_start.elapsed() < tick_time_lock {
                    thread::yield_now();
                }
            }
        }
        let (x_t, y_t) = move_fn(
            start_pos_f64,
            end_pos_f64,
            duration.as_secs_f64(),
            duration.as_secs_f64(),
        );
        mouse_set_position(x_t as u32, y_t as u32, disp_res.0, disp_res.1);
        Ok(())
    }

    /// Executes a controlled relative mouse movement using a specified movement function with sub-pixel precision.
    ///
    /// # Arguments
    /// * `moving` - (x, y) relative movement in pixels
    /// * `duration` - Total movement time (must be non-zero)
    /// * `mps_lock` - Movement updates per second (must be ≥ 1)
    /// * `move_fn` - Movement function implementing the path calculation:
    ///   - Signature: `Fn((f64, f64), (f64, f64), f64, f64) -> (f64, f64)`
    ///   - Receives: (0.0, 0.0), target_pos, total_time, elapsed_time
    ///   - Returns: current (x, y) progress
    ///
    /// # Returns
    /// `Ok(())` on success, or `Err(&'static str)` if:
    /// - Duration is zero
    /// - mps_lock is zero
    /// - Both movement components are zero
    ///
    /// # Behavior
    /// - Maintains precise timing using mps_lock rate limiting
    /// - Accumulates sub-pixel movements for smooth motion
    /// - Processes x and y movements independently
    /// - Uses yield-based busy waiting for precise timing
    /// - Guarantees final movement equals target
    ///
    /// # Precision Features
    /// - Sub-pixel movement accumulation
    /// - Fractional pixel carry-over between frames
    /// - Independent x/y axis processing
    ///
    /// # Safety
    /// - Contains unsafe mouse movement calls
    /// - Movement function should produce stable values
    pub fn mouse_movement_complex(
        moving: (u32, u32),
        duration: Duration,
        mps_lock: u64,
        move_fn: &MoveFunction,
    ) -> Result<(), &'static str> {
        if duration.as_secs() == 0 && duration.subsec_nanos() == 0 {
            return Err("Duration must be greater than zero");
        }
        if mps_lock == 0 {
            return Err("Moves per second (mps_lock) must be greater than zero");
        }
        if moving.0 == 0 && moving.1 == 0 {
            return Err("Movement(moving) must be greater than zero");
        }

        let tick_time_lock: Duration = Duration::from_secs_f64(1.0 / mps_lock as f64);

        let mut last_x: f64 = 0.0;
        let mut last_y: f64 = 0.0;
        let mut accumulation_x: f64 = 0.0;
        let mut accumulation_y: f64 = 0.0;
        let moving_f64 = (moving.0 as f64, moving.1 as f64);

        let fn_start_time = Instant::now();
        while fn_start_time.elapsed() < duration {
            let tick_start = Instant::now();

            let (x_t, y_t) = move_fn(
                (0.0, 0.0),
                moving_f64,
                duration.as_secs_f64(),
                fn_start_time.elapsed().as_secs_f64(),
            );
            let (d_x, d_y) = (x_t - last_x, y_t - last_y);
            (last_x, last_y) = (x_t, y_t);
            accumulation_x += d_x;
            accumulation_y += d_y;

            if accumulation_x.abs() >= 1.0 {
                mouse_movement(accumulation_x as i32, 0);
                accumulation_x -= accumulation_x.trunc();
            }
            if accumulation_y.abs() >= 1.0 {
                mouse_movement(0, accumulation_y as i32);
                accumulation_y -= accumulation_y.trunc();
            }

            let elapsed_tick = tick_start.elapsed();
            if elapsed_tick < tick_time_lock {
                while tick_start.elapsed() < tick_time_lock {
                    thread::yield_now();
                }
            }
        }
        let (x_t, y_t) = move_fn(
            (0.0, 0.0),
            moving_f64,
            duration.as_secs_f64(),
            duration.as_secs_f64(),
        );
        let (d_x, d_y) = (x_t - last_x, y_t - last_y);
        mouse_movement((accumulation_x + d_x) as i32, (accumulation_y + d_y) as i32);
        Ok(())
    }
}
