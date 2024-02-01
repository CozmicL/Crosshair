extern crate winapi;

use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

use notan::prelude::*;
use notan::draw::*;

// ! TODO: CLEAN
//* Split into different sections */


//TODO: Implement a start menu and allow to choose screen size and crosshair (and upload picture) almost done
//TODO: Title and icon

//TODO: Make sure that you can resize crosshair and make a gui to see the changes
//TODO: Possibly make a crosshair creator
#[derive(AppState)]
struct State {
    font: Font,
    welcome_time: f32,
    res_time: f32,
    confirmed: bool,
}


#[notan_main]
fn main() -> Result<(), String> {
    
    init_start()
}

fn init_crosshair(w:f32,h: f32, m_w:f32, m_h:f32) -> Result<(), String>{
    let win = WindowConfig::default()
        .set_size(((m_w/2.0)+(w/2.0)) as u32, ((m_h/2.0)+(h/2.0))as u32) // !w/2 + size | h/2 + size CHANGE
        .set_position(0,0)
        .set_transparent(true)
        .set_decorations(false)
        .set_mouse_passthrough(true)
        .set_always_on_top(true);
    
    notan::init_with(init)
        .add_config(win)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn init_start() -> Result<(), String> {
    let win = WindowConfig::default()
        .set_size(500, 500)
        .set_transparent(true)
        .set_decorations(false);
    
    notan::init_with(init)
        .add_config(win)
        .add_config(DrawConfig)
        .draw(draw_start)
        .build()
}
fn init(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Ubuntu-Medium.ttf"))
        .unwrap();
    State {font:font,welcome_time:0.0,res_time:0.0,confirmed:false }
}

fn draw_start(app: &mut App,gfx: &mut Graphics, state: &mut State){
    let window = app.window();
    let ww = window.width() as f32;
    let hh = window.height() as f32;

    let text = "Welcome!";
    let lerp_duration = 1.0;
    let lerp_factor = (state.welcome_time / lerp_duration).min(1.0);
    let lerped_alpha = lerp(1.0, 0.0, lerp_factor);
    state.welcome_time += app.timer.delta_f32();
    state.welcome_time = state.welcome_time.min(lerp_duration);    
    
    let mut draw = gfx.create_draw();
    draw.clear(Color::from_rgb(0.72, 0.84, 0.96));
    draw.text(&state.font, text)
        .color(Color::from_rgba(1.0,1.0,1.0,lerped_alpha))
        .position(ww * 0.5, hh * 0.5)
        .size(40.0)
        .h_align_center()
        .v_align_middle();

    if lerped_alpha == 0.0 {
        let width;
        let height;
        unsafe {
            width = GetSystemMetrics(SM_CXSCREEN);
            height = GetSystemMetrics(SM_CYSCREEN);
        }

        
        let text2 = format!("{} x {}", width, height);

        let lerp_duration2 = 0.5;

        let lerp_factor2 = (state.res_time / lerp_duration2).min(1.0);


        let lerped_alpha2 = lerp(0.0, 1.0, lerp_factor2);
        state.res_time += app.timer.delta_f32();

        state.res_time = state.res_time.min(lerp_duration2);

        draw.text(&state.font, text2.as_str())
            .color(Color::from_rgba(1.0, 1.0, 1.0, lerped_alpha2))
            .position(ww * 0.5, hh * 0.5)
            .size(40.0)
            .h_align_center()
            .v_align_middle();
        if lerped_alpha2 == 1.0{
            draw.text(&state.font, "Does res look right? (if so press space to continue)")
            .color(Color::from_rgba(1.0, 1.0, 1.0, 0.8))
                .position(ww * 0.5, hh * 0.6)
                .size(20.0)
                .h_align_center();
            {
                let app_timer = &app.timer;
                state.res_time += app_timer.delta_f32();
            }
        
            if state.res_time >= lerp_duration2 {
                let is_space_down = app.keyboard.is_down(KeyCode::Space);
        
                if is_space_down {
                    state.confirmed = true;
        
                    let window = app.window();
                    window.set_size((width / 2) as u32 + (16 / 2), (height / 2) as u32 + (16 / 2));
                    window.set_position(0, 0);
                    window.set_mouse_passthrough(true);
                    window.set_always_on_top(true);
                }
            }
        
        }
    }
    gfx.render(&draw);
}


fn draw(app: &mut App,gfx: &mut Graphics) { // * TEMPORARY
    draw_square(app,gfx, 6.0, 6.0)
    //draw_img(app,gfx, 16.0, 16.0)
}

fn draw_square(app: &mut App,gfx: &mut Graphics, w: f32, h: f32) {
    let window = app.window();
    let mut draw = gfx.create_draw();
    draw.clear(Color::TRANSPARENT);
    draw.rect((window.width() as f32 - w*1.5, window.height() as f32 - h*1.5), (w, h))
        .color(Color::MAGENTA);
    gfx.render(&draw);
}

fn draw_img(app: &mut App,gfx: &mut Graphics, w: f32, h: f32){
    let window = app.window();
    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("Crosshairs_tmep/2.png"))
        .build()
        .unwrap();

    let mut draw = gfx.create_draw();

    println!("{} {}",window.width(),window.height());

    draw.image(&texture)
        .position(window.width() as f32 - w*1.5, window.height() as f32 - h*1.5)
        .size(w,h);

    gfx.render(&draw);
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}