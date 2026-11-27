//! Original silent visual prototype: the square-cube law.
use oxframe::prelude::*;

#[oxframe::scene]
async fn hook(view: View) {
    let title = view.add(Txt::new().text("Twice as big.").font_size(80).weight(800).y(-112.0).opacity(0.0));
    title.fade_in().await;
    let question = view.add(Txt::new().text("Twice as much?").font_size(64).fill(Accent).y(0.0).opacity(0.0));
    question.fade_in().await;
    let detail = view.add(Txt::new().text("Length, area and volume do not grow alike.").font_size(30).y(116.0).opacity(0.0));
    detail.fade_in().await;
    wait_for(2.4).await;
}

#[oxframe::scene]
async fn area(view: View) {
    view.add(Txt::new().text("01  /  Two dimensions").font_size(30).fill(Muted).y(-270.0));
    view.add(Txt::new().text("Double the side length").font_size(48).weight(700).y(-210.0));
    let one = view.add(Rect::new().width(100.0).height(100.0).fill(Accent).x(-330.0).y(0.0).opacity(0.0));
    one.fade_in().await;
    view.add(Math::new("1 times 1 = 1").size(40.0).x(-330.0).y(175.0));
    for row in 0..2 {
        for col in 0..2 {
            let tile = view.add(Rect::new().width(100.0).height(100.0)
                .fill(Accent).stroke(color!("#101827")).line_width(3.0)
                .x(260.0 + (col as f64 - 0.5) * 100.0)
                .y((row as f64 - 0.5) * 100.0).opacity(0.0));
            tile.fade_in().await;
        }
    }
    let result = view.add(Math::new("2 times 2 = 4").size(40.0).x(260.0).y(175.0).opacity(0.0));
    result.fade_in().await;
    let note = view.add(Txt::new().text("Four unit squares, not two.").font_size(32).y(278.0).opacity(0.0));
    note.fade_in().await;
    wait_for(2.3).await;
}

#[oxframe::scene]
async fn volume(view: View) {
    view.add(Txt::new().text("02  /  Three dimensions").font_size(30).fill(Muted).y(-270.0));
    view.add(Txt::new().text("Now double the depth, too.").font_size(46).weight(700).y(-210.0));
    let world = Scene3d::new();
    world.camera().position().set(Vec3::new(4.4, 3.5, 5.8));
    let cluster = world.add(Mesh::group());
    let mut cubes = Vec::new();
    for x in [-0.5, 0.5] {
        for y in [-0.5, 0.5] {
            for z in [-0.5, 0.5] {
                let pos = Vec3::new(x, y, z);
                let cube = cluster.add(Mesh::cube().color(Accent).roughness(0.45).position(pos));
                cubes.push((cube, pos));
            }
        }
    }
    view.add(View3d::new(&world).size(600.0, 430.0).x(-250.0).y(18.0));
    let eq = view.add(Math::new("2 times 2 times 2").size(48.0).x(325.0).y(-48.0).opacity(0.0));
    eq.fade_in().await;
    // Explode the exact 2×2×2 partition for visibility, then orbit it.
    for (cube, p) in &cubes {
        cube.position().set(Vec3::new(p.x * 1.22, p.y * 1.22, p.z * 1.22));
    }
    all((
        cluster.rotation().to(Vec3::new(0.0, 140.0, 0.0), 4.0),
        world.camera().position().to(Vec3::new(4.0, 3.0, 5.6), 4.0),
    )).await;
    let eight = view.add(Math::new("= 8").size(100.0).color(Accent).x(325.0).y(74.0).opacity(0.0));
    eight.fade_in().await;
    view.add(Txt::new().text("Eight unit cubes  ·  exploded view").font_size(28).y(288.0));
    wait_for(2.3).await;
}

#[oxframe::scene]
async fn powers(view: View) {
    view.add(Txt::new().text("03  /  The pattern").font_size(30).fill(Muted).y(-270.0));
    let axes = view.add(Axes::new().x_range(0.0..3.0).y_range(0.0..27.0)
        .width(580.0).height(350.0).x(-195.0).y(10.0));
    let linear = axes.plot(|x| x).stroke(color!("#7dd3fc"));
    let square = axes.plot(|x| x*x).stroke(color!("#fbbf24"));
    let cubic = axes.plot(|x| x*x*x).stroke(color!("#a78bfa"));
    linear.draw_on().await;
    square.draw_on().await;
    cubic.draw_on().await;
    view.add(Math::new("L = s").size(44.0).color(color!("#7dd3fc")).x(360.0).y(-120.0));
    view.add(Math::new("A = s^2").size(44.0).color(color!("#fbbf24")).x(360.0).y(-30.0));
    view.add(Math::new("V = s^3").size(44.0).color(color!("#a78bfa")).x(360.0).y(60.0));
    let s = Signal::new(0.0);
    axes.point_on(&cubic, s.clone());
    s.to(3.0, 3.5).await;
    view.add(Txt::new().text("Scale factor s").font_size(26).x(-195.0).y(280.0));
    wait_for(2.3).await;
}

#[oxframe::scene]
async fn conclusion(view: View) {
    view.add(Txt::new().text("The rule in any dimension").font_size(48).weight(700).y(-190.0));
    let formula = view.add(Math::new("s^d").size(150.0).color(Accent));
    stagger(0.12, formula.paths(), |glyph| glyph.pop_in()).await;
    view.add(Txt::new().text("Scale every length by s in d dimensions.").font_size(34).y(145.0));
    view.add(Txt::new().text("The d-dimensional measure scales by s to the power d.").font_size(28).fill(Muted).y(205.0));
    wait_for(2.4).await;
}

fn main() {
    Project::new("example-0001-square-cube")
        .size(1280, 720)
        .fps(30)
        .theme(Theme::dark().accent(color!("#70d6c7")))
        .scene(hook())
        .scene(area())
        .scene(volume())
        .scene(powers())
        .scene(conclusion())
        .main();
}
