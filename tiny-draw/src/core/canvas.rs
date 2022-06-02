use super::{color::Color, offset::Offset};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{window, CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement};
use std::rc::Rc;

#[derive(Debug)]
pub enum CanvasError {
  InvalidDocument,
  InvalidElement,
  InvalidStyle,
  InvalidAppend,
  InvalidScale,
  InvalidEvent,
  InvalidWrapperElement,
}
pub type CanvasResult<T> = Result<T, CanvasError>;

/// 画布
pub struct Canvas {
  /// 画布宽度
  width: u32,
  /// 画布高度
  height: u32,
  /// 画布背景颜色
  background_color: Color,
  /// 画布容器
  wrapper: HtmlElement,
  /// 绘制画布
  draw_canvas: Rc<HtmlCanvasElement>,
  /// 操作画布
  option_canvas: HtmlCanvasElement,
  /// 操作画布环境
  option_ctx: CanvasRenderingContext2d,
  /// 绘制画布环境
  draw_ctx: CanvasRenderingContext2d,
  /// 缓存区
  cache_canvas: HtmlCanvasElement,
  cache_ctx: CanvasRenderingContext2d,

  /// 偏移量
  offset: Offset,
}

impl Canvas {
  pub fn new(id: &str, width: u32, height: u32) -> CanvasResult<Self> {
    let document = match window().unwrap().document() {
      Some(doc) => Ok(doc),
      None => return Err(CanvasError::InvalidDocument),
    }?;
    let wrapper = Self::init_wrapper(&document, id, width, height)?;
    let (draw_canvas, draw_ctx) = Self::create_draw_canvas(&document, width, height)?;
    let (option_canvas, option_ctx) = Self::create_option_canvas(&document, width, height)?;
    let (cache_canvas, cache_ctx) = Self::create_cache_canvas(&document, width, height)?;
    let background_color = Color::hex("#fff").unwrap();
    wrapper
      .append_child(&draw_canvas)
      .map_err(|_| CanvasError::InvalidAppend)?;
    wrapper
      .append_child(&option_canvas)
      .map_err(|_| CanvasError::InvalidAppend)?;
    let canvas = Rc::new(draw_canvas);

    let offset = Offset::new(0, 0);

    let mut this = Self {
      width,
      height,
      background_color,
      wrapper,
      draw_canvas: canvas,
      draw_ctx,
      option_canvas,
      option_ctx,
      cache_canvas,
      cache_ctx,
      offset,
    };

    this.update_offset()?;
    this.init_event()?;

    // 处理模糊问题
    this.init_retina_scaling()?;

    Ok(this)
  }

  pub fn get_offset(&self) -> Offset {
    self.offset.clone()
  }
}

// 关联函数
impl Canvas {
  fn init_wrapper(
    document: &Document,
    id: &str,
    width: u32,
    height: u32,
  ) -> CanvasResult<HtmlElement> {
    // 创建画布容器
    let el = document
      .get_element_by_id(id)
      .unwrap()
      .dyn_into::<HtmlElement>()
      .map_err(|_| CanvasError::InvalidElement)?;
    el.style()
      .set_property("position", "relative")
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("width", format!("{}px", width).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("height", format!("{}px", height).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("user-select", "none")
      .map_err(|_| CanvasError::InvalidStyle)?;
    Ok(el)
  }
  
  // 创建绘制画布
  fn create_draw_canvas(
    document: &Document,
    width: u32,
    height: u32,
  ) -> CanvasResult<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let el = document
      .create_element("canvas")
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .map_err(|_| CanvasError::InvalidElement)?;
    Self::apply_canvas_style(&el, width, height)?;
    let ctx = el
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .map_err(|_| CanvasError::InvalidElement)?;

    Ok((el, ctx))
  }

  fn create_option_canvas(
    document: &Document,
    width: u32,
    height: u32,
  ) -> CanvasResult<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let el = document
      .create_element("canvas")
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .map_err(|_| CanvasError::InvalidElement)?;
    Self::apply_canvas_style(&el, width, height)?;
    let ctx = el
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .map_err(|_| CanvasError::InvalidElement)?;

    Ok((el, ctx))
  }

  fn apply_canvas_style(el: &HtmlCanvasElement, width: u32, height: u32) -> CanvasResult<()> {
    el.set_width(width);
    el.set_height(height);
    el.style()
      .set_property("position", "absolute")
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("width", format!("{}px", width).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("height", format!("{}px", height).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("top", "0")
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("left", "0")
      .map_err(|_| CanvasError::InvalidStyle)?;
    el.style()
      .set_property("user-select", "none")
      .map_err(|_| CanvasError::InvalidStyle)
  }

  fn create_cache_canvas(
    document: &Document,
    width: u32,
    height: u32,
  ) -> CanvasResult<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let el = document
      .create_element("canvas")
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .map_err(|_| CanvasError::InvalidElement)?;
    el.set_width(width);
    el.set_height(height);
    let ctx = el
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .map_err(|_| CanvasError::InvalidElement)?;

    Ok((el, ctx))
  }

}

// 私有方法
impl Canvas {
  // 初始化画布缩放
  fn init_retina_scaling(&self) -> CanvasResult<()> {
    let dpr = window().unwrap().device_pixel_ratio();
    self.init_option_retina_scaling(dpr)?;
    self.init_draw_retina_scaling(dpr)?;
    self.init_cache_retina_scaling(dpr)?;

    Ok(())
  }
  fn init_option_retina_scaling(&self, dpr: f64) -> CanvasResult<()> {
    let w = ((self.width as f64) * dpr).round() as u32;
    let h = ((self.height as f64) * dpr).round() as u32;
    self.option_canvas.set_width(w);
    self.option_canvas.set_height(h);
    self
      .option_canvas
      .style()
      .set_property("width", format!("{}px", w).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    self
      .option_canvas
      .style()
      .set_property("height", format!("{}px", h).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    self.option_ctx.scale(dpr, dpr).map_err(|_| CanvasError::InvalidScale)?;
    Ok(())
  }
  fn init_draw_retina_scaling(&self, dpr: f64) -> CanvasResult<()> {
    let w = ((self.width as f64) * dpr).round() as u32;
    let h = ((self.height as f64) * dpr).round() as u32;
    self.draw_canvas.set_width(w);
    self.draw_canvas.set_height(h);
    self
      .draw_canvas
      .style()
      .set_property("width", format!("{}px", w).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    self
      .draw_canvas
      .style()
      .set_property("height", format!("{}px", h).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    self.draw_ctx.scale(dpr, dpr).map_err(|_| CanvasError::InvalidScale)?;
    Ok(())
  }
  fn init_cache_retina_scaling(&self, dpr: f64) -> CanvasResult<()> {
    let w = ((self.width as f64) * dpr).round() as u32;
    let h = ((self.height as f64) * dpr).round() as u32;
    self.cache_canvas.set_width(w);
    self.cache_canvas.set_height(h);
    self
      .cache_canvas
      .style()
      .set_property("width", format!("{}px", w).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    self
      .cache_canvas
      .style()
      .set_property("height", format!("{}px", h).as_str())
      .map_err(|_| CanvasError::InvalidStyle)?;
    self.cache_ctx.scale(dpr, dpr).map_err(|_| CanvasError::InvalidScale)?;
    Ok(())
  }

  fn update_offset(&mut self) -> CanvasResult<()> {
    self.offset = calc_offset(self.draw_canvas.clone())?;
    Ok(())
  }

  fn init_event(&'static mut self) -> CanvasResult<()> {
    let handler = move |_event: web_sys::EventTarget| {
      self.resize_handler();
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window().unwrap().add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).map_err(|_| CanvasError::InvalidEvent)?;
    closure.forget();
    Ok(())
  }

  fn resize_handler(&mut self) {
    self.update_offset().unwrap();
  }
}

fn calc_offset(el: Rc<HtmlCanvasElement>) -> CanvasResult<Offset> {
  let mut top = 0;
  let mut left = 0;
  top += el.offset_top();
  left += el.offset_left();
  let mut el = match el.offset_parent() {
    Some(el) => el.dyn_into::<HtmlElement>().unwrap(),
    _ => return Ok(Offset::new(top, left)),
  };
  loop {
    top += el.offset_top();
    left += el.offset_left();
    el = match el.offset_parent() {
      Some(el) => el.dyn_into::<HtmlElement>().unwrap(),
      _ => break,
    };
  }

  Ok(Offset::new(top, left))
}
