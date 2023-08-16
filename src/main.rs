use crate::reader::Pages;
use hyprland_wiki::*;

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(u16::MAX);
    area
}
fn clear_and_redraw<W: Write>(w: &mut W, view: &mut MadView, _skin: &MadSkin) -> Result<(), Error> {
    queue!(w, Clear(ClearType::All))?;
    view.write_on(w)?;
    w.flush()?;
    Ok(())
}

fn run_app(skin: MadSkin) -> Result<(), Error> {
    let mut pages = Pages::construct();
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?;

    let mut view = MadView::from(pages.current_page_content(), view_area(), skin.clone());

    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                Left => {
                    clear_and_redraw(&mut w, &mut view, &skin)?;
                    if pages.current_page > 0 {
                        pages.current_page -= 1;
                        view =
                            MadView::from(pages.current_page_content(), view_area(), skin.clone());
                    }
                    clear_and_redraw(&mut w, &mut view, &skin)?;
                }
                Right => {
                    clear_and_redraw(&mut w, &mut view, &skin)?;
                    if pages.current_page < pages.pages.len() - 1 {
                        pages.current_page += 1;
                        view =
                            MadView::from(pages.current_page_content(), view_area(), skin.clone());
                    }
                    clear_and_redraw(&mut w, &mut view, &skin)?;
                }
                Char('R') => {
                    clear_and_redraw(&mut w, &mut view, &skin)?;
                }
                Char(':') => {
                    if let Ok(Event::Key(KeyEvent {
                        code: Char('q'), ..
                    })) = event::read()
                    {
                        break;
                    }
                }
                _ => {}
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    queue!(w, Show)?;
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.paragraph.align = Alignment::Center;
    skin.set_bg(rgb(36, 36, 36));
    skin.set_headers_fg(rgb(127, 255, 212));
    skin.bold.set_fg(White);
    skin.italic.set_fg(rgb(205, 210, 215));
    skin.scrollbar.thumb.set_fg(rgb(127, 255, 212));
    skin.code_block.align = Alignment::Center;
    skin
}

fn main() -> Result<(), Error> {
    let skin = make_skin();
    run_app(skin)
}
