//! Example 0002: Euler's formula, expressed as continuous motion.
use oxframe::prelude::*;
use std::f64::consts::{PI, TAU};

fn backdrop(view: View) {
    // Restrained concentric strokes, not a starfield or distracting texture.
    for (r, alpha) in [(560.0, 0.23), (490.0, 0.20), (420.0, 0.16)] {
        // Clip the decorative circles analytically into two safe-area arcs.
        // No oversized off-frame node or accidental line across the composition.
        for side in [-1.0, 1.0] {
            let pts = (0..=120).map(|j| {
                let y = -300.0 + 600.0 * j as f64 / 120.0;
                let x = side * (r * r - y * y).sqrt();
                (x, y)
            }).collect::<Vec<_>>();
            view.add(Line::new().points(pts).stroke(color!("#34435a")).line_width(1.0).opacity(alpha));
        }
    }
}
fn ring_points(r: f64) -> Vec<(f64, f64)> {
    (0..=160).map(|j| {
        let a = TAU * j as f64 / 160.0;
        (r * a.cos(), -r * a.sin())
    }).collect()
}

#[oxframe::scene]
async fn circle_and_waves(view: View) {
    backdrop(view);
    let eyebrow = view.add(Txt::new().text("A  C I R C L E   H I D D E N   I N").font_size(22).weight(500).fill(color!("#9ba9ba")).y(-165.0).opacity(0.0));
    let title = view.add(Txt::new().text("Exponentials").font_size(72).weight(700).fill(color!("#f4eee0")).y(-94.0).opacity(0.0));
    let hero = view.add(Math::new("e^(i t)").size(152.0).color(color!("#e9c77f")).y(65.0).opacity(0.0));
    all((eyebrow.opacity().to(1.0, 1.2), title.opacity().to(1.0, 1.2), hero.opacity().to(1.0, 2.0))).await;
    wait_for(1.9).await;
    all((eyebrow.opacity().to(0.0, 1.0), title.opacity().to(0.0, 1.0), hero.y().to(-258.0, 1.5), hero.scale().to(0.44, 1.5))).await;

    // All geometry, projections and graph markers follow the same angle.
    let g = view.add(Group::new().opacity(0.0).y(5.0));
    let r = 152.0;
    let ring = g.add(Line::new().points(ring_points(r)).stroke(color!("#51b8b7")).line_width(2.8));
    g.add(Line::new().points([(-190.0, 0.0), (190.0, 0.0)]).stroke(color!("#425269")).line_width(1.6));
    g.add(Line::new().points([(0.0, -190.0), (0.0, 190.0)]).stroke(color!("#425269")).line_width(1.6));
    g.add(Math::new("1").size(24.0).color(color!("#9ba9ba")).x(169.0).y(21.0));
    g.add(Math::new("-1").size(24.0).color(color!("#9ba9ba")).x(-169.0).y(21.0));
    g.add(Math::new("i").size(28.0).color(color!("#9ba9ba")).x(19.0).y(-175.0));
    g.add(Math::new("-i").size(25.0).color(color!("#9ba9ba")).x(22.0).y(175.0));
    let angle = Signal::new(0.0_f64);
    let vector = g.add(Line::new().points([(0.0,0.0),(r,0.0)]).stroke(color!("#e9c77f")).line_width(3.0).end_arrow(true));
    let a = angle.clone();
    vector.points().bind(move || vec![Vec2::new(0.0,0.0), Vec2::new(r*a.get().cos(),-r*a.get().sin())]);
    let dot = g.add(Circle::new().size(13.0).fill(color!("#f9dfa6")).glow(color!("#e9c77f"), 10.0));
    let a = angle.clone(); dot.x().bind(move || r*a.get().cos());
    let a = angle.clone(); dot.y().bind(move || -r*a.get().sin());
    let note = view.add(Txt::new().text("A complex exponential traces the unit circle.").font_size(29).fill(color!("#f4eee0")).y(266.0).opacity(0.0));
    all((g.opacity().to(1.0, 1.1),note.opacity().to(1.0, 1.1))).await;
    ring.draw_on().await;
    angle.to(TAU, 6.0).ease(presets::LINEAR).await;
    all((g.x().to(-305.0,1.6), note.opacity().to(0.0,0.7))).await;
    // Reset by exactly one full turn; the point is unchanged.
    angle.set(0.0);
    let px = g.add(Line::new().points([(0.0,0.0),(r,0.0)]).stroke(color!("#73d8ec")).line_width(3.0).opacity(0.0));
    let a = angle.clone(); px.points().bind(move || vec![Vec2::new(0.0,0.0),Vec2::new(r*a.get().cos(),0.0)]);
    let py = g.add(Line::new().points([(r,0.0),(r,0.0)]).stroke(color!("#c7a2f5")).line_width(3.0).opacity(0.0));
    let a = angle.clone(); py.points().bind(move || vec![Vec2::new(r*a.get().cos(),0.0),Vec2::new(r*a.get().cos(),-r*a.get().sin())]);
    let graphs = view.add(Group::new().opacity(0.0).x(250.0).y(10.0));
    let width = 470.0;
    for (y, c, label) in [(-94.0, color!("#73d8ec"), "cos t"), (94.0, color!("#c7a2f5"), "sin t")] {
        graphs.add(Line::new().points([(-width/2.0,y),(width/2.0,y)]).stroke(color!("#425269")).line_width(1.4));
        graphs.add(Line::new().points([(-width/2.0,y-65.0),(-width/2.0,y+65.0)]).stroke(color!("#425269")).line_width(1.4));
        graphs.add(Math::new(label).size(32.0).color(c).x(-155.0).y(y-75.0));
    }
    let real = graphs.add(Line::new().points([(0.0,0.0),(0.0,0.0)]).stroke(color!("#73d8ec")).line_width(3.0));
    let a = angle.clone();
    real.points().bind(move || {
        let t = a.get();
        (0..=100).map(|j| {let u=t*j as f64/100.0;Vec2::new(-width/2.0+width*u/TAU,-94.0-56.0*u.cos())}).collect()
    });
    let imag = graphs.add(Line::new().points([(0.0,0.0),(0.0,0.0)]).stroke(color!("#c7a2f5")).line_width(3.0));
    let a = angle.clone();
    imag.points().bind(move || {
        let t = a.get();
        (0..=100).map(|j| {let u=t*j as f64/100.0;Vec2::new(-width/2.0+width*u/TAU,94.0-56.0*u.sin())}).collect()
    });
    let real_dot = graphs.add(Circle::new().size(10.0).fill(color!("#73d8ec")));
    let imag_dot = graphs.add(Circle::new().size(10.0).fill(color!("#c7a2f5")));
    for (p, base, sine) in [(real_dot,-94.0,false),(imag_dot,94.0,true)] {
        let a=angle.clone(); p.x().bind(move || -width/2.0+width*a.get()/TAU);
        let a=angle.clone(); p.y().bind(move || base-56.0*if sine {a.get().sin()} else {a.get().cos()});
    }
    let formula = view.add(Math::new("e^(i t) = cos t + i sin t").size(62.0).color(color!("#e9c77f")).y(267.0).opacity(0.0));
    all((graphs.opacity().to(1.0,1.2),px.opacity().to(1.0,1.2),py.opacity().to(1.0,1.2),formula.opacity().to(1.0,1.2))).await;
    angle.to(TAU, 8.5).ease(presets::LINEAR).await;
    wait_for(1.6).await;
    all((graphs.opacity().to(0.0,1.2),g.opacity().to(0.0,1.2),hero.opacity().to(0.0,0.8),formula.y().to(-258.0,1.5))).await;
}

// A cylinder is aligned along +Y. Rotate each short segment into its chord.
fn rod(parent: &Object3d, a: Vec3, b: Vec3, diameter: f64, c: Color) {
    let (dx,dy,dz)=(b.x-a.x,b.y-a.y,b.z-a.z);
    let len=(dx*dx+dy*dy+dz*dz).sqrt();
    if len < 1e-9 {return;}
    let axis_len=(dx*dx+dz*dz).sqrt();
    let rot=if axis_len < 1e-9 {
        if dy>=0.0 {Quat::IDENTITY} else {Quat::from_axis_angle(Vec3::new(1.0,0.0,0.0),180.0)}
    } else {
        Quat::from_axis_angle(Vec3::new(dz/axis_len,0.0,-dx/axis_len),(dy/len).clamp(-1.0,1.0).acos().to_degrees())
    };
    let o=parent.add(Mesh::cylinder().scale3(Vec3::new(diameter,len*1.08,diameter))
        .color(c).emissive(c).roughness(0.35)
        .position(Vec3::new((a.x+b.x)/2.0,(a.y+b.y)/2.0,(a.z+b.z)/2.0)));
    o.orientation().set(rot);
}
fn helix_point(t:f64)->Vec3 {
    Vec3::new(1.18*t.cos(),-1.85+3.7*t/(2.0*TAU),1.18*t.sin())
}

#[oxframe::scene]
async fn helix(view: View) {
    backdrop(view);
    let formula=view.add(Math::new("e^(i t) = cos t + i sin t").size(62.0).color(color!("#e9c77f")).y(-258.0));
    transition(Transition::fade().duration(1.15)).await;
    let world=Scene3d::new();
    world.camera().position().set(Vec3::new(5.4,2.2,7.8));
    let sculpture=world.add(Mesh::group());
    let strand=sculpture.add(Mesh::group());
    let re=sculpture.add(Mesh::group());
    let im=sculpture.add(Mesh::group());
    // The two thin traces lie in parallel projection planes.
    for j in 0..80 {
        let a=helix_point(2.0*TAU*j as f64/80.0);
        let b=helix_point(2.0*TAU*(j+1) as f64/80.0);
        rod(&strand,a,b,0.092,color!("#d9b76b"));
        rod(&re,Vec3::new(a.x,a.y,-1.55),Vec3::new(b.x,b.y,-1.55),0.025,color!("#408fa1"));
        rod(&im,Vec3::new(-1.55,a.y,a.z),Vec3::new(-1.55,b.y,b.z),0.025,color!("#8969b0"));
    }
    rod(&sculpture,Vec3::new(0.0,-2.05,0.0),Vec3::new(0.0,2.05,0.0),0.025,color!("#43536a"));
    re.opacity().set(0.0); im.opacity().set(0.0);
    let tip=sculpture.add(Mesh::sphere().scale(0.16).color(color!("#fff0b9")).emissive(color!("#e9c77f")).position(helix_point(0.0)));
    let progress=Signal::new(0.0_f64);
    let p=progress.clone();tip.position().bind(move || helix_point(p.get()));
    let frame=view.add(View3d::new(&world).size(670.0,420.0).x(-170.0).y(15.0).opacity(0.0));
    let label=view.add(Txt::new().text("Rotation").font_size(43).weight(700).fill(color!("#f4eee0")).x(355.0).y(-74.0).opacity(0.0));
    let label2=view.add(Txt::new().text("through time").font_size(37).fill(color!("#f4eee0")).x(355.0).y(-18.0).opacity(0.0));
    let label3=view.add(Txt::new().text("becomes a helix.").font_size(32).fill(color!("#e9c77f")).x(355.0).y(38.0).opacity(0.0));
    let axis=view.add(Math::new("(cos t, sin t, t)").size(32.0).color(color!("#9ba9ba")).x(355.0).y(135.0).opacity(0.0));
    all((frame.opacity().to(1.0,1.8),label.opacity().to(1.0,1.8),label2.opacity().to(1.0,1.8),label3.opacity().to(1.0,1.8),axis.opacity().to(1.0,1.8))).await;
    all((sculpture.rotation().to(Vec3::new(0.0,100.0,0.0),9.5).ease(presets::LINEAR),
        progress.to(2.0*TAU,9.5).ease(presets::LINEAR),re.opacity().to(0.75,3.0),im.opacity().to(0.75,3.0),
        world.camera().position().to(Vec3::new(4.3,2.7,7.5),9.5))).await;
    all((frame.opacity().to(0.0,1.5),label.opacity().to(0.0,1.0),label2.opacity().to(0.0,1.0),label3.opacity().to(0.0,1.0),axis.opacity().to(0.0,1.0))).await;
    wait_for(0.3).await;
    // Keep the formula in the same location across the blend.
    let _=formula;
}

#[oxframe::scene]
async fn finale(view: View) {
    backdrop(view);
    let formula=view.add(Math::new("e^(i t) = cos t + i sin t").size(62.0).color(color!("#e9c77f")).y(-258.0));
    transition(Transition::fade().duration(1.15)).await;
    let group=view.add(Group::new().x(-295.0).y(10.0).opacity(0.0));
    let r=146.0;
    group.add(Line::new().points(ring_points(r)).stroke(color!("#51b8b7")).line_width(2.6));
    group.add(Line::new().points([(-180.0,0.0),(180.0,0.0)]).stroke(color!("#425269")).line_width(1.4));
    group.add(Line::new().points([(0.0,-180.0),(0.0,180.0)]).stroke(color!("#425269")).line_width(1.4));
    let t=Signal::new(0.0_f64);
    let v=group.add(Line::new().points([(0.0,0.0),(r,0.0)]).stroke(color!("#e9c77f")).line_width(3.0).end_arrow(true));
    let a=t.clone();v.points().bind(move || vec![Vec2::new(0.0,0.0),Vec2::new(r*a.get().cos(),-r*a.get().sin())]);
    let p=group.add(Circle::new().size(13.0).fill(color!("#f9dfa6")).glow(color!("#e9c77f"),10.0));
    let a=t.clone();p.x().bind(move || r*a.get().cos());let a=t.clone();p.y().bind(move || -r*a.get().sin());
    let minus=group.add(Math::new("-1").size(28.0).color(color!("#f4eee0")).x(-158.0).y(30.0).opacity(0.0));
    let half=view.add(Math::new("t = pi").size(62.0).color(color!("#f4eee0")).x(270.0).y(-55.0).opacity(0.0));
    all((group.opacity().to(1.0,1.1),half.opacity().to(1.0,1.1))).await;
    t.to(PI,3.3).ease(presets::LINEAR).await;
    let identity=view.add(Math::new("e^(i pi) = -1").size(70.0).color(color!("#e9c77f")).x(270.0).y(55.0).opacity(0.0));
    all((identity.opacity().to(1.0,0.9),minus.opacity().to(1.0,0.9))).await;
    wait_for(1.0).await;
    all((group.opacity().to(0.0,1.2),half.opacity().to(0.0,1.2),formula.opacity().to(0.0,1.2),identity.opacity().to(0.0,1.2))).await;
    let final_eq=view.add(Math::new("e^(i pi) + 1 = 0").size(126.0).color(color!("#e9c77f")).y(-10.0).opacity(0.0));
    let closing=view.add(Txt::new().text("One identity. A whole circle of ideas.").font_size(30).fill(color!("#f4eee0")).y(144.0).opacity(0.0));
    all((final_eq.opacity().to(1.0,1.5),closing.opacity().to(1.0,2.0))).await;
    wait_for(2.4).await;
    all((final_eq.opacity().to(0.0,1.0),closing.opacity().to(0.0,1.0))).await;
}
fn main() {
    let mut theme=Theme::dark().accent(color!("#e9c77f"));
    theme.background=color!("#0b1020");
    theme.text=color!("#f4eee0");
    theme.muted=color!("#9ba9ba");
    Project::new("example-0002-euler-circle").size(1280,720).fps(30).theme(theme)
        .scene(circle_and_waves()).scene(helix()).scene(finale()).main();
}
