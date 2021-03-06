
use config::{SCREEN_CFG, UI_CFG};
use super::commonuse::*;
use super::widget::*;
use super::SpecialDialogResult;
use text;

pub struct StartWindow {
    title_screen: ImageWidget,
}

impl StartWindow {
    pub fn new() -> StartWindow {
        let rect = Rect::new(0, 0, SCREEN_CFG.screen_w, SCREEN_CFG.screen_h);
        
        StartWindow {
            title_screen: ImageWidget::ui_img(rect, "!title-screen"),
        }
    }
    
    pub fn draw(&mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
                  _anim: Option<(&Animation, u32)>) {

        self.title_screen.draw(canvas, sv);
    }
}

pub struct StartDialog {
    rect: Rect,
    answer_list: ListWidget,
}

impl StartDialog {
    pub fn new() -> StartDialog {
        let choices = vec![
            text::ui_txt("dialog.choice.newgame").to_owned(),
            text::ui_txt("dialog.choice.loadgame").to_owned(),
            text::ui_txt("dialog.choice.exit").to_owned()];
        let rect = UI_CFG.start_dialog.rect.into();
        StartDialog {
            rect: rect,
            answer_list: ListWidget::texts_choices((0, 0, rect.w as u32, 0), choices),
        }
    }
}

impl Window for StartDialog {
    fn draw(&mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
              _anim: Option<(&Animation, u32)>) {

        draw_rect_border(canvas, self.rect);
        
        self.answer_list.draw(canvas, sv);
    }
}

impl DialogWindow for StartDialog {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.answer_list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(0) => { // New Game
                    return DialogResult::Special(SpecialDialogResult::StartDialogNewGame);
                }
                ListWidgetResponse::Select(1) => { // Load Game
                    return DialogResult::Special(SpecialDialogResult::StartDialogLoadGame);
                }
                ListWidgetResponse::Select(2) => { // Exit
                    return DialogResult::Quit;
                }
                _ => (),
            }
            return DialogResult::Continue;
        }
        
        DialogResult::Continue
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

