use graph1::core::context::{GraphContext, WindowContext};
use graph1::primitives::point::Point;
use graph1::utils::color::palettes::RetroNeon;
use graph1_wasm_demo::demo::user_data::DemoUserData;
use minifb::{Key, Window, WindowOptions};
use graph1::utils::color::adapters::rgba_to_0rgb;

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 1600;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 1200;

// const NUM_THREADS: usize = 6;
const NUM_THREADS: usize = 6;
const GPU: bool = true;

fn main() {
    // println!("Hello, world!");

    let x: f64 = -10.0123456789;
    let y: f64 = 20.987654321;

    // Define a point with coordinates of f64 type
    let point_f64: Point<f64> = Point::new(x, y);

    // Lose some precision when converting to `Point<f32>`
    let point_f32: Point<f32> = point_f64.convert();

    // Lose the floating-point part when converting to `Point<i32>`
    let point_i32: Point<i32> = point_f32.convert();

    // Conversion to `Point<u32>` truncates negative coordinates to zero
    let point_u32: Point<u32> = point_i32.convert();

    let another_point_u32: Point<u32> = point_f64.convert();

    /*
    println!(
        "point_f64: {:?}\n\
        point_f32: {:?}\n\
        point_i32: {:?}\n\
        point_u32: {:?}\n\
        another_point_u32: {:?}",
        point_f64, point_f32, point_i32, point_u32, another_point_u32
    );
    */

    let mut width = WIN_WIDTH as usize;
    let mut height = WIN_HEIGHT as usize;

    // The window context
    let mut win_ctx = WindowContext::new(
        WIN_WIDTH,
        WIN_HEIGHT,
        Some(RetroNeon::CYBER_BLUE),
        Some(RetroNeon::LASER_LIME),
        // We can avoid using the color adapter for the output buffer if we adapt the input colors
        // That makes alpha blending impossible though.
        // Some(rgba_color_to_0rgb(RetroNeon::CYBER_BLUE)),
        // Some(rgba_color_to_0rgb(RetroNeon::LASER_LIME)),
    );

    let mut output_buf_0rgb: Vec<u32> = vec![win_ctx.background_color; win_ctx.get_num_pixels()];

    // Graph context
    let mut ctx: GraphContext<DemoUserData> =
        GraphContext::new(win_ctx, true, 1, None, NUM_THREADS, None);
    // let mut ctx:GraphContext<DemoUserData> = GraphContext::new(win_ctx,  true,false, None, 1);

    ctx.win.background_color = 0x00_00_00_ff;
    ctx.user_data.bouncy.dx = 2;
    ctx.user_data.bouncy.dy = 2;
    ctx.alpha.method = graph1::core::context::alpha::AlphaMethod::Float;
    ctx.alpha.enabled = true;

    // Draw a rectangle of size 40x20 at the top-left corner of the window
    // draw::rectangle::filled(&mut ctx, &RectArea::new(0, 0, 40, 20, None));

    // ctx.set_gpu_state(true);
    // ctx.set_gpu_state(true);

    let mut window_options: WindowOptions = WindowOptions::default();
    window_options.resize = true;
    window_options.scale_mode = minifb::ScaleMode::Center;

    let mut window = Window::new(
        "Graph1 - Minifb demo",
        ctx.win.w_usize,
        ctx.win.h_usize,
        window_options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(100);
    window.set_position(100, 100);

    // NB! [ GPU ]======================================================================
    ctx.gpu_context.set_enabled(GPU);
    // NB! [ GPU ]======================================================================

    // MAIN LOOP
    // **************
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        // let frame_start = std::time::Instant::now();
        // println!(">>> frame_count: {} ", ani_ctx.frame_count);

        // =====================================================================================
        // Handle window resizing
        // =====================================================================================

        if window.is_key_released(Key::H) {
            ctx.user_data.intro.z_heatmap ^= true;
        }
        if window.is_key_released(Key::A) {
            ctx.user_data.intro.autoscale ^= true;
        }
        if window.is_key_released(Key::X) {
            ctx.user_data.intro.rotate_x ^= true;
        }
        if window.is_key_released(Key::Y) {
            ctx.user_data.intro.rotate_y ^= true;
        }
        if window.is_key_released(Key::Z) {
            ctx.user_data.intro.rotate_z ^= true;
        }
        if window.is_key_released(Key::B) {
            ctx.user_data.intro.x_log_y ^= true;
        }
        if window.is_key_released(Key::U) {
            ctx.user_data.intro.x_log ^= true;
        }
        if window.is_key_released(Key::V) {
            ctx.user_data.intro.y_log ^= true;
        }
        if window.is_key_released(Key::W) {
            ctx.user_data.intro.z_log ^= true;
        }
        if window.is_key_released(Key::S) {
            ctx.user_data.intro.shift_z ^= true;
        }
        if window.is_key_released(Key::C) {
            ctx.user_data.intro.y_zero_centered ^= true;
        }
        if window.is_key_released(Key::Up) {
            ctx.user_data.intro.z_shift += 10.0;
        }
        if window.is_key_released(Key::Down) {
            ctx.user_data.intro.z_shift -= 10.0;
        }
        if window.is_key_released(Key::Right) {
            ctx.user_data.intro.variance += 1.0;
        }
        if window.is_key_released(Key::Left) {
            ctx.user_data.intro.variance -= 1.0;
        }
        if window.is_key_released(Key::RightBracket) {
            ctx.user_data.intro.x_log_base += 1.0;
        }
        if window.is_key_released(Key::LeftBracket) {
            ctx.user_data.intro.x_log_base -= 1.0;
        }
        if window.is_key_released(Key::PageUp) {
            ctx.user_data.intro.scale *= 1.1;
        }
        if window.is_key_released(Key::Home) {
            ctx.user_data.intro.scale *= 10.0;
        }
        if window.is_key_released(Key::PageDown) {
            ctx.user_data.intro.scale /= 1.1;
        }
        if window.is_key_released(Key::End) {
            ctx.user_data.intro.scale /= 10.0;
        }
        // Check if the window size has changed
        let (new_width, new_height) = window.get_size();

        if new_width != width || new_height != height {
            // Update dimensions and buffer
            width = new_width;
            height = new_height;
            ctx.resize(width as u32, height as u32);
            output_buf_0rgb.resize(ctx.win.get_num_pixels(), 0);

            println!("Window resized to: {}x{}", width, height);
        }

        // let start = Instant::now(); // Start timing
        // ===[ DEMO SELECTION ]===========================

        graph1_wasm_demo::demo::d_000_intro::render_frame(&mut ctx);

        // let duration = start.elapsed(); // Measure elapsed time
        // println!("[cube] duration:  {:?}", duration.as_micros());

        // graph1_wasm_demo::demo::d_004_bouncy::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_001_basic_concepts_pt1::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_007_shapes_and_quadrants::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_005_alpha::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_006_luminance_vs_intensity::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_001_basic_concepts_pt1::render_frame(&mut ctx);

        ctx.alpha.method = graph1::core::context::alpha::AlphaMethod::Int;
        ctx.alpha.enabled = true;
        // graph1_wasm_demo::demo::d_014_brushes::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_015_transformations::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_012_grid::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_012_grid::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_000_intro::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_014_brushes::render_frame(&mut ctx);
        // graph1_wasm_demo::demo::d_013_text::render_frame(&mut ctx);

        // graph1_wasm_demo::demo::d_015_transformations::render_frame(&mut ctx);

        // graph1_wasm_demo::demo::d_005_alpha::render_frame(&mut ctx);

        // graph1_wasm_demo::demo::d_014_brushes::render_frame(&mut ctx);

        // let duration = start.elapsed(); // Measure elapsed time
        // println!("Frame duration:  {:?}", duration.as_micros());

        // ===[ COLOR ADAPTER ]===========================
        // let start = Instant::now();
        let _ = rgba_to_0rgb(
            &mut output_buf_0rgb,
            &mut ctx.frame_buf,
            ctx.num_threads,
            false,
        );
        // let duration = start.elapsed();
        // println!(
        //     "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
        //     ctx.num_threads,
        //     duration.as_millis(),
        //     output_buf_0rgb.len()
        // );

        // println!("[ multithreaded ] stats: {:?}", stats);

        // rgba_to_0rgb_unsafe(&mut output_buf_0rgb, &mut ctx.frame_buf,false);

        /* REDRAW THE MAIN WINDOW
         ********************************************************************************************/
        window
            .update_with_buffer(&output_buf_0rgb, ctx.win.w_usize, ctx.win.h_usize)
            // Use the frame_buf directly for the output without the color adapter
            // .update_with_buffer(&ctx.frame_buf, ctx.win.w_usize, ctx.win.h_usize)
            .unwrap();

        ctx.frame_count += 1;
        //
        //
        // println!("[frame] duration:  {:?}, fps: {:.0}", frame_start.elapsed().as_micros(), 1.0/frame_start.elapsed().as_secs_f64());
        //
        //
    } // main while loop
}
