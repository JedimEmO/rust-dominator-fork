#![feature(trace_macros)]
#![feature(log_syntax)]

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate dominator;

use std::rc::Rc;
use stdweb::web::{document, HtmlElement};
use stdweb::web::event::ClickEvent;
use stdweb::web::IParentNode;

use dominator::{Dom, signal};
use dominator::signal::Signal;


fn main() {
    stylesheet!("div", {
        style("border", "5px solid black");
    });

    let foobar = class! {
        style("border-right", "10px solid purple");
    };

    /*let media_query = stylesheet!(format!("@media (max-width: 500px) .{}", foobar), {
        style("border-left", "10px solid teal");
    });*/

    let mut count = 0;

    let (sender_elements, receiver_elements) = signal::unsync::mutable(count);


    let mut width: u32 = 10;

    let (sender1, receiver1) = signal::unsync::mutable(width);
    let (sender2, receiver2) = signal::unsync::mutable(vec![width]);
    let (sender3, receiver3) = signal::unsync::mutable(vec![width]);


    trace_macros!(true);

    /*let style_width = receiver1.switch(move |x| {
        receiver2.clone().switch(move |y| {
            receiver3.clone().map(move |z| {
                Some(format!("{}px", x + y[0] + z[0]))
            })
        })
    });*/

    let style_width = map_rc! {
        let x: Rc<u32> = receiver1,
        let y: Rc<Vec<u32>> = receiver2,
        let _z: Rc<Vec<u32>> = receiver3 =>
        Some(format!("{}px", *x + y[0]))
    };

    trace_macros!(false);


    html!("div", {
        style("border", "10px solid blue");
        children([
            html!("div", {
                style("width", style_width);
                style("height", "50px");
                style("background-color", "green");
                event(move |event: ClickEvent| {
                    count += 1;
                    width += 5;

                    console!(log, &event);

                    sender1.set(width).unwrap();
                    sender2.set(vec![width]).unwrap();
                    sender3.set(vec![width]).unwrap();
                    sender_elements.set(count).unwrap();
                });
                children(receiver_elements.map(|count| {
                    (0..count).map(|_| {
                        html!("div", {
                            style("border", "5px solid red");
                            style("width", "50px");
                            style("height", "50px");
                        })
                    })
                }));
            }),

            html!("div", {
                style("width", "50px");
                style("height", "50px");
                style("background-color", "red");
                children([
                    html!("div", {
                        style("width", "10px");
                        style("height", "10px");
                        style("background-color", "orange");
                    })
                ].as_mut());
            }),

            html!("div", {
                style("width", "50px");
                style("height", "50px");
                style("background-color", "red");
                class(&foobar, true);
                children([
                    html!("div", {
                        style("width", "10px");
                        style("height", "10px");
                        style("background-color", "orange");
                    })
                ].as_mut());
            }),

            Dom::with_state(Rc::new(vec![1, 2, 3]), |a| {
                html!("div", {
                    style("width", "100px");
                    style("height", "100px");
                    style("background-color", "orange");
                    class("foo", true);
                    class("bar", false);
                    event(clone!({ a } move |event: ClickEvent| {
                        console!(log, &*a, &event);
                    }));
                })
            }),

            html!("input", {
                focused(true);
            }),
        ].as_mut());
    }).insert_into(
        &document().query_selector("body").unwrap().unwrap()
    );
}
