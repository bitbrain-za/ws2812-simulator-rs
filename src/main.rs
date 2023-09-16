use std::sync::Mutex;

use glib::{timeout_add_local, ControlFlow};
use gtk::prelude::*;
use gtk::{cairo, glib, Application, ApplicationWindow};
use gtk4 as gtk;
use lazy_static::lazy_static;

use smart_led_effects::strip as effects;
use smart_led_effects::strip::EffectIterator;
use smart_led_effects::Srgb;

const COUNT: usize = 55;
const EFFECTS_COUNT: usize = 13;
const REFRESH_DURATION: std::time::Duration = std::time::Duration::from_millis(50);

lazy_static! {
    static ref INDEX: Mutex<usize> = Mutex::new(14);
    static ref BOUNCE: Mutex<effects::Bounce> =
        Mutex::new(effects::Bounce::new(COUNT, None, None, None, None, None));
    static ref RAINBOW: Mutex<effects::Rainbow> = Mutex::new(effects::Rainbow::new(COUNT, None));
    static ref BREATHE: Mutex<effects::Breathe> =
        Mutex::new(effects::Breathe::new(COUNT, None, None));
    static ref COLLIDE: Mutex<effects::Collision> =
        Mutex::new(effects::Collision::new(COUNT, None));
    static ref CYCLE: Mutex<effects::Cycle> = Mutex::new(effects::Cycle::new(COUNT, None));
    static ref FIRE: Mutex<effects::Fire> = Mutex::new(effects::Fire::new(COUNT, None, None));
    static ref METEOR: Mutex<effects::Meteor> =
        Mutex::new(effects::Meteor::new(COUNT, None, None, None));
    static ref RUNNING_LIGHTS: Mutex<effects::RunningLights> =
        Mutex::new(effects::RunningLights::new(COUNT, None, false));
    static ref CYLON: Mutex<effects::Cylon> = Mutex::new(effects::Cylon::new(
        COUNT,
        Srgb::<u8>::new(255, 0, 0),
        None,
        None
    ));
    static ref TIMER: Mutex<effects::Timer> = Mutex::new(effects::Timer::new(
        COUNT,
        std::time::Duration::from_millis(5000),
        None,
        None,
        None,
        true
    ));
    static ref TWINKLE: Mutex<effects::Twinkle> =
        Mutex::new(effects::Twinkle::new(COUNT, None, None, None, None));
    static ref SPARKLE: Mutex<effects::Twinkle> =
        Mutex::new(effects::Twinkle::sparkle(COUNT, None));
    static ref SNOW: Mutex<effects::SnowSparkle> =
        Mutex::new(effects::SnowSparkle::new(COUNT, None, None, None, None));
    static ref CHRISTMAS: Mutex<effects::Christmas> =
        Mutex::new(effects::Christmas::new(COUNT, None, None, None, None));
    static ref WIPE: Mutex<effects::Wipe> = Mutex::new(effects::Wipe::colour_wipe(
        COUNT,
        Srgb::<u8>::new(0, 255, 0),
        false
    ));
}

fn get_effect(index: usize) -> Vec<Srgb<u8>> {
    match index {
        0 => BOUNCE.lock().unwrap().next().unwrap(),
        1 => RAINBOW.lock().unwrap().next().unwrap(),
        2 => BREATHE.lock().unwrap().next().unwrap(),
        3 => CYCLE.lock().unwrap().next().unwrap(),
        4 => FIRE.lock().unwrap().next().unwrap(),
        5 => METEOR.lock().unwrap().next().unwrap(),
        6 => RUNNING_LIGHTS.lock().unwrap().next().unwrap(),
        7 => CYLON.lock().unwrap().next().unwrap(),
        8 => TIMER.lock().unwrap().next().unwrap(),
        9 => TWINKLE.lock().unwrap().next().unwrap(),
        10 => SPARKLE.lock().unwrap().next().unwrap(),
        11 => SNOW.lock().unwrap().next().unwrap(),
        12 => WIPE.lock().unwrap().next().unwrap(),
        13 => CHRISTMAS.lock().unwrap().next().unwrap(),
        14 => COLLIDE.lock().unwrap().next().unwrap(),

        _ => BOUNCE.lock().unwrap().next().unwrap(),
    }
}

fn draw_pixels(_da: &gtk::DrawingArea, cr: &cairo::Context, width: i32, height: i32) {
    let index = *INDEX.lock().unwrap();
    let effect = get_effect(index);

    let pixels = effect
        .iter()
        .map(|x| x.into_format::<f64>())
        .collect::<Vec<Srgb<f64>>>();

    for (i, pixel) in pixels.iter().enumerate() {
        cr.set_source_rgb(pixel.red, pixel.green, pixel.blue);
        cr.rectangle(
            0.0 + i as f64 * width as f64 / COUNT as f64,
            0.0,
            width as f64 / COUNT as f64,
            height as f64,
        );
        let _ = cr.stroke_preserve();
        let _ = cr.fill();
    }
}

fn main() {
    let list = effects::list();
    println!("Effects: {:?}", list);
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(100)
            .title("Hello, World!")
            .build();

        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_size_request(640, 20);
        drawing_area.set_draw_func(move |da, cr, width, height| {
            draw_pixels(da, cr, width, height);
        });

        let button = gtk::Button::with_label("Next");
        button.connect_clicked(move |_| {
            let mut index = INDEX.lock().unwrap();
            *index += 1;
            if *index >= EFFECTS_COUNT {
                *index = 0;
            }
        });

        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 0);
        box_.append(&button);
        box_.append(&drawing_area);
        window.set_child(Some(&box_));

        timeout_add_local(REFRESH_DURATION, move || {
            drawing_area.queue_draw();
            ControlFlow::Continue
        });

        window.present();
    });

    app.run();
}
