// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of a custom drawing widget.
//! We draw an image, some text, a shape, and a curve.

use druid::{Vec2, WidgetExt};
use druid::kurbo::{BezPath, Circle};
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder, LineJoin};
use druid::widget::{Flex, Scroll, prelude::*, Container, SizedBox, Slider, Button, LabelText, Label};
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc, kurbo::Line, piet::StrokeStyle
};
use std::{vec, boxed};

mod sim;

#[derive(Clone, PartialEq)]
struct LinkageShow {
    lines:  Vec<[Vec2; 2]>,
    points: Vec<Vec2>,
    scale: f64,
}

impl Data for LinkageShow {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

struct CustomWidget;

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget<LinkageShow> for CustomWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut LinkageShow, _env: &Env) {
        match event {
            Event::MouseDown(mouse_event) => {
                println!("trigged mouse event");
                let draw_scale = ctx.size().min_side();
                data.points.push((mouse_event.pos.x / draw_scale, mouse_event.pos.y / draw_scale).into());
                ctx.request_paint();
            },
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &LinkageShow,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &LinkageShow, _data: &LinkageShow, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &LinkageShow,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &LinkageShow, _env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::grey8(0x30));

        let draw_scale = ctx.size().min_side();

        for line in &data.lines {
            let line = line.map(|i|i*data.scale*draw_scale);
            let line_shape = Line::new(
                (line[0].x, line[0].y),
                (line[1].x, line[1].y),
            );
            let stroke_color = Color::rgb8(0, 128, 128);
            let mut stroke_style = StrokeStyle::new();
            stroke_style.set_line_cap(druid::piet::LineCap::Round);
            ctx.stroke_styled(line_shape, &stroke_color, 3.0, &stroke_style);
        }

        for point in &data.points {
            let point = point.clone()*data.scale*draw_scale;
            let circle = Circle::new((point.x, point.y), 5.0);
            let stroke_color = Color::rgb8(0, 0, 128);
            ctx.fill(circle, &stroke_color)
        }

        // let line = Line::new((size.width * 0.2, size.height * 0.9), (size.width * 0.8, size.height * 0.9));
        // let stroke_color = Color::rgb8(0, 128, 128);
        // let mut stroke_style = StrokeStyle::new();
        // stroke_style.set_line_cap(druid::piet::LineCap::Butt);
        // ctx.stroke_styled(line, &stroke_color, 20.0, &stroke_style);

        // let circle = Circle::new((size.width * 0.5, size.height * 0.5), 20.0);
        // let stroke_color = Color::rgb8(0, 0, 128);
        // ctx.fill(circle, &stroke_color)

    }
}

fn make_widget() -> impl Widget<LinkageShow> {
    Flex::row()
    .with_flex_child(
        Flex::column()
        .with_flex_child(Container::new(CustomWidget{}).expand(), 1.0)
        .with_child(
            Flex::row()
            .with_child(Label::new(|data: &LinkageShow, _env: &_| format!("points num : {}", data.points.len())))
            .with_child(
                Button::new("Clear Point").on_click(|ctx: &mut EventCtx, data: &mut LinkageShow, _env: &Env| {
                    data.points.clear();
                    ctx.request_update();
                })
            )   
        ), 1.0
    )
    .with_child(
        Scroll::new(
            Flex::column()
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0x80)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0xb0)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0x80)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0xb0)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0x80)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0xb0)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0x80)).rounded(10.0))
            .with_child(Container::new(SizedBox::empty().width(100.0).height(200.0)).background(Color::grey8(0xb0)).rounded(10.0))
        )
    )
}

pub fn main() {
    let p: Vec<Vec2> = vec![(0.2, 0.2).into(), (0.6, 0.2).into(), (0.4, 0.6).into(), (0.8, 0.6).into()];
    let linkage = LinkageShow {
        lines: Vec::from([[p[0], p[1]], [p[1], p[2]], [p[2], p[3]], [p[0],p[3]], [p[0], p[2]]]),
        points: Vec::from(p),
        scale: 1.0,
    };
    let window = WindowDesc::new(make_widget)
    .window_size(Size {
        width: 800.0,
        height: 800.0,
    })
    .title(LocalizedString::new("Fancy Colors"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(linkage)
        .expect("launch failed");
}
