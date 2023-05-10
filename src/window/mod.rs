use webview_app::{app::App, app::AppSettings};
 
pub fn create_instance(window_title: &str, window_navigation_url: &str, window_width: i32, window_height: i32)
{
    let app = App::new(
        AppSettings { 
            title : window_title.to_string(),
            url   : window_navigation_url.to_string(), 
            width : window_width,
            height: window_height,
            ..Default::default()
        }
    );
    app.run();
}