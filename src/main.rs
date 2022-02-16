use mobject::{geometry::Rectangle, mobject::Mobject};

mod mobject;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
    let mut rect = Rectangle::new();
    rect.scale(15.0, None, None);
    for point in rect.points() {
        println!("{}", point);
    }
}

struct Model {
    rect: Rectangle,
}

fn model(_app: &App) -> Model {
    let frames = vec![Frame { objects }];
    Model {
        frames,
        current_frame: 0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {}

fn view(app: &App, model: &Model, nframe: nannou::Frame) {
    let draw = app.draw();

    let frame = &model.frames[model.current_frame];
    for object in &frame.objects {
        object.borrow().view(&draw);
    }
    draw.to_frame(app, &nframe).unwrap();
}
