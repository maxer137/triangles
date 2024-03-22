use nannou::prelude::*;
use nannou::winit::event::DeviceEvent;
use crate::node::Node;
use crate::tree::{Tree, TreesEnum};

pub struct Model {
    tree: Tree,
    camera: Point2,
    scale: f64,
    selected: Option<(TreesEnum, usize)>,
    click: Point2,
    render_options: RenderOptions,
}

pub struct RenderOptions {
    show_path: bool,
}

impl RenderOptions {
    fn default() -> Self {
        Self {
            show_path: false
        }
    }
}

pub const SIZE: f32 = 5.0;

pub fn model(_app: &App) -> Model {
    let mut t = Tree::empty();
    for i in 0..10 {
        t.add_node(TreesEnum::First, Node::from_pos(0.0, (i * 10) as f32));
        t.add_node(TreesEnum::Second, Node::from_pos(0.0, -(i * 10) as f32));
        t.add_node(TreesEnum::Third, Node::from_pos((i * 10) as f32, -(i * 10) as f32));
    }
    t.add_node(TreesEnum::Second, Node::from_pos(-250.0, -300.0));
    t.add_node(TreesEnum::Third, Node::from_pos(250.0, -300.0));
    Model {
        tree: t,
        camera: Point2::new(0.0, 200.0),
        scale: 1.0,
        selected: None,
        click: (0.0, 0.0).into(),
        render_options: RenderOptions::default(),
    }
}

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
                    if let Some((tree, index)) = &model.selected {
                        match tree {
                            TreesEnum::First => { model.tree.tree1[*index].pos += vec2((delta.0 / model.scale) as f32, -(delta.1 / model.scale) as f32) }
                            TreesEnum::Second => { model.tree.tree2[*index].pos += vec2((delta.0 / model.scale) as f32, -(delta.1 / model.scale) as f32) }
                            TreesEnum::Third => { model.tree.tree3[*index].pos += vec2((delta.0 / model.scale) as f32, -(delta.1 / model.scale) as f32) }
                        }
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

    for tree_branch in TreesEnum::iterator() {
        draw.line().end((tree[*tree_branch].first().unwrap().pos + cam_pos) * scale).start((tree.pos + cam_pos) * scale).color(GRAY);
        //Draw the lines between nodes
        for i in 0..tree[*tree_branch].len() - 1 {
            // Print the current element and the next element
            draw.line().end((tree[*tree_branch][i].pos + cam_pos) * scale).start((tree[*tree_branch][i + 1].pos + cam_pos) * scale).color(GRAY);
        }
    }

    //Draw all the nodes
    for tree_branch in TreesEnum::iterator() {
        for node in &tree[*tree_branch] {
            draw.ellipse().xy((node.pos + cam_pos) * scale).color(
                match tree_branch {
                    TreesEnum::First => { BLUE }
                    TreesEnum::Second => { GREEN }
                    TreesEnum::Third => { RED }
                }
            ).radius(SIZE);
        }
    }

    if let Some((select, index)) = &model.selected {
        draw.ellipse().xy((tree[*select][*index].pos + cam_pos) * scale).color(WHITE).radius(SIZE);
    };

    //Draw center for triangle
    draw.ellipse().xy((tree.pos + cam_pos) * scale).color(GRAY).radius(SIZE);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    render_triangle(app, model);

    draw.to_frame(app, &frame).unwrap();
}