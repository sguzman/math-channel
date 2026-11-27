use oxframe::prelude::*;

#[oxframe::scene]
async fn formula(view: View) {
    let identity = view.add(Math::new("sin(x)^2 + cos(x)^2 = 1").size(64.0));
    identity.fade_in().await;
    wait_for(1.0).await;
}

#[oxframe::scene]
async fn plot(view: View) {
    let axes = view.add(
        Axes::new()
            .x_range(-3.0..3.0)
            .y_range(-1.5..1.5)
            .width(pct(80))
            .height(pct(65)),
    );
    let curve = axes.plot(|x| x.sin()).stroke(Accent);
    curve.draw_on().await;
    let x = Signal::new(-3.0);
    axes.point_on(&curve, x.clone());
    x.to(3.0, 2.0).await;
    wait_for(0.5).await;
}

fn main() {
    Project::new("engine-trial")
        .size(960, 540)
        .fps(30)
        .scene(formula())
        .scene(plot())
        .main();
}
