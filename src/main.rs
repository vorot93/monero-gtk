use derive_more::*;
use gio::prelude::*;
use gtk::prelude::*;

pub trait Widget<O> {
    fn id() -> &'static str;
    fn inner(self) -> O;
}

macro_rules! widget {
    ($name:ident: $inner:ty) => {
        #[derive(Clone, Debug, From)]
        pub struct $name(pub $inner);

        impl Widget<$inner> for $name {
            fn id() -> &'static str {
                stringify!($name)
            }

            fn inner(self) -> $inner {
                self.0
            }
        }
    };
}

widget!(MainWindow: gtk::ApplicationWindow);

widget!(SendArea: gtk::Popover);
widget!(ReceiveArea: gtk::Popover);

widget!(SendButton: gtk::Button);
widget!(ReceiveButton: gtk::Button);

pub trait GetObject {
    fn make_object<T, O>(&self) -> T
    where
        T: Widget<O> + std::convert::From<O>,
        O: glib::IsA<glib::Object>;
}

impl GetObject for gtk::Builder {
    fn make_object<T, O>(&self) -> T
    where
        T: Widget<O> + std::convert::From<O>,
        O: glib::IsA<glib::Object>,
    {
        T::from(self.get_object::<O>(T::id()).unwrap())
    }
}

fn main() {
    env_logger::init();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let application = gtk::Application::new(
        Some("com.github.vorot93.monero"),
        gio::ApplicationFlags::empty(),
    )
    .unwrap();

    let builder = gtk::Builder::new_from_string(include_str!("app.ui"));

    application.connect_startup(move |app| {
        for (button, popover) in vec![
            (
                builder.make_object::<SendButton, _>().0,
                builder.make_object::<SendArea, _>().0,
            ),
            (
                builder.make_object::<ReceiveButton, _>().0,
                builder.make_object::<ReceiveArea, _>().0,
            ),
        ] {
            button.connect_clicked(move |_| popover.popup());
        }

        let window = builder.make_object::<MainWindow, _>().0;
        window.connect_delete_event(|_, _| Inhibit(false));

        window.show_all();

        app.add_window(&window);
    });
    application.connect_activate(|_| {});

    application.run(&std::env::args().collect::<Vec<_>>());
}
