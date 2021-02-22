use web_view::*;

fn Hello(){
    println!("Hello World");
}

fn main() {
    let html_content = 
    "<html>
        <body>
        <h1>Hello, Rust Web View!</h1>
        <p> Just a simple app to see how it works. </p>
        </body>
    </html>";
	
    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
