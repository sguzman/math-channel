//! A deliberately tautological longitudinal study. All data are defined, not observed.
use oxframe::prelude::*;

fn ink()->Color {color!("#20262e")}
fn blue()->Color {color!("#2346a1")}
fn gray()->Color {color!("#747980")}
fn grid()->Color {color!("#dbdce0")}
fn red()->Color {color!("#a82828")}

#[oxframe::scene]
async fn study(view:View) {
    // A single academic figure. The two axes use exactly the same scale.
    view.add(Txt::new().text("ON THE LINEAR GROWTH OF PROGRESS").font_size(35).weight(700).fill(ink()).y(-456.0));
    view.add(Txt::new().text("Fig. 1. Longitudinal analysis (preliminary)").font_size(24).fill(gray()).y(-413.0));
    view.add(Line::new().points([(-465.0,-379.0),(465.0,-379.0)]).stroke(ink()).line_width(1.4));

    let plot=view.add(Group::new().x(-35.0).y(-24.0));
    let l=-325.0;let bottom=315.0;let step=52.0;
    for j in 0..=12 {
        let a=l+step*j as f64;let h=bottom-step*j as f64;
        plot.add(Line::new().points([(a,bottom),(a,bottom-12.0*step)]).stroke(grid()).line_width(1.0));
        plot.add(Line::new().points([(l,h),(l+12.0*step,h)]).stroke(grid()).line_width(1.0));
        if j%2==0 {
            plot.add(Txt::new().text(format!("{j}")).font_size(22).fill(ink()).x(a).y(bottom+22.0));
            plot.add(Txt::new().text(format!("{j}")).font_size(22).fill(ink()).x(l-28.0).y(h));
        }
    }
    plot.add(Line::new().points([(l,bottom-12.0*step-10.0),(l,bottom),(l+12.0*step+10.0,bottom)]).stroke(ink()).line_width(2.2));
    view.add(Txt::new().text("Cumulative progress (years)").font_size(24).fill(ink()).x(-20.0).y(-355.0));
    view.add(Txt::new().text("Elapsed time (years)").font_size(25).fill(ink()).x(-35.0).y(360.0));

    let t=Signal::new(0.0_f64);
    let trend=plot.add(Line::new().points([(l,bottom),(l,bottom)]).stroke(blue()).line_width(3.6));
    let s=t.clone();trend.points().bind(move||{let n=s.get().floor().min(12.0);vec![Vec2::new(l,bottom),Vec2::new(l+step*n,bottom-step*n)]});
    for j in 0..=12 {
        let v=j as f64;
        let point=plot.add(Circle::new().size(12.0).fill(blue()).x(l+step*v).y(bottom-step*v).opacity(0.0));
        let s=t.clone();point.opacity().bind(move||if s.get()>=v {1.0}else{0.0});
    }
    let cursor=plot.add(Circle::new().size(23.0).fill(red()));
    let s=t.clone();cursor.x().bind(move||l+step*s.get().floor().min(12.0));
    let s=t.clone();cursor.y().bind(move||bottom-step*s.get().floor().min(12.0));
    // The exact counts update in lockstep; no empirical observations are claimed.
    let counter=view.add(Group::new().x(295.0).y(245.0));
    counter.add(Rect::new().width(305.0).height(88.0).fill(color!("#f3f2ec")).stroke(grid()).line_width(1.0));
    counter.add(Txt::new().text("YEARS OBSERVED").font_size(18).weight(600).fill(gray()).y(-22.0));
    for j in 0..=12 {
        let v=j as f64;
        let label=counter.add(Txt::new().text(format!("{j:02} / 12")).font_size(31).weight(700).fill(ink()).y(12.0).opacity(0.0));
        let s=t.clone();label.opacity().bind(move||if s.get().floor().min(12.0)==v {1.0}else{0.0});
    }

    let box_=view.add(Group::new().x(274.0).y(-242.0).opacity(0.0));
    box_.add(Rect::new().width(298.0).height(169.0).fill(color!("#f7f6f1")).stroke(ink()).line_width(1.0));
    box_.add(Txt::new().text("MODEL DIAGNOSTICS").font_size(18).weight(600).fill(ink()).y(-58.0));
    box_.add(Math::new("R^2 = 1.0000").size(34.0).color(blue()).y(-16.0));
    box_.add(Math::new("dif P / dif t = 1").size(31.0).color(ink()).y(29.0));
    box_.add(Txt::new().text("Residual variance: 0").font_size(18).fill(gray()).y(62.0));

    let footer=view.add(Txt::new().text("Collecting additional years...").font_size(26).fill(gray()).y(435.0));
    let fit=view.add(Group::new().x(-236.0).y(-205.0).opacity(0.0));
    fit.add(Rect::new().width(239.0).height(89.0).fill(color!("#f7f6f1")).stroke(grid()).line_width(1.0));
    fit.add(Math::new("P(t)=t").size(46.0).color(ink()).y(-5.0));
    fit.add(Txt::new().text("EXACT FIT").font_size(17).weight(700).fill(blue()).y(29.0));

    wait_for(0.45).await;
    t.to(12.01,7.2).ease(presets::LINEAR).await;
    all((fit.opacity().to(1.0,0.35),box_.opacity().to(1.0,0.35),footer.opacity().to(0.0,0.25))).await;
    let result=view.add(Txt::new().text("Further study is unlikely to change the conclusion.").font_size(28).weight(600).fill(ink()).y(435.0).opacity(0.0));
    result.opacity().to(1.0,0.3).await;
    wait_for(2.65).await;
    // Intentional hard loop boundary, not a second unrelated scene.
}
fn main() {
    let mut theme=Theme::dark().accent(blue());
    theme.background=color!("#f7f6f1");theme.text=ink();theme.muted=gray();
    Project::new("example-0005-progress").size(1080,1080).fps(60).theme(theme).scene(study()).main();
}
