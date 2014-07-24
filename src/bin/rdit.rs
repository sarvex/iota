extern crate rustbox;
extern crate rdit;

fn main() {
    rustbox::init();

    rustbox::print(1, 1, rustbox::Bold, rustbox::White, rustbox::Black, "Hello, world!".to_string());
    rustbox::present();

    let(events, receiver) = channel();
    let editor = rdit::Editor {
        events: receiver,
        buffers: Vec::new()
    };

    spawn(proc() {
        loop {
            events.send(rustbox::poll_event());
            //break;
        }
    });

    editor.open_file("/home/gchp/test.txt");
    editor.start();

    rustbox::shutdown();
}
