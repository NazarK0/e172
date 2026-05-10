use e172::gradient::{
    GradientFillMode, GradientType, LinearGradientFlow, RadialGradientFlow, SweepGradientFlow
};
use e172::ui_components::shapes::{
    Circle, CircleConfig, Rectangle, RectangleConfig, Square, SquareConfig, Triangle,
    TriangleConfig,
};
use e172::{AppConfig, E172App, Screen, engine::VelloEngine};
use e172::{Background, Color, Gradient, Point, color, color_stop_list};

fn main() {
    let engine = VelloEngine::new(None);

    let config = AppConfig {
        title: String::from("Circle Example"),
        engine,
    };

    let mut app = E172App::new(config);
    let mut screen = Screen::new(None);

    let hex_color = Color::from_hex("#2A50A89B");
    let hex_with_separate_alpha_color = Color::from_hex2("#F250A8", 0.5);
    let hsla_color = Color::from_hsla(200.0, 0.5, 0.5, 0.4);
    let hsva_color = Color::from_hsva(200.0, 0.5, 0.5, 0.4);

    let mut gradient_colors_1 = e172::ColorStopList::new();
    gradient_colors_1.push(Color::from_hex2("#F250A8", 0.5), 0.0);
    gradient_colors_1.push(Color::from_hex2("#f2b450", 0.5), 0.4);
    gradient_colors_1.push(Color::from_hex2("#7ef250", 0.5), 0.8);
    gradient_colors_1.push(Color::from_hex2("#5e50f2", 0.5), 1.0);

    let gradient_colors_2 = e172::ColorStopList::from_vec(vec![
        (Color::from_css(color::ColorCSS::Red), 0.0),
        (Color::from_css(color::ColorCSS::Lime), 0.4),
        (Color::from_css(color::ColorCSS::Blue), 0.8),
        (Color::from_css(color::ColorCSS::Yellow), 1.0),
    ]);

    let gradient_colors_3 = color_stop_list![
        Color::from_hex2("#b8136e", 1.0),
        Color::from_hex2("#ddc11f", 1.0),
        Color::from_hex2("#41a918", 1.0),
        Color::from_hex2("#5e50f2", 1.0),
    ];

    let gradient_linear = Gradient::new(
        GradientType::Linear(LinearGradientFlow {
            angle: 90.0,
            relative_width: 1.0,
        }),
        gradient_colors_2.clone().set_alpha(1.0),
        GradientFillMode::Pad,
    );

    let gradient_linear_2 = Gradient::new(
        GradientType::Linear(LinearGradientFlow {
            angle: 45.0,
            relative_width: 1.0,
        }),
        gradient_colors_3.clone(),
        GradientFillMode::Pad,
    );

    let gradient_radial = Gradient::new(
        GradientType::Radial(RadialGradientFlow {
            relative_center: Point::new(0.0, 0.0),
            relative_radius: 1.0,
        }),
        gradient_colors_1.clone(),
        GradientFillMode::Pad,
    );

    let gradient_sweep = Gradient::new(
        GradientType::Sweep(SweepGradientFlow {
            relative_center: Point::new(0.0, 0.0), // circle center
            start_angle: 0.0,
            end_angle: 1.8 * std::f64::consts::PI,
        }),
        gradient_colors_1.clone().set_alpha(0.8),
        GradientFillMode::Pad,
    );

    let circle_1 = Circle::new(CircleConfig {
        id: Some(String::from("circle1")),
        radius: 100.0,
        background: Background::Solid(Color::from_rgba(255, 0, 0, 128)),
    });

    let circle_2 = Circle::new(CircleConfig {
        id: None,
        radius: 100.0,
        background: Background::Gradient(gradient_linear_2),
    });

    let circle_3 = Circle::new(CircleConfig {
        id: None,
        radius: 120.0,
        background: Background::Gradient(gradient_radial),
    });

    let circle_4 = Circle::new(CircleConfig {
        id: None,
        radius: 100.0,
        background: Background::Gradient(gradient_sweep),
    });

    let triangle_1 = Triangle::new(TriangleConfig {
        id: None,
        a: Point::new(100.0, 50.0),
        b: Point::new(150.0, 100.0),
        c: Point::new(50.0, 100.0),
        background: Background::Solid(hex_color),
    });

    let triangle_2 = Triangle::new(TriangleConfig {
        id: None,
        a: Point::new(100.0, 50.0),
        b: Point::new(150.0, 100.0),
        c: Point::new(50.0, 100.0),
        background: Background::Gradient(gradient_linear),
    });

    let rectangle_1 = Rectangle::new(RectangleConfig {
        id: None,
        width: 100.0,
        height: 50.0,
        background: Background::Solid(hex_with_separate_alpha_color),
    });

    let square_1 = Square::new(SquareConfig {
        id: None,
        side: 100.0,
        background: Background::Solid(hsla_color),
    });

    let square_2 = Square::new(SquareConfig {
        id: None,
        side: 100.0,
        background: Background::Solid(hsva_color),
    });

    screen.add_component(circle_1, Point::new(200.0, 200.0));
    screen.add_component(circle_2, Point::new(120.0, 440.0));
    screen.add_component(circle_3, Point::new(370.0, 400.0));
    screen.add_component(circle_4, Point::new(480.0, 240.0));
    screen.add_component(triangle_1, Point::new(100.0, 50.0));
    screen.add_component(rectangle_1, Point::new(300.0, 250.0));
    screen.add_component(square_1, Point::new(300.0, 40.0));
    screen.add_component(square_2, Point::new(370.0, 40.0));
    screen.add_component(triangle_2, Point::new(570.0, 40.0));

    // Standalone Text component usage
    // screen.add_component(Text::new(
    //     "label_info",
    //     "Hello, Vello!",
    //     Point::new(50.0, 500.0),
    //     24.0,
    //     Background::Solid(Color::from_rgba(0, 0, 0, 255)),
    //     Background::Solid(Color::from_rgba(255, 0, 0, 255)),
    // ));

    // screen.get_component_by_id("circle1").map(|component| {
    //     println!("Found component with ID 'circle1': {:?}", component.id());
    // });

    app.add_screen(screen);
    app.run();
}
