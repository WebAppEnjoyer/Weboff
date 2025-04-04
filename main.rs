#![windows_subsystem = "windows"]
use std::env;
use tao::window::Fullscreen;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    };
use wry::WebViewBuilder;





  fn main() -> wry::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let builder = WebViewBuilder::new().with_url(&format!("file://{}", file_path));
    
      
    #[cfg(any(
      target_os = "windows",
      target_os = "macos",
      target_os = "ios",
      target_os = "android"
    ))]
    let _webview = builder.build(&window)?;

    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
     
    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      if let Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } = event
      {
        *control_flow = ControlFlow::Exit;
      }
    });
  }

