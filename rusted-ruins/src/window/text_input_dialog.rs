
use config::UI_CFG;
use sdlvalues::FontKind;
use super::commonuse::*;
use super::text_input;
use super::widget::*;

pub struct TextInputDialog {
    label: LabelWidget,
    rect: Rect,
    text: String,
}

impl TextInputDialog {
    pub fn new() -> TextInputDialog {
        text_input::start();

        let rect: Rect = UI_CFG.text_input_dialog.rect.into();
        let label_rect = Rect::new(0, 0, rect.width(), rect.height());
        
        TextInputDialog {
            label: LabelWidget::new(label_rect, "", FontKind::M),
            rect: rect,
            text: String::new(),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// This function is used when the result string is invalid,
    /// and text input is needed again.
    pub fn restart(&self) {
        text_input::start();
    }
}

impl Window for TextInputDialog {
    fn draw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.label.draw(canvas, sv);
    }
}

impl DialogWindow for TextInputDialog {
    fn process_command(&mut self, command: &Command, _pa: &mut DoPlayerAction) -> DialogResult {
        match command {
            &Command::TextInput { ref text } => {
                self.text.push_str(&text);
                self.label.set_text(&self.text);
                DialogResult::Continue
            },
            &Command::TextDelete => {
                self.text.pop();
                self.label.set_text(&self.text);
                DialogResult::Continue
            },
            &Command::Enter => {
                text_input::end();
                DialogResult::Close
            },
            &Command::Cancel => {
                text_input::end();
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::TextInput
    }
}

