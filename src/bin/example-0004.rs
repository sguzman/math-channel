//! Gaussian integral — premium production sample, native 1440p60.
use oxframe::prelude::*;
use std::f64::consts::{PI,TAU};
fn gold()->Color {color!("#edca8e")}
fn ivory()->Color {color!("#f6f0e7")}
fn teal()->Color {color!("#68d5c5")}
fn blue()->Color {color!("#8ba8ee")}
fn muted_color()->Color {color!("#a3b1c8")}
fn grid()->Color {color!("#303e54")}
fn gaussian(x:f64)->f64 {(-x*x).exp()}
fn bell(r:f64)->f64 {(-r*r).exp()}
fn backdrop(view:View) {
    for y in [-630.0,630.0] {
        view.add(Line::new().points([(-1120.0,y),(1120.0,y)]).stroke(grid()).line_width(2.0));
        for x in [-1120.0,1120.0] {view.add(Circle::new().size(7.0).fill(gold()).x(x).y(y));}
    }
}
fn curve_pts(w:f64,h:f64)->Vec<(f64,f64)> {
    (0..=240).map(|j| {let x=-3.3+6.6*j as f64/240.0;(w*x/6.6,-h*gaussian(x))}).collect()
}
#[oxframe::scene]
async fn opening(view:View) {
    backdrop(view);
    let kicker=view.add(Txt::new().text("A N   I N T E G R A L   I N   D I S G U I S E").font_size(38).weight(500).fill(muted_color()).y(-430.0).opacity(0.0));
    let headline=view.add(Txt::new().text("An impossible-looking area").font_size(108).weight(700).fill(ivory()).y(-310.0).opacity(0.0));
    let graph=view.add(Group::new().y(235.0).opacity(0.0));
    let w=1600.0;let h=425.0;
    graph.add(Line::new().points([(-900.0,0.0),(900.0,0.0)]).stroke(grid()).line_width(3.0));
    graph.add(Line::new().points([(0.0,-465.0),(0.0,75.0)]).stroke(grid()).line_width(3.0));
    let extent=Signal::new(0.0_f64);
    let area=graph.add(Group::new());
    // Dense translucent vertical samples are a quadrature illustration, not an exact finite sum.
    for j in 0..160 {
        let x=-3.3+6.6*(j as f64+0.5)/160.0;
        let v=h*gaussian(x);
        let stripe=area.add(Rect::new().width(w/160.0+0.4).height(v).x(w*x/6.6).y(-v/2.0).fill(teal()).opacity(0.0));
        let e=extent.clone();stripe.opacity().bind(move||0.52*((e.get()-x.abs())*7.0).clamp(0.0,1.0));
    }
    let curve=graph.add(Line::new().points(curve_pts(w,h)).stroke(teal()).line_width(7.0).glow(teal(),9.0));
    let equation=view.add(Math::new("I = integral_(-infinity)^infinity e^(-x^2) dif x").size(100.0).color(gold()).y(465.0).opacity(0.0));
    all((kicker.opacity().to(1.0,1.0),headline.opacity().to(1.0,1.5),graph.opacity().to(1.0,1.5))).await;
    curve.draw_on().await;
    all((extent.to(3.5,4.5).ease(presets::LINEAR),equation.opacity().to(1.0,1.8))).await;
    wait_for(2.3).await;
    all((kicker.opacity().to(0.0,1.0),headline.opacity().to(0.0,1.0),graph.opacity().to(0.0,1.5),equation.y().to(-460.0,1.7),equation.scale().to(0.82,1.7))).await;
    let a=view.add(Txt::new().text("There is no elementary antiderivative.").font_size(69).fill(ivory()).y(-110.0).opacity(0.0));
    let c=view.add(Txt::new().text("But the total area is exact.").font_size(76).weight(700).fill(gold()).y(28.0).opacity(0.0));
    let d=view.add(Txt::new().text("The trick is to change dimension.").font_size(48).fill(muted_color()).y(190.0).opacity(0.0));
    a.opacity().to(1.0,1.2).await;
    c.opacity().to(1.0,1.4).await;
    d.opacity().to(1.0,1.3).await;
    wait_for(2.3).await;
    all((a.opacity().to(0.0,1.0),c.opacity().to(0.0,1.0),d.opacity().to(0.0,1.0),equation.opacity().to(0.0,1.2))).await;
}
#[oxframe::scene]
async fn square(view:View) {
    backdrop(view);transition(Transition::fade().duration(1.1)).await;
    let title=view.add(Txt::new().text("01   /   Square the integral").font_size(75).weight(700).fill(ivory()).y(-495.0).opacity(0.0));
    let product=view.add(Math::new("I^2 = (integral_(-infinity)^infinity e^(-x^2) dif x)(integral_(-infinity)^infinity e^(-y^2) dif y)").size(80.0).color(gold()).y(-280.0).opacity(0.0));
    let plane=view.add(Group::new().x(-400.0).y(115.0).opacity(0.0));
    // Exact separable density samples rendered as a heatmap.
    let n=27;let side=710.0;let cell=side/n as f64;
    for i in 0..n {for j in 0..n {
        let x=-3.0+6.0*(i as f64+0.5)/n as f64;
        let y=-3.0+6.0*(j as f64+0.5)/n as f64;
        let density=gaussian(x)*gaussian(y);
        plane.add(Rect::new().width(cell+0.4).height(cell+0.4).x(-side/2.0+cell*(i as f64+0.5)).y(-side/2.0+cell*(j as f64+0.5)).fill(teal()).opacity(0.86*density));
    }}
    for v in [-350.0,-235.0,-117.5,0.0,117.5,235.0,350.0] {
        plane.add(Line::new().points([(v,-355.0),(v,355.0)]).stroke(grid()).line_width(1.6).opacity(0.75));
        plane.add(Line::new().points([(-355.0,v),(355.0,v)]).stroke(grid()).line_width(1.6).opacity(0.75));
    }
    let note=view.add(Txt::new().text("Two independent axes.").font_size(52).fill(ivory()).x(530.0).y(-30.0).opacity(0.0));
    let note2=view.add(Txt::new().text("One circular density.").font_size(55).weight(700).fill(teal()).x(530.0).y(54.0).opacity(0.0));
    let eq=view.add(Math::new("I^2 = integral.double_(RR^2) e^(-(x^2+y^2)) dif A").size(78.0).color(gold()).x(520.0).y(220.0).opacity(0.0));
    all((title.opacity().to(1.0,1.2),product.opacity().to(1.0,1.6))).await;
    wait_for(2.4).await;
    all((plane.opacity().to(1.0,2.0),note.opacity().to(1.0,1.5))).await;
    all((note2.opacity().to(1.0,1.4),eq.opacity().to(1.0,2.0))).await;
    wait_for(2.4).await;
    all((title.opacity().to(0.0,1.0),product.opacity().to(0.0,1.0),plane.opacity().to(0.0,1.3),note.opacity().to(0.0,1.0),note2.opacity().to(0.0,1.0),eq.x().to(0.0,1.6),eq.y().to(-490.0,1.6))).await;
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

fn surf(r:f64,t:f64)->Vec3 {Vec3::new(r*t.cos(),2.2*bell(r),r*t.sin())}
#[oxframe::scene]
async fn surface(view:View) {
    backdrop(view);
    let eq=view.add(Math::new("I^2 = integral.double_(RR^2) e^(-(x^2+y^2)) dif A").size(78.0).color(gold()).y(-490.0));
    transition(Transition::fade().duration(1.1)).await;
    let world=Scene3d::new();world.camera().position().set(Vec3::new(6.0,4.8,8.8));
    let mesh=world.add(Mesh::group());
    // Continuous-looking radial wire sculpture, with filled-looking fine longitude mesh.
    for j in 1..=18 {
        let r=2.65*j as f64/18.0;
        for k in 0..64 {
            let a=surf(r,TAU*k as f64/64.0);let b=surf(r,TAU*(k+1) as f64/64.0);
            let c=if j<8 {gold()} else if j<17 {teal()} else {blue()};
            rod(&mesh,a,b,if j%4==0 {0.023}else{0.013},c);
        }
    }
    for k in 0..24 {
        let t=TAU*k as f64/24.0;
        for j in 0..20 {rod(&mesh,surf(2.65*j as f64/20.0,t),surf(2.65*(j+1) as f64/20.0,t),0.013,teal());}
    }
    for j in 1..=5 {
        let r=0.5*j as f64;
        for k in 0..64 {let a=TAU*k as f64/64.0;let b=TAU*(k+1) as f64/64.0;rod(&mesh,Vec3::new(r*a.cos(),-0.07,r*a.sin()),Vec3::new(r*b.cos(),-0.07,r*b.sin()),0.009,grid());}
    }
    let frame=view.add(View3d::new(&world).size(1550.0,890.0).x(-380.0).y(45.0).opacity(0.0));
    let title=view.add(Txt::new().text("02   /   A Gaussian bell").font_size(43).weight(500).fill(muted_color()).x(745.0).y(-290.0).opacity(0.0));
    let a=view.add(Txt::new().text("Every direction").font_size(64).weight(700).fill(ivory()).x(745.0).y(-105.0).opacity(0.0));
    let c=view.add(Txt::new().text("looks the same.").font_size(64).weight(700).fill(ivory()).x(745.0).y(-15.0).opacity(0.0));
    let r=view.add(Math::new("x^2+y^2=r^2").size(82.0).color(teal()).x(745.0).y(158.0).opacity(0.0));
    all((frame.opacity().to(1.0,2.0),title.opacity().to(1.0,1.5),a.opacity().to(1.0,2.0),c.opacity().to(1.0,2.0),r.opacity().to(1.0,2.0))).await;
    all((mesh.rotation().to(Vec3::new(0.0,112.0,0.0),11.0).ease(presets::LINEAR),world.camera().position().to(Vec3::new(5.2,5.9,8.5),11.0))).await;
    all((frame.opacity().to(0.0,1.5),title.opacity().to(0.0,1.0),a.opacity().to(0.0,1.0),c.opacity().to(0.0,1.0),r.opacity().to(0.0,1.0),eq.opacity().to(0.0,1.5))).await;
}
#[oxframe::scene]
async fn polar(view:View) {
    backdrop(view);transition(Transition::fade().duration(1.1)).await;
    let title=view.add(Txt::new().text("03   /   Change coordinates").font_size(76).weight(700).fill(ivory()).y(-495.0).opacity(0.0));
    let center=view.add(Group::new().x(-485.0).y(32.0).opacity(0.0));
    let cart=center.add(Group::new());
    for v in [-360.0,-270.0,-180.0,-90.0,0.0,90.0,180.0,270.0,360.0] {
        cart.add(Line::new().points([(v,-360.0),(v,360.0)]).stroke(grid()).line_width(2.5));
        cart.add(Line::new().points([(-360.0,v),(360.0,v)]).stroke(grid()).line_width(2.5));
    }
    let rings=center.add(Group::new().opacity(0.0));
    for r in [90.0,180.0,270.0,360.0] {
        rings.add(Line::new().points((0..=160).map(|j|{let t=TAU*j as f64/160.0;(r*t.cos(),r*t.sin())}).collect::<Vec<_>>()).stroke(teal()).line_width(3.0).opacity(0.68));
    }
    for j in 0..12 {let t=TAU*j as f64/12.0;rings.add(Line::new().points([(0.0,0.0),(360.0*t.cos(),360.0*t.sin())]).stroke(grid()).line_width(2.5));}
    let sweep=Signal::new(0.0_f64);
    let wedge=center.add(Group::new().opacity(0.0));
    // Annular sector: the area is (r dr + dr²/2) dtheta, tending to r dr dtheta.
    for j in 0..54 {
        let t=0.72*j as f64/54.0;
        let line=wedge.add(Line::new().points([(225.0*t.cos(),-225.0*t.sin()),(285.0*t.cos(),-285.0*t.sin())]).stroke(gold()).line_width(4.5));
        let a=sweep.clone();line.opacity().bind(move||if t<=a.get(){0.85}else{0.0});
    }
    let right=view.add(Math::new("x^2+y^2=r^2").size(112.0).color(teal()).x(510.0).y(-195.0).opacity(0.0));
    let jac=view.add(Math::new("dif A = r dif r dif theta").size(110.0).color(gold()).x(510.0).y(0.0).opacity(0.0));
    let note=view.add(Txt::new().text("The farther out, the longer the arc.").font_size(42).fill(ivory()).x(510.0).y(175.0).opacity(0.0));
    all((center.opacity().to(1.0,1.4),title.opacity().to(1.0,1.4))).await;
    wait_for(1.6).await;
    all((cart.opacity().to(0.0,2.2),rings.opacity().to(1.0,2.2),right.opacity().to(1.0,1.8))).await;
    all((wedge.opacity().to(1.0,1.0),sweep.to(0.72,3.0),jac.opacity().to(1.0,1.7),note.opacity().to(1.0,1.7))).await;
    wait_for(2.5).await;
    all((center.opacity().to(0.0,1.4),title.opacity().to(0.0,1.1),right.opacity().to(0.0,1.1),jac.opacity().to(0.0,1.1),note.opacity().to(0.0,1.1))).await;
}
#[oxframe::scene]
async fn conclusion(view:View) {
    backdrop(view);transition(Transition::fade().duration(1.1)).await;
    let title=view.add(Txt::new().text("04   /   The integral unlocks").font_size(72).weight(700).fill(ivory()).y(-495.0).opacity(0.0));
    let a=view.add(Math::new("I^2 = integral_0^(2 pi) integral_0^infinity e^(-r^2) r dif r dif theta").size(110.0).color(gold()).y(-270.0).opacity(0.0));
    let b=view.add(Math::new("= 2 pi times [-1/2 e^(-r^2)]_0^infinity").size(108.0).color(ivory()).y(-40.0).opacity(0.0));
    let c=view.add(Math::new("= 2 pi times 1/2 = pi").size(116.0).color(teal()).y(190.0).opacity(0.0));
    let note=view.add(Txt::new().text("The positive square root is the original area.").font_size(46).fill(muted_color()).y(470.0).opacity(0.0));
    all((title.opacity().to(1.0,1.2),a.opacity().to(1.0,1.6))).await;wait_for(2.4).await;
    b.opacity().to(1.0,1.4).await;wait_for(2.4).await;
    c.opacity().to(1.0,1.4).await;note.opacity().to(1.0,1.1).await;wait_for(2.4).await;
    all((title.opacity().to(0.0,1.2),a.opacity().to(0.0,1.2),b.opacity().to(0.0,1.2),c.opacity().to(0.0,1.2),note.opacity().to(0.0,1.2))).await;
    let hero=view.add(Math::new("integral_(-infinity)^infinity e^(-x^2) dif x = sqrt(pi)").size(174.0).color(gold()).y(-35.0).opacity(0.0));
    let subtitle=view.add(Txt::new().text("T H E   G A U S S I A N   I N T E G R A L").font_size(40).weight(500).fill(ivory()).y(255.0).opacity(0.0));
    let line=view.add(Line::new().points([(-340.0,350.0),(340.0,350.0)]).stroke(teal()).line_width(3.0).opacity(0.0));
    all((hero.opacity().to(1.0,2.0),subtitle.opacity().to(1.0,2.0),line.opacity().to(0.65,2.0))).await;
    wait_for(3.0).await;
    all((hero.opacity().to(0.0,1.5),subtitle.opacity().to(0.0,1.5),line.opacity().to(0.0,1.5))).await;
}
fn main() {
    let mut theme=Theme::dark().accent(gold());theme.background=color!("#0b1322");theme.text=ivory();theme.muted=muted_color();
    Project::new("example-0004-gaussian-integral").size(2560,1440).fps(60).theme(theme)
        .scene(opening()).scene(square()).scene(surface()).scene(polar()).scene(conclusion()).main();
}
