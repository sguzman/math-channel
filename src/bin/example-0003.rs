//! Example 0003: Fourier series, epicycles, spectrum and convergence.
use oxframe::prelude::*;
use std::f64::consts::{PI, TAU};
fn gold()->Color {color!("#efcd87")}
fn ivory()->Color {color!("#f5eee4")}
fn muted_color()->Color {color!("#a5a6bb")}
fn grid()->Color {color!("#394254")}
fn colors()->[Color;5] {[color!("#71e1c1"),color!("#78c9f2"),color!("#a9a2f9"),color!("#e3a1db"),color!("#efcd87")]}

fn amp(j:usize)->f64 {4.0/(PI*(2*j+1) as f64)}
fn gain(n:f64,j:usize)->f64 {(n-j as f64).clamp(0.0,1.0)}
fn sum(t:f64,n:f64)->f64 {
    (0..17).map(|j|gain(n,j)*amp(j)*((2*j+1) as f64*t).sin()).sum()
}
fn endpoint(t:f64,n:f64,k:usize)->Vec2 {
    let mut x=0.0;let mut y=0.0;
    for j in 0..k {let a=100.0*amp(j)*gain(n,j);let p=(2*j+1) as f64*t;x+=a*p.cos();y-=a*p.sin();}
    Vec2::new(x,y)
}
fn backdrop(view:View) {
    for r in [430.0_f64,500.0,570.0] {
        for side in [-1.0,1.0] {
            let pts=(0..=100).map(|j|{let y=-300.0+600.0*j as f64/100.0;(side*(r*r-y*y).sqrt(),y)}).collect::<Vec<_>>();
            view.add(Line::new().points(pts).stroke(grid()).line_width(1.0).opacity(0.24));
        }
    }
}
#[oxframe::scene]
async fn orchestra(view:View) {
    backdrop(view);
    let small=view.add(Txt::new().text("T H E   M A T H E M A T I C S   O F   H A R M O N Y").font_size(20).weight(500).fill(muted_color()).y(-160.0).opacity(0.0));
    let title=view.add(Txt::new().text("An orchestra of circles").font_size(66).weight(700).fill(ivory()).y(-77.0).opacity(0.0));
    let hero=view.add(Math::new("sin t + 1/3 sin(3t) + 1/5 sin(5t) + dots").size(53.0).color(gold()).y(62.0).opacity(0.0));
    all((small.opacity().to(1.0,1.2),title.opacity().to(1.0,1.5),hero.opacity().to(1.0,2.0))).await;
    wait_for(2.0).await;
    all((small.opacity().to(0.0,0.8),title.opacity().to(0.0,0.8),hero.y().to(-264.0,1.6),hero.scale().to(0.85,1.6))).await;
    let phase=Signal::new(0.0_f64);
    let count=Signal::new(1.0_f64);
    let mech=view.add(Group::new().x(-310.0).y(0.0).opacity(0.0));
    mech.add(Line::new().points([(-235.0,0.0),(235.0,0.0)]).stroke(grid()).line_width(1.2));
    mech.add(Line::new().points([(0.0,-235.0),(0.0,235.0)]).stroke(grid()).line_width(1.2));
    for j in 0..5 {
        let ring=mech.add(Line::new().points([(0.0,0.0),(1.0,0.0)]).stroke(colors()[j]).line_width(1.8).opacity(0.68));
        let (p,n)=(phase.clone(),count.clone());
        ring.points().bind(move || {
            let q=endpoint(p.get(),n.get(),j);let r=100.0*amp(j)*gain(n.get(),j);
            (0..=100).map(|i|{let t=TAU*i as f64/100.0;Vec2::new(q.x+r*t.cos(),q.y+r*t.sin())}).collect()
        });
        let arm=mech.add(Line::new().points([(0.0,0.0),(1.0,0.0)]).stroke(colors()[j]).line_width(3.0));
        let (p,n)=(phase.clone(),count.clone());
        arm.points().bind(move || vec![endpoint(p.get(),n.get(),j),endpoint(p.get(),n.get(),j+1)]);
    }
    let tip=mech.add(Circle::new().size(12.0).fill(ivory()).glow(gold(),12.0));
    let (p,n)=(phase.clone(),count.clone());tip.x().bind(move||endpoint(p.get(),n.get(),5).x);
    let (p,n)=(phase.clone(),count.clone());tip.y().bind(move||endpoint(p.get(),n.get(),5).y);
    let graph=view.add(Group::new().x(280.0).y(0.0).opacity(0.0));
    let w=460.0;
    for y in [-100.0,0.0,100.0] {
        graph.add(Line::new().points([(-w/2.0,y),(w/2.0,y)]).stroke(grid()).line_width(1.2));
    }
    graph.add(Line::new().points([(-w/2.0,-133.0),(-w/2.0,133.0)]).stroke(grid()).line_width(1.2));
    let target=graph.add(Group::new().opacity(0.0));
    for (x0,x1,y) in [(-230.0,0.0,-100.0),(0.0,230.0,100.0)] {
        target.add(Line::new().points([(x0,y),(x1,y)]).stroke(muted_color()).line_width(2.0).opacity(0.65));
    }
    let curve=graph.add(Line::new().points([(0.0,0.0),(1.0,0.0)]).stroke(gold()).line_width(3.5));
    let (p,n)=(phase.clone(),count.clone());curve.points().bind(move|| {
        let end=p.get();
        (0..=320).map(|j|{let t=end*j as f64/320.0;Vec2::new(-w/2.0+w*t/TAU,-100.0*sum(t,n.get()))}).collect()
    });
    let marker=graph.add(Circle::new().size(12.0).fill(ivory()).glow(gold(),10.0));
    let p=phase.clone();marker.x().bind(move||-w/2.0+w*p.get()/TAU);
    let (p,n)=(phase.clone(),count.clone());marker.y().bind(move||-100.0*sum(p.get(),n.get()));
    // End of arm and graph marker share the same vertical coordinate.
    let connector=view.add(Line::new().points([(0.0,0.0),(1.0,0.0)]).stroke(color!("#73869c")).line_width(1.3).opacity(0.0));
    let (p,n)=(phase.clone(),count.clone());connector.points().bind(move||{
        let q=endpoint(p.get(),n.get(),5);
        vec![Vec2::new(-310.0+q.x,q.y),Vec2::new(280.0-w/2.0+w*p.get()/TAU,q.y)]
    });
    let label=view.add(Txt::new().text("One rotation. One wave.").font_size(32).fill(ivory()).y(272.0).opacity(0.0));
    all((mech.opacity().to(1.0,1.2),graph.opacity().to(1.0,1.2),connector.opacity().to(0.45,1.2),label.opacity().to(1.0,1.2))).await;
    phase.to(TAU,6.0).ease(presets::LINEAR).await;
    all((label.opacity().to(0.0,0.8),target.opacity().to(1.0,1.0))).await;
    // The trace remains visible while the extra harmonics grow into place.
    let label2=view.add(Txt::new().text("Add smaller circles that rotate faster.").font_size(31).fill(ivory()).y(272.0).opacity(0.0));
    all((count.to(5.0,5.0),label2.opacity().to(1.0,1.0))).await;
    phase.set(0.0);
    phase.to(TAU,7.0).ease(presets::LINEAR).await;
    wait_for(1.0).await;
    all((mech.opacity().to(0.0,1.4),graph.opacity().to(0.0,1.4),connector.opacity().to(0.0,1.4),label2.opacity().to(0.0,1.0),hero.opacity().to(0.0,1.4))).await;
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

fn spectral_point(t:f64,j:usize)->Vec3 {
    Vec3::new(-2.2+4.4*t/TAU,amp(j)*((2*j+1) as f64*t).sin()*0.85,-1.5+0.6*j as f64)
}
#[oxframe::scene]
async fn spectrum(view:View) {
    backdrop(view);
    let title=view.add(Txt::new().text("Separate the frequencies.").font_size(48).weight(700).fill(ivory()).y(-262.0));
    transition(Transition::fade().duration(1.2)).await;
    let world=Scene3d::new();
    world.camera().position().set(Vec3::new(5.0,3.3,8.4));
    let sculpture=world.add(Mesh::group());
    for j in 0..5 {
        for k in 0..120 {
            let a=spectral_point(TAU*k as f64/120.0,j);
            let b=spectral_point(TAU*(k+1) as f64/120.0,j);
            rod(&sculpture,a,b,0.038,colors()[j]);
        }
        rod(&sculpture,Vec3::new(-2.2,0.0,-1.5+0.6*j as f64),Vec3::new(2.2,0.0,-1.5+0.6*j as f64),0.013,grid());
    }
    for k in 0..180 {
        let a=TAU*k as f64/180.0;let b=TAU*(k+1) as f64/180.0;
        rod(&sculpture,Vec3::new(-2.2+4.4*a/TAU,0.85*sum(a,5.0),1.85),
            Vec3::new(-2.2+4.4*b/TAU,0.85*sum(b,5.0),1.85),0.065,gold());
    }
    let frame=view.add(View3d::new(&world).size(760.0,440.0).x(-170.0).y(8.0).opacity(0.0));
    let bars=view.add(Group::new().x(355.0).y(8.0).opacity(0.0));
    bars.add(Txt::new().text("ODD HARMONICS").font_size(20).fill(muted_color()).y(-166.0));
    for j in 0..5 {
        let y=-108.0+j as f64*52.0;
        bars.add(Math::new(format!("{}",2*j+1)).size(29.0).color(colors()[j]).x(-109.0).y(y));
        let width=156.0/(2*j+1) as f64;
        bars.add(Rect::new().width(width).height(12.0).fill(colors()[j]).x(-75.0+width/2.0).y(y));
    }
    bars.add(Txt::new().text("Relative amplitude").font_size(21).fill(muted_color()).y(157.0));
    let note=view.add(Txt::new().text("Each frequency contributes its own wave.").font_size(29).fill(ivory()).y(270.0).opacity(0.0));
    all((frame.opacity().to(1.0,1.6),bars.opacity().to(1.0,1.6),note.opacity().to(1.0,1.6))).await;
    all((sculpture.rotation().to(Vec3::new(0.0,105.0,0.0),10.0).ease(presets::LINEAR),
         world.camera().position().to(Vec3::new(3.5,3.6,8.0),10.0))).await;
    all((frame.opacity().to(0.0,1.2),bars.opacity().to(0.0,1.2),note.opacity().to(0.0,1.2),title.opacity().to(0.0,1.2))).await;
}

#[oxframe::scene]
async fn convergence(view:View) {
    backdrop(view);
    transition(Transition::fade().duration(1.2)).await;
    let title=view.add(Txt::new().text("From smooth waves to a sharp edge").font_size(45).weight(700).fill(ivory()).y(-265.0).opacity(0.0));
    let formula=view.add(Math::new("4/pi sum_(k=0)^infinity sin((2k+1)t)/(2k+1)").size(60.0).color(gold()).y(262.0).opacity(0.0));
    let plot=view.add(Group::new().y(-5.0).opacity(0.0));
    let n=Signal::new(1.0_f64);
    for y in [-130.0,0.0,130.0] {plot.add(Line::new().points([(-470.0,y),(470.0,y)]).stroke(grid()).line_width(1.2));}
    plot.add(Line::new().points([(-470.0,-166.0),(-470.0,166.0)]).stroke(grid()).line_width(1.2));
    for (a,b,y) in [(-470.0,0.0,-130.0),(0.0,470.0,130.0)] {
        plot.add(Line::new().points([(a,y),(b,y)]).stroke(muted_color()).line_width(2.0).opacity(0.62));
    }
    let wave=plot.add(Line::new().points([(0.0,0.0),(1.0,0.0)]).stroke(gold()).line_width(3.5));
    let c=n.clone();wave.points().bind(move|| {
        (0..=960).map(|j|{let t=TAU*j as f64/960.0;Vec2::new(-470.0+940.0*t/TAU,-130.0*sum(t,c.get()))}).collect()
    });
    let label=view.add(Txt::new().text("1 harmonic").font_size(27).fill(ivory()).y(-195.0).opacity(0.0));
    all((title.opacity().to(1.0,1.2),plot.opacity().to(1.0,1.2),label.opacity().to(1.0,1.2),formula.opacity().to(1.0,1.2))).await;
    wait_for(1.1).await;
    label.opacity().to(0.0,0.5).await;
    let l2=view.add(Txt::new().text("5 harmonics").font_size(27).fill(ivory()).y(-195.0).opacity(0.0));
    all((n.to(5.0,3.0),l2.opacity().to(1.0,1.0))).await;
    wait_for(1.0).await;
    l2.opacity().to(0.0,0.5).await;
    let l3=view.add(Txt::new().text("17 harmonics").font_size(27).fill(ivory()).y(-195.0).opacity(0.0));
    all((n.to(17.0,4.0),l3.opacity().to(1.0,1.0))).await;
    wait_for(1.6).await;
    all((plot.opacity().to(0.0,1.5),title.opacity().to(0.0,1.0),l3.opacity().to(0.0,1.0),formula.y().to(-45.0,1.8),formula.scale().to(1.14,1.8))).await;
    let close=view.add(Txt::new().text("Complexity is a harmony of simple things.").font_size(32).fill(ivory()).y(110.0).opacity(0.0));
    let note=view.add(Txt::new().text("The Fourier series of a square wave").font_size(24).fill(muted_color()).y(170.0).opacity(0.0));
    all((close.opacity().to(1.0,1.6),note.opacity().to(1.0,1.6))).await;
    wait_for(2.5).await;
    all((formula.opacity().to(0.0,1.3),close.opacity().to(0.0,1.3),note.opacity().to(0.0,1.3))).await;
}
fn main() {
    let mut theme=Theme::dark().accent(gold());
    theme.background=color!("#0c111c");theme.text=ivory();theme.muted=muted_color();
    Project::new("example-0003-fourier-orchestra").size(1280,720).fps(30).theme(theme)
        .scene(orchestra()).scene(spectrum()).scene(convergence()).main();
}
