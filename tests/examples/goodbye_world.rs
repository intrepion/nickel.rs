use util::{run_example, read_url};

fn t(file: &str) {
    run_example(file, |port| {
        let paths = ["", "foo", "bar.html", "foo-barrrr/baz"];

        for path in &paths {
            let url = format!("http://localhost:{}/{}", port, path);
            let s = read_url(&url);

            assert_eq!(s, "Goodbye World");
        }
    })
}

#[test] fn non_macro() { t("goodbye_world") }
#[test] fn _macro() { t("goodbye_world_macro") }
