use nannou::prelude::*;
use nannou::winit::event::DeviceEvent;
use crate::node::Node;
use crate::tree::{Tree, TreeIndex, TreesEnum};

pub struct Model {
    pub tree: Tree,
    pub camera: Point2,
    pub scale: f64,
    pub selected: Option<TreeIndex>,
    pub click: Point2,
    pub render_options: RenderOptions,
}

pub struct RenderOptions {
    show_path: bool,
}

impl RenderOptions {
    pub fn default() -> Self {
        Self {
            show_path: false
        }
    }
}

pub const SIZE: f32 = 5.0;


pub fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple, .. } => {
            if simple.is_none() {
                return;
            }
            match simple.unwrap() {
                MousePressed(MouseButton::Left) => {
                    let data = Point2::new(
                        app.mouse.position().x / model.scale as f32,
                        app.mouse.position().y / model.scale as f32)
                        - model.camera;
                    model.click = data;
                    model.selected = model.tree.find_node_at_pos(data, SIZE / model.scale as f32);
                }
                MouseReleased(_) => {
                    model.selected = None;
                }
                _ => {}
            }
        }
        Event::DeviceEvent(_, ref data) => {
            if let DeviceEvent::MouseMotion { delta } = data {
                if app.mouse.buttons.middle().is_down() {
                    model.camera.x += (delta.0 / model.scale) as f32;
                    model.camera.y -= (delta.1 / model.scale) as f32;
                }
                if app.mouse.buttons.left().is_down() {
                    if let Some(index) = &model.selected {
                        model.tree[*index].pos += vec2((delta.0 / model.scale) as f32, -(delta.1 / model.scale) as f32)
                    }
                }
            }
            if let DeviceEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_x, y) } = data {
                model.scale += -1.0 * f64::max(-1.0, f64::min(1.0, *y as f64)) * 0.5 * model.scale;
            }
        }
        Event::Update(_) => {}
        Event::Suspended => {}
        Event::Resumed => {}
    }
}

pub fn render_triangle(app: &App, model: &Model) {
    let tree = &model.tree;
    let cam_pos = model.camera;
    let scale = model.scale as f32;
    let draw = app.draw();
    draw.background().color(BLACK);

    //Draw triangle bounds
    draw.line().end((tree.tree1.last().unwrap().pos + cam_pos) * scale).start((tree.tree2.last().unwrap().pos + cam_pos) * scale).color(GRAY);
    draw.line().end((tree.tree2.last().unwrap().pos + cam_pos) * scale).start((tree.tree3.last().unwrap().pos + cam_pos) * scale).color(GRAY);
    draw.line().end((tree.tree3.last().unwrap().pos + cam_pos) * scale).start((tree.tree1.last().unwrap().pos + cam_pos) * scale).color(GRAY);

    //Draw edges
    for e in tree.get_all_edges() {
        draw.line().end((e.0 + cam_pos) * scale).start((e.1 + cam_pos) * scale).color(GRAY);
    }

    //Draw all the nodes
    for index in tree.iter() {
            draw.ellipse().xy((tree[index].pos + cam_pos) * scale).color(
                match index.0 {
                    TreesEnum::First => { BLUE }
                    TreesEnum::Second => { GREEN }
                    TreesEnum::Third => { RED }
                    TreesEnum::Center => { GRAY }
                }
            ).radius(SIZE);
    }

    if let Some(index) = model.selected {
        draw_vis_edges(&draw, model, index);
        draw.ellipse().xy((tree[index].pos + cam_pos) * scale).color(WHITE).radius(SIZE);
    };

    draw_node_list(&draw, model, tree.find_special_nodes(), DARKCYAN);
}

fn draw_node_list(draw: &Draw, model: &Model, list: Vec<TreeIndex>, color: Srgb<u8>) {
    let tree = &model.tree;
    let cam_pos = model.camera;
    let scale = model.scale as f32;
    for index in list {
        draw.ellipse().xy((tree[index].pos + cam_pos) * scale).color(
            color
        ).radius(SIZE);
    }
}

fn draw_vis_edges(draw: &Draw, model: &Model, index: TreeIndex) {
    let tree = &model.tree;
    let cam_pos = model.camera;
    let scale = model.scale as f32;
    let edges = tree.check_node_vis(index);
    for edge in edges {
        draw.line().end((tree[index].pos + cam_pos) * scale).start((tree[edge].pos + cam_pos) * scale).color(PURPLE);
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    render_triangle(app, model);

    draw.to_frame(app, &frame).unwrap();
}